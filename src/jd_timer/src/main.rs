#![no_std]
#![no_main]
#![allow(unused_imports)]

use panic_halt as _;

#[rtic::app(device = stm32f1xx_hal::pac,
peripherals = true, dispatchers = [DMA1_CHANNEL1,DMA1_CHANNEL2,DMA1_CHANNEL3])]
// RTIC application
mod app {

    use stm32f1xx_hal::{
        prelude::*,
        serial,
        gpio::{
            gpiob::{PB8, PB9, PB6, PB5},
            gpioa::{PA0, PA1, PA4, PA9, PA10},
            {Output, PushPull},
            {Input, PullUp},
            {Alternate, OpenDrain},
        },
        timer::{Event, Timer},
        pac::{I2C1, USART1},
        i2c::{BlockingI2c, DutyCycle, Mode},
    };

    use cortex_m::asm::delay;

    //use core::fmt::Write;
    use ssd1306::{
        prelude::*,
        Builder,
        I2CDIBuilder,
    };

    // Import peripheral control methods from general HAL definition
    use embedded_hal::digital::v2::{OutputPin, InputPin};
    use core::ptr::write_volatile;

    use core::fmt::Write;
    use core::future::Future;

    // Resources shared by all handlers
    #[resources]
    struct Resources {
        display: GraphicsMode<I2CInterface<BlockingI2c<I2C1, (PB8<Alternate<OpenDrain>>,PB9<Alternate<OpenDrain>>) >>, DisplaySize128x64>,
        buttons: (PB5<Input<PullUp>>, PB6<Input<PullUp>>),
        EXTI: stm32f1xx_hal::pac::EXTI,
        clocks: stm32f1xx_hal::rcc::Clocks,
        serial: (serial::Tx<USART1>, serial::Rx<USART1>),
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

        // Configure button inputs
        let button_start = gpiob.pb5.into_pull_up_input(&mut gpiob.crl);
        let button_brightness = gpiob.pb6.into_pull_up_input(&mut gpiob.crl);

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

        // Schedule the display to be updated with initial value
        //update_display(&mut display, "Hello");

        (init::LateResources {
            display,
            buttons: (button_start, button_brightness),
            EXTI: cx.device.EXTI,
            clocks,
            serial: (tx, rx),
        }, init::Monotonics())
    }
}
