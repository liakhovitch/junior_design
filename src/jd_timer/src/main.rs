#![no_std]
#![no_main]
#![allow(unused_imports)]

use panic_halt as _;
mod buttons;
mod pot;

#[rtic::app(device = stm32f1xx_hal::pac,
peripherals = true, dispatchers = [DMA1_CHANNEL1,DMA1_CHANNEL2,DMA1_CHANNEL3])]
// RTIC application
mod app {

    use crate::buttons::*;
    use crate::pot::*;

    use stm32f1xx_hal::{
        adc::{Adc, SampleTime},
        prelude::*,
        serial,
        gpio::{
            gpiob::{PB8, PB9, PB6, PB5},
            gpioa::{PA0, PA1, PA4, PA9, PA10},
            {Output, PushPull},
            {Input, PullUp},
            {Alternate, OpenDrain},
            Edge::*,
            ExtiPin,
        },
        timer::{Event, Timer},
        pac::{I2C1, USART1, ADC1},
        i2c::{BlockingI2c, DutyCycle, Mode},
    };

    use cortex_m::asm::delay;

    //use core::fmt::Write;
    use ssd1306::{
        prelude::*,
        Builder,
        I2CDIBuilder,
    };
    use embedded_graphics::{
        fonts::Text,
        pixelcolor::BinaryColor,
        prelude::*,
        style::TextStyle,
    };
    use profont::ProFont24Point;

    // Import peripheral control methods from general HAL definition
    use embedded_hal::digital::v2::{OutputPin, InputPin};
    use core::ptr::write_volatile;

    use core::fmt::Write;
    use core::future::Future;
    use stm32f1xx_hal::gpio::Analog;

    // Resources shared by all handlers
    #[resources]
    struct Resources {
        display: GraphicsMode<I2CInterface<BlockingI2c<I2C1, (PB8<Alternate<OpenDrain>>,PB9<Alternate<OpenDrain>>) >>, DisplaySize128x64>,
        button_start: PB5<Input<PullUp>>,
        button_brightness: PB6<Input<PullUp>>,
        EXTI: stm32f1xx_hal::pac::EXTI,
        clocks: stm32f1xx_hal::rcc::Clocks,
        serial: (serial::Tx<USART1>, serial::Rx<USART1>),
        adc1: Adc<ADC1>,
        pot: PA4<Analog>,
        pot_pos: u16,
        #[init(0)]
        brightness_state: u8,
        #[init(false)]
        pot_dir: bool,
    }

    // Init function (duh)
    #[init ()]
    // CX object contains our PAC. LateResources
    fn init(cx: init::Context) -> (init::LateResources,init::Monotonics){

        // Enable cycle counter
        let mut core = cx.core;
        core.DWT.enable_cycle_counter();
        // Take ownership of clock register
        let mut rcc = cx.device.RCC.constrain();
        // Take ownership of flash peripheral
        let mut flash = cx.device.FLASH.constrain();
        // Take ownership of AFIO register
        let mut afio = cx.device.AFIO.constrain(&mut rcc.apb2);

        // Configure clocks and make clock object from clock register
        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        // Split GPIO ports into smaller pin objects
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.apb2);
        let mut gpiob = cx.device.GPIOB.split(&mut rcc.apb2);

        // -----------
        // Init buttons
        // -----------
        // Start timer button
        let mut button_start = gpiob.pb5.into_pull_up_input(&mut gpiob.crl);
        button_start.make_interrupt_source(&mut afio);
        button_start.trigger_on_edge(&cx.device.EXTI, FALLING);
        button_start.enable_interrupt(&cx.device.EXTI);
        // Brightness button
        let mut button_brightness = gpiob.pb6.into_pull_up_input(&mut gpiob.crl);
        button_brightness.make_interrupt_source(&mut afio);
        button_brightness.trigger_on_edge(&cx.device.EXTI, FALLING);
        button_brightness.enable_interrupt(&cx.device.EXTI);
        // Note: Both buttons trigger the EXTI9_5 interrupt.
        // Small rant:
        // This feature is undocumented in the MOTHERTRUCKING hal documentation so I did all this
        // using unsafe direct memory writes in a previous project. I got lucky here and stumbled
        // on this in the library source code.

        // ------------
        // Init pot adc
        // ------------
        // Setup adc1 with default settings
        let mut adc1 = Adc::adc1(cx.device.ADC1, &mut rcc.apb2, clocks);
        adc1.set_sample_time(SampleTime::T_239);
        // Setup PA4 as analog input
        let mut pot = gpioa.pa4.into_analog(&mut gpioa.crl);
        let mut pot_pos = adc1.read(&mut pot).unwrap();
        pot_pos = pot_pos >> 4;

        // -----------
        // Init serial
        // -----------
        let tx1_pin = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
        let rx1_pin = gpioa.pa10.into_floating_input(&mut gpioa.crh);
        let cfg = serial::Config::default().baudrate(115_200.bps());
        let usart1 = serial::Serial::usart1(
            cx.device.USART1,
            (tx1_pin, rx1_pin),
            &mut afio.mapr,
            cfg,
            clocks,
            &mut rcc.apb2,
        );
        let (tx, rx) = usart1.split();

        // ----------------
        // Init I2C display
        // ----------------
        // Init IO pins
        let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
        let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

        // Init i2c peripheral
        let i2c = BlockingI2c::i2c1(
            cx.device.I2C1,
            (scl, sda),
            &mut afio.mapr,
            Mode::Fast {
                frequency: 400_000.hz(),
                duty_cycle: DutyCycle::Ratio2to1,
            },
            clocks,
            &mut rcc.apb1,
            1000,
            10,
            1000,
            1000,
        );

        // Init i2c interface
        let interface = I2CDIBuilder::new().init(i2c);
        // Create display in graphics mode
        let mut display: GraphicsMode<_, _> = Builder::new().connect(interface).into();
        // Init display
        display.init().unwrap();
        display.clear();

        // Show boot message
        Text::new("Hello", Point::new(20,16))
            .into_styled(TextStyle::new(ProFont24Point, BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        // Schedule the display to be updated with initial value
        //update_display(&mut display, "Hello");

        (init::LateResources {
            display,
            button_start, button_brightness,
            EXTI: cx.device.EXTI,
            clocks,
            serial: (tx, rx),
            adc1,
            pot,
            pot_pos,
        }, init::Monotonics())
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            // Read off the ADC value
            handle_adc::spawn().unwrap();
        }
    }

    extern "Rust" {
        #[task(binds = EXTI9_5, resources = [&clocks, button_start, button_brightness, EXTI, display, brightness_state], priority=1)]
        fn handle_buttons(cx: handle_buttons::Context);
        #[task(resources = [pot, display, pot_pos, adc1, pot_dir], priority=1)]
        fn handle_adc(cx: handle_adc::Context);
    }
}
