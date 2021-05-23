#![no_std]
#![no_main]
#![allow(unused_imports)]

use panic_halt as _;
mod buttons;
mod pot;
mod ui;
mod rtc;
mod beep;
mod types;
mod config;

#[rtic::app(device = stm32f1xx_hal::pac,
peripherals = true, dispatchers = [DMA1_CHANNEL1,DMA1_CHANNEL2,DMA1_CHANNEL3])]
// RTIC application
mod app {

    use crate::buttons::*;
    use crate::pot::*;
    use crate::ui::*;
    use crate::types::*;

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

    use dwt_systick_monotonic::DwtSystick;

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

    // Declare type for monotonic timer used by RTIC for task scheduling
    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<8_000_000>; // 8 MHz

    // Resources shared by all handlers.
    // All resourced not initialized here by macros are initialized in [init] and returned to RTIC
    //   in the init::LateResources object.
    #[resources]
    struct Resources {
        display: GraphicsMode<I2CInterface<BlockingI2c<I2C1, (PB8<Alternate<OpenDrain>>,PB9<Alternate<OpenDrain>>) >>, DisplaySize128x64>,
        button_start: PB5<Input<PullUp>>,
        button_brightness: PB6<Input<PullUp>>,
        EXTI: stm32f1xx_hal::pac::EXTI,
        clocks: stm32f1xx_hal::rcc::Clocks,
        //serial: (serial::Tx<USART1>, serial::Rx<USART1>),
        adc1: Adc<ADC1>,
        pot: PA4<Analog>,
        pot_pos: u16,
        #[init(0)]
        brightness_state: u8,
        #[init(false)]
        pot_dir: bool,
        #[init(SysState::Setup)]
        sys_state: SysState,
        #[init(0)]
        time_remaining: u16,
    }

    // Init function (duh)
    #[init ()]
    // CX object contains our PAC.
    // Init function initializes resources and returns them to RTIC via the LateResources object.
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
            //.use_hse(8.mhz())
            //.sysclk(8.mhz())
            //.pclk1(4.mhz())
            .freeze(&mut flash.acr);

        // Split GPIO ports into smaller pin objects
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.apb2);
        let mut gpiob = cx.device.GPIOB.split(&mut rcc.apb2);

        // ---------------------
        // Init scheduling timer
        // ---------------------
        // This monotonic timer object is returned to the RTIC framework for use in task scheduling
        let mono = DwtSystick::new(&mut core.DCB, core.DWT, core.SYST, 8_000_000);

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

        // Serial disabled because TX pin is being hacked into use for PSU control.
        // Serial was only there for debug purposes anyway. We have SWD for debugging.
        // -----------
        // Init serial
        // -----------
        //let tx1_pin = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
        // let rx1_pin = gpioa.pa10.into_floating_input(&mut gpioa.crh);
        // let cfg = serial::Config::default().baudrate(115_200.bps());
        // let usart1 = serial::Serial::usart1(
        //     cx.device.USART1,
        //     (tx1_pin, rx1_pin),
        //     &mut afio.mapr,
        //     cfg,
        //     clocks,
        //     &mut rcc.apb2,
        // );
        // let (tx, rx) = usart1.split();
        let mut sleep_pin = gpioa.pa9.into_push_pull_output(&mut gpioa.crh);
        // Assert buck converter enable pin to stop PSU from shutting off.
        // TODO: implement auto-sleep to shut off after an idle period
        // Tell the PMIC to please not shut us off
        sleep_pin.set_high().unwrap();

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
        // Create display in graphics mode (as opposed to terminal mode)
        let mut display: GraphicsMode<_, _> = Builder::new().connect(interface).into();
        // Init display
        display.init().unwrap();
        display.clear();

        // Read initial ADC value
        let _ = handle_adc::spawn(true);
        // Show boot message
        let _ = update_display::spawn(ScreenPage::Boot);

        // Schedule the display to be updated with initial value
        //update_display(&mut display, "Hello");

        // Return initialized resources to RTIC so they can be loaned to tasks
        (init::LateResources {
            display,
            button_start, button_brightness,
            EXTI: cx.device.EXTI,
            clocks,
            //serial: (tx, rx),
            adc1,
            pot,
            pot_pos,
        },
         // Return timer object so RTIC can use it for task scheduling.
         init::Monotonics(mono))
    }

    // Idle task, run when nothing else is happening. This is where polling happens.
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            // Read off the ADC value
            let _ = handle_adc::spawn(false);
        }
    }

    // This is where we declare tasks which are in external files.
    extern "Rust" {
        #[task(binds = EXTI9_5, resources = [&clocks, button_start, button_brightness, EXTI, display, brightness_state], priority=1)]
        fn handle_buttons(cx: handle_buttons::Context);
        #[task(resources = [pot, pot_pos, adc1, pot_dir, time_remaining], priority=1)]
        fn handle_adc(cx: handle_adc::Context, silent:bool);
        #[task(resources = [display, sys_state, time_remaining, brightness_state], priority=1, capacity=3)]
        fn update_display(cx: update_display::Context, screen_type:ScreenPage);
    }
}
