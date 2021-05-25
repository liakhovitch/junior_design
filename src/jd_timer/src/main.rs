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
mod logo;
mod states;
mod rtc_util;

#[rtic::app(device = stm32f1xx_hal::pac,
peripherals = true, dispatchers = [DMA1_CHANNEL1,DMA1_CHANNEL2,DMA1_CHANNEL3])]
// RTIC application
mod app {

    use crate::buttons::*;
    use crate::pot::*;
    use crate::ui::*;
    use crate::types::*;
    use crate::beep::*;
    use crate::rtc::*;
    use crate::states::*;
    use crate::rtc_util;

    use crate::config::{SLEEP_TIME};

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
            Analog,
        },
        timer::{Event, Timer, Tim2NoRemap},
        pac::{I2C1, USART1, ADC1, TIM2},
        i2c::{BlockingI2c, DutyCycle, Mode},
        pwm::C1,
        rtc::Rtc,
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

    use core::fmt::Write;
    use core::future::Future;
    use core::ptr::*;

    // Declare type for monotonic timer used by RTIC for task scheduling
    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<8_000_000>; // 8 MHz

 /*   struct RtcRegs {

}*/

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
        sleep_pin: PA9<Output<PushPull>>,
        buzzer: stm32f1xx_hal::pwm::PwmChannel<TIM2, C1>,
        // TODO: Replace This
        //rtc: Rtc,
        #[init(0)]
        brightness_state: u8,
        #[init(false)]
        pot_dir: bool,
        #[init(SysState::Setup)]
        sys_state: SysState,
        #[init(0)]
        max_time: u16,
        #[init(0)]
        time_remaining: u16,
        #[init(0)]
        disp_call_cnt: u8,
    }

    // Init function (duh)
    #[init ()]
    // CX object contains our PAC.
    // Init function initializes resources and returns them to RTIC via the LateResources object.
    fn init(cx: init::Context) -> (init::LateResources,init::Monotonics){

        // Enable writing to backup domain and power registers so we can later configure RTC.
        // This must be done before the HAL initializes because the HAL takes ownership of the
        //   APB1ENR register and only allows access from within the library. NOTE: this code
        //   is extremely unstable and may stop working if a later version of the HAL starts
        //   overwriting these bits in this register.

        // Enable cycle counter
        let mut core = cx.core;
        core.DWT.enable_cycle_counter();
        // Take ownership of clock register
        let mut rcc = cx.device.RCC.constrain();
        // Take ownership of flash peripheral
        let mut flash = cx.device.FLASH.constrain();
        // Take ownership of AFIO register
        let mut afio = cx.device.AFIO.constrain(&mut rcc.apb2);

        //let mut backup_domain = rcc.bkp.constrain(bkp, &mut rcc.apb1, &mut pwr);
        /*
        cx.device.RCC.apb2enr.write(|w| w.afioen().enabled());
        cx.device.EXTI.imr.modify(|_,w| w.mr6().set_bit());
        cx.device.EXTI.imr.modify(|_,w| w.mr7().set_bit());
        */

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

        // --------
        // Init RTC
        // --------
        let mut pwr = cx.device.PWR;
        let mut bkp = cx.device.BKP;
        let mut rtc = cx.device.RTC;

        let mut lsi_hertz:u32 = 30_000;

        // We start by modifying some registers that the HAL already has control of,
        // behind the HAL's back, using unsafe code. We can do this with safe code before
        // initializing the HAL, but we have no guarantee that HAL initialization won't
        // overwrite our changes. For reference, this is what that might have looked like:
        //cx.device.RCC.apb1enr.modify(|_,w| w.pwren().set_bit().bkpen().set_bit());
        //cx.device.RCC.csr.modify(|_,w| w.lsion().set_bit());
        unsafe {
            // Get address of PWR register block
            // Note: PWR access can actually be done safely, but we need the volatile write to
            // ensure that the compiler doesn't put this after our attempts to write to RCC_BDCR.
            let pwr_ptr: *mut u32 = stm32f1xx_hal::pac::PWR::ptr() as *mut u32;

            // Get address of RCC register block
            let rcc_ptr: *mut u32 = stm32f1xx_hal::pac::RCC::ptr() as *mut u32;

            // Offset to get apb1enr address
            // Offset is: 0x1C bytes (from datasheet) / 4 = 7 words
            let apb1enr_ptr: *mut u32 = rcc_ptr.offset(7);
            // Get current reg value
            let mut apb1enr_temp: u32 = read_volatile(apb1enr_ptr);
            // Set BKPEN: Enable backup domain access
            apb1enr_temp |= 1<<27;
            // Set PWREN: Enable backup domain access even harder
            apb1enr_temp |= 1<<28;
            // Write changes
            write_volatile(apb1enr_ptr, apb1enr_temp);

            // Offset to get RCC_CSR address
            // Offset is: 0x24 bytes (from datasheet) / 4 = 9 words
            let rcc_csr_ptr: *mut u32 = rcc_ptr.offset(9);
            // Get current reg value
            let mut rcc_csr_temp: u32 = read_volatile(rcc_csr_ptr);
            // Set LSION: Turn on low speed internal oscillator
            rcc_csr_temp |= 1<<0;
            // Write changes
            write_volatile(rcc_csr_ptr, rcc_csr_temp);

            // Offset to get pwr_cr address
            // Offset is: 0x00 bytes (from datasheet) / 4 = 0 words
            let pwr_cr_ptr: *mut u32 = pwr_ptr.offset(0);
            // Get current reg value
            let mut pwr_cr_temp: u32 = read_volatile(pwr_cr_ptr);
            // Set DBP: Disable backup domain write protection
            pwr_cr_temp |= 1<<8;
            // Write changes
            write_volatile(pwr_cr_ptr, pwr_cr_temp);

            // Offset to get RCC_BDCR address
            // Offset is: 0x20 bytes (from datasheet) / 4 = 8 words
            let rcc_bdcr_ptr: *mut u32 = rcc_ptr.offset(8);
            // Get current reg value
            let mut rcc_bdcr_temp: u32 = read_volatile(rcc_bdcr_ptr);
            // Set RTCEN: Enable RTC
            rcc_bdcr_temp |= 1<<15;
            // Set RTCSEL: Set RTC clock source to LSI (Low Speed Internal) clock
            rcc_bdcr_temp |= 0b10<<8;
            // Write changes
            write_volatile(rcc_bdcr_ptr, rcc_bdcr_temp);
        }

        // Set RTC prescaler. Code partly borrowed from the HAL.
        let prescaler = (lsi_hertz / 1) - 1;
        rtc_util::rtc_write(&mut rtc, |rtc| {
            rtc.prlh.write(|w| unsafe { w.bits(prescaler >> 16) });
            rtc.prll.write(|w| unsafe { w.bits(prescaler as u16 as u32) });
        });

        rtc_util::set_time(&mut rtc, 0);
        rtc_util::set_alarm(&mut rtc, SLEEP_TIME as u32);
        rtc_util::listen_alarm(&mut rtc);
        rtc_util::unlisten_seconds(&mut rtc);
        rtc_util::clear_alarm_flag(&mut rtc);
        rtc_util::clear_second_flag(&mut rtc);

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

        // -----------------
        // Init PMIC control
        // -----------------
        let mut sleep_pin = gpioa.pa9.into_push_pull_output(&mut gpioa.crh);
        // Assert buck converter enable pin to stop PSU from shutting off.
        // TODO: implement auto-sleep to shut off after an idle period
        // Tell the PMIC to please not shut us off
        sleep_pin.set_high().unwrap();

        // ------------
        // Init buzzer
        // ------------
        // For now, only using PWM on one pin
        let buzz0 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
        let mut buzz1 = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
        buzz1.set_high().unwrap();
        let mut buzzer = Timer::tim2(cx.device.TIM2, &clocks, &mut rcc.apb1)
            .pwm::<Tim2NoRemap, _, _, _>(buzz0, &mut afio.mapr, 500.hz()).split();
        buzzer.set_duty(buzzer.get_max_duty() / 2);

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
        // Do startup beep
        //let _ = beep::spawn(70, 2);
        // Show boot message
        let _ = update_display::spawn(ScreenPage::Boot);

        // Return initialized resources to RTIC so they can be loaned to tasks
        (init::LateResources {
            display,
            button_start, button_brightness,
            EXTI: cx.device.EXTI,
            clocks,
            adc1,
            pot,
            pot_pos,
            sleep_pin,
            buzzer,
            //rtc,
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
        #[task(binds = EXTI9_5, resources = [&clocks, button_start, button_brightness, EXTI, display, brightness_state, sys_state], priority=1)]
        fn handle_buttons(cx: handle_buttons::Context);
        #[task(resources = [pot, pot_pos, adc1, pot_dir, max_time], priority=1)]
        fn handle_adc(cx: handle_adc::Context, silent:bool);
        // TODO: Add RTC permission
        #[task(resources = [display, max_time, brightness_state, disp_call_cnt], priority=1, capacity=2)]
        fn update_display(cx: update_display::Context, screen_type:ScreenPage);
        #[task(resources = [disp_call_cnt, sys_state], priority=1, capacity=10)]
        fn reset_display(cx: reset_display::Context);
        #[task(resources = [buzzer], priority=1, capacity=1)]
        fn beep(cx: beep::Context, length: u32, count: u8);
        #[task(resources = [buzzer], priority=1, capacity=1)]
        fn unbeep(cx: unbeep::Context, length: u32, count: u8);
        // TODO: Add RTC permission
        #[task(binds = RTC, resources = [sys_state, max_time, disp_call_cnt], priority=2)]
        fn tick(cx: tick::Context);
        // TODO: Add RTC permission
        #[task(binds = RTCALARM, resources = [sys_state, max_time], priority=2)]
        fn alarm(cx: alarm::Context);
        // TODO: Add RTC permission
        #[task(resources = [sys_state], priority=3, capacity=1)]
        fn kick_dog(cx: kick_dog::Context);
        // TODO: Add RTC permission
        #[task(resources = [sys_state, sleep_pin, max_time, disp_call_cnt], priority=3, capacity=1)]
        fn to_state(cx: to_state::Context, target: SysState);
    }
}
