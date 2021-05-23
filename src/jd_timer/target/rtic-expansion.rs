#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
      r" Always include the device crate which contains the vector table"] use
    stm32f1xx_hal :: pac as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ; pub use
    rtic :: Monotonic as _ ;
    #[doc = r" Holds static methods for each monotonic."] pub mod monotonics
    {
        #[doc =
          "This module holds the static implementation for `MyMono::now()`"]
        #[allow(non_snake_case)] pub mod MyMono
        {
            #[allow(unused_imports)] use crate :: buttons :: * ;
            #[allow(unused_imports)] use crate :: pot :: * ;
            #[allow(unused_imports)] use crate :: ui :: * ;
            #[allow(unused_imports)] use crate :: types :: * ;
            #[allow(unused_imports)] use stm32f1xx_hal ::
            {
                adc :: { Adc, SampleTime }, prelude :: *, serial, gpio ::
                {
                    gpiob :: { PB8, PB9, PB6, PB5 }, gpioa ::
                    { PA0, PA1, PA4, PA9, PA10 }, { Output, PushPull },
                    { Input, PullUp }, { Alternate, OpenDrain }, Edge :: *,
                    ExtiPin,
                }, timer :: { Event, Timer }, pac :: { I2C1, USART1, ADC1 },
                i2c :: { BlockingI2c, DutyCycle, Mode },
            } ; #[allow(unused_imports)] use dwt_systick_monotonic ::
            DwtSystick ; #[allow(unused_imports)] use cortex_m :: asm :: delay
            ; #[allow(unused_imports)] use ssd1306 ::
            { prelude :: *, Builder, I2CDIBuilder, } ;
            #[allow(unused_imports)] use embedded_graphics ::
            {
                fonts :: Text, pixelcolor :: BinaryColor, prelude :: *, style
                :: TextStyle,
            } ; #[allow(unused_imports)] use profont :: ProFont24Point ;
            #[allow(unused_imports)] use embedded_hal :: digital :: v2 ::
            { OutputPin, InputPin } ; #[allow(unused_imports)] use core :: ptr
            :: write_volatile ; #[allow(unused_imports)] use core :: fmt ::
            Write ; #[allow(unused_imports)] use core :: future :: Future ;
            #[allow(unused_imports)] use stm32f1xx_hal :: gpio :: Analog ;
            #[doc = r" Read the current time from this monotonic"] pub fn
            now() -> rtic :: time :: Instant < DwtSystick < 8_000_000 > >
            {
                rtic :: export :: interrupt ::
                free(| _ |
                     {
                         use rtic :: Monotonic as _ ; use rtic :: time ::
                         Clock as _ ; if let Some(m) = unsafe
                         {
                             crate :: app ::
                             __rtic_internal_MONOTONIC_STORAGE_MyMono .
                             as_ref()
                         }
                         {
                             if let Ok(v) = m . try_now() { v } else
                             {
                                 unreachable !
                                 ("Your monotonic is not infallible!")
                             }
                         } else
                         {
                             panic !
                             ("Use of monotonic \'MyMono\' before it was passed to the runtime")
                             ;
                         }
                     })
            }
        }
    } use crate :: buttons :: * ; use crate :: pot :: * ; use crate :: ui :: *
    ; use crate :: types :: * ; use stm32f1xx_hal ::
    {
        adc :: { Adc, SampleTime }, prelude :: *, serial, gpio ::
        {
            gpiob :: { PB8, PB9, PB6, PB5 }, gpioa ::
            { PA0, PA1, PA4, PA9, PA10 }, { Output, PushPull },
            { Input, PullUp }, { Alternate, OpenDrain }, Edge :: *, ExtiPin,
        }, timer :: { Event, Timer }, pac :: { I2C1, USART1, ADC1 }, i2c ::
        { BlockingI2c, DutyCycle, Mode },
    } ; use dwt_systick_monotonic :: DwtSystick ; use cortex_m :: asm :: delay
    ; use ssd1306 :: { prelude :: *, Builder, I2CDIBuilder, } ; use
    embedded_graphics ::
    {
        fonts :: Text, pixelcolor :: BinaryColor, prelude :: *, style ::
        TextStyle,
    } ; use profont :: ProFont24Point ; use embedded_hal :: digital :: v2 ::
    { OutputPin, InputPin } ; use core :: ptr :: write_volatile ; use core ::
    fmt :: Write ; use core :: future :: Future ; use stm32f1xx_hal :: gpio ::
    Analog ; #[doc = r" User code from within the module"] type MyMono =
    DwtSystick < 8_000_000 > ; #[doc = r" User code end"]
    #[allow(non_snake_case)] fn init(cx : init :: Context) ->
    (init :: LateResources, init :: Monotonics)
    {
        let mut core = cx . core ; core . DWT . enable_cycle_counter() ; let
        mut rcc = cx . device . RCC . constrain() ; let mut flash = cx .
        device . FLASH . constrain() ; let mut afio = cx . device . AFIO .
        constrain(& mut rcc . apb2) ; let clocks = rcc . cfgr .
        freeze(& mut flash . acr) ; let mut gpioa = cx . device . GPIOA .
        split(& mut rcc . apb2) ; let mut gpiob = cx . device . GPIOB .
        split(& mut rcc . apb2) ; let mono = DwtSystick ::
        new(& mut core . DCB, core . DWT, core . SYST, 8_000_000) ; let mut
        button_start = gpiob . pb5 . into_pull_up_input(& mut gpiob . crl) ;
        button_start . make_interrupt_source(& mut afio) ; button_start .
        trigger_on_edge(& cx . device . EXTI, FALLING) ; button_start .
        enable_interrupt(& cx . device . EXTI) ; let mut button_brightness =
        gpiob . pb6 . into_pull_up_input(& mut gpiob . crl) ;
        button_brightness . make_interrupt_source(& mut afio) ;
        button_brightness . trigger_on_edge(& cx . device . EXTI, FALLING) ;
        button_brightness . enable_interrupt(& cx . device . EXTI) ; let mut
        adc1 = Adc :: adc1(cx . device . ADC1, & mut rcc . apb2, clocks) ;
        adc1 . set_sample_time(SampleTime :: T_239) ; let mut pot = gpioa .
        pa4 . into_analog(& mut gpioa . crl) ; let mut pot_pos = adc1 .
        read(& mut pot) . unwrap() ; pot_pos = pot_pos >> 4 ; let mut
        sleep_pin = gpioa . pa9 . into_push_pull_output(& mut gpioa . crh) ;
        sleep_pin . set_high() . unwrap() ; let scl = gpiob . pb8 .
        into_alternate_open_drain(& mut gpiob . crh) ; let sda = gpiob . pb9 .
        into_alternate_open_drain(& mut gpiob . crh) ; let i2c = BlockingI2c
        ::
        i2c1(cx . device . I2C1, (scl, sda), & mut afio . mapr, Mode :: Fast
             {
                 frequency : 400_000 . hz(), duty_cycle : DutyCycle ::
                 Ratio2to1,
             }, clocks, & mut rcc . apb1, 1000, 10, 1000, 1000,) ; let
        interface = I2CDIBuilder :: new() . init(i2c) ; let mut display :
        GraphicsMode < _, _ > = Builder :: new() . connect(interface) . into()
        ; display . init() . unwrap() ; display . clear() ; let _ = handle_adc
        :: spawn(true) ; let _ = update_display :: spawn(ScreenPage :: Boot) ;
        (init :: LateResources
         {
             display, button_start, button_brightness, EXTI : cx . device .
             EXTI, clocks, adc1, pot, pot_pos,
         }, init :: Monotonics(mono))
    } #[allow(non_snake_case)] fn idle(_cx : idle :: Context) -> !
    {
        use rtic :: Mutex as _ ; use rtic :: mutex_prelude :: * ; loop
        { let _ = handle_adc :: spawn(false) ; }
    } #[allow(non_snake_case)] #[doc = "Initialization function"] pub mod init
    {
        #[allow(unused_imports)] use crate :: buttons :: * ;
        #[allow(unused_imports)] use crate :: pot :: * ;
        #[allow(unused_imports)] use crate :: ui :: * ;
        #[allow(unused_imports)] use crate :: types :: * ;
        #[allow(unused_imports)] use stm32f1xx_hal ::
        {
            adc :: { Adc, SampleTime }, prelude :: *, serial, gpio ::
            {
                gpiob :: { PB8, PB9, PB6, PB5 }, gpioa ::
                { PA0, PA1, PA4, PA9, PA10 }, { Output, PushPull },
                { Input, PullUp }, { Alternate, OpenDrain }, Edge :: *,
                ExtiPin,
            }, timer :: { Event, Timer }, pac :: { I2C1, USART1, ADC1 }, i2c
            :: { BlockingI2c, DutyCycle, Mode },
        } ; #[allow(unused_imports)] use dwt_systick_monotonic :: DwtSystick ;
        #[allow(unused_imports)] use cortex_m :: asm :: delay ;
        #[allow(unused_imports)] use ssd1306 ::
        { prelude :: *, Builder, I2CDIBuilder, } ; #[allow(unused_imports)]
        use embedded_graphics ::
        {
            fonts :: Text, pixelcolor :: BinaryColor, prelude :: *, style ::
            TextStyle,
        } ; #[allow(unused_imports)] use profont :: ProFont24Point ;
        #[allow(unused_imports)] use embedded_hal :: digital :: v2 ::
        { OutputPin, InputPin } ; #[allow(unused_imports)] use core :: ptr ::
        write_volatile ; #[allow(unused_imports)] use core :: fmt :: Write ;
        #[allow(unused_imports)] use core :: future :: Future ;
        #[allow(unused_imports)] use stm32f1xx_hal :: gpio :: Analog ;
        #[doc = r" Resources initialized at runtime"] #[allow(non_snake_case)]
        pub struct LateResources
        {
            pub EXTI : stm32f1xx_hal :: pac :: EXTI, pub adc1 : Adc < ADC1 >,
            pub button_brightness : PB6 < Input < PullUp > >, pub button_start
            : PB5 < Input < PullUp > >, pub clocks : stm32f1xx_hal :: rcc ::
            Clocks, pub display : GraphicsMode < I2CInterface < BlockingI2c <
            I2C1,
            (PB8 < Alternate < OpenDrain > >, PB9 < Alternate < OpenDrain > >)
            > >, DisplaySize128x64 >, pub pot : PA4 < Analog >, pub pot_pos :
            u16
        } #[doc = r" Monotonics used by the system"] #[allow(non_snake_case)]
        pub struct Monotonics(pub DwtSystick < 8_000_000 >) ;
        #[doc = r" Execution context"] pub struct Context < 'a >
        {
            #[doc = r" Core (Cortex-M) peripherals"] pub core : rtic :: export
            :: Peripherals, #[doc = r" Device peripherals"] pub device :
            stm32f1xx_hal :: pac :: Peripherals,
            #[doc = r" Critical section token for init"] pub cs : rtic ::
            export :: CriticalSection < 'a >,
        } impl < 'a > Context < 'a >
        {
            #[inline(always)] pub unsafe fn
            new(core : rtic :: export :: Peripherals,) -> Self
            {
                Context
                {
                    device : stm32f1xx_hal :: pac :: Peripherals :: steal(),
                    cs : rtic :: export :: CriticalSection :: new(), core,
                }
            }
        }
    } #[allow(non_snake_case)] #[doc = "Idle loop"] pub mod idle
    {
        #[allow(unused_imports)] use crate :: buttons :: * ;
        #[allow(unused_imports)] use crate :: pot :: * ;
        #[allow(unused_imports)] use crate :: ui :: * ;
        #[allow(unused_imports)] use crate :: types :: * ;
        #[allow(unused_imports)] use stm32f1xx_hal ::
        {
            adc :: { Adc, SampleTime }, prelude :: *, serial, gpio ::
            {
                gpiob :: { PB8, PB9, PB6, PB5 }, gpioa ::
                { PA0, PA1, PA4, PA9, PA10 }, { Output, PushPull },
                { Input, PullUp }, { Alternate, OpenDrain }, Edge :: *,
                ExtiPin,
            }, timer :: { Event, Timer }, pac :: { I2C1, USART1, ADC1 }, i2c
            :: { BlockingI2c, DutyCycle, Mode },
        } ; #[allow(unused_imports)] use dwt_systick_monotonic :: DwtSystick ;
        #[allow(unused_imports)] use cortex_m :: asm :: delay ;
        #[allow(unused_imports)] use ssd1306 ::
        { prelude :: *, Builder, I2CDIBuilder, } ; #[allow(unused_imports)]
        use embedded_graphics ::
        {
            fonts :: Text, pixelcolor :: BinaryColor, prelude :: *, style ::
            TextStyle,
        } ; #[allow(unused_imports)] use profont :: ProFont24Point ;
        #[allow(unused_imports)] use embedded_hal :: digital :: v2 ::
        { OutputPin, InputPin } ; #[allow(unused_imports)] use core :: ptr ::
        write_volatile ; #[allow(unused_imports)] use core :: fmt :: Write ;
        #[allow(unused_imports)] use core :: future :: Future ;
        #[allow(unused_imports)] use stm32f1xx_hal :: gpio :: Analog ;
        #[doc = r" Execution context"] pub struct Context < > { } impl < >
        Context < >
        {
            #[inline(always)] pub unsafe fn
            new(priority : & rtic :: export :: Priority) -> Self
            { Context { } }
        }
    } mod resources
    {
        use rtic :: export :: Priority ; #[allow(non_camel_case_types)] pub
        struct clocks < 'a > { priority : & 'a Priority, } impl < 'a > clocks
        < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { clocks { priority } } #[inline(always)] pub unsafe fn
            priority(& self) -> & Priority { self . priority }
        } #[allow(non_camel_case_types)] pub struct button_start < 'a >
        { priority : & 'a Priority, } impl < 'a > button_start < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { button_start { priority } } #[inline(always)] pub unsafe fn
            priority(& self) -> & Priority { self . priority }
        } #[allow(non_camel_case_types)] pub struct button_brightness < 'a >
        { priority : & 'a Priority, } impl < 'a > button_brightness < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { button_brightness { priority } } #[inline(always)] pub
            unsafe fn priority(& self) -> & Priority { self . priority }
        } #[allow(non_camel_case_types)] pub struct EXTI < 'a >
        { priority : & 'a Priority, } impl < 'a > EXTI < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { EXTI { priority } } #[inline(always)] pub unsafe fn
            priority(& self) -> & Priority { self . priority }
        } #[allow(non_camel_case_types)] pub struct display < 'a >
        { priority : & 'a Priority, } impl < 'a > display < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { display { priority } } #[inline(always)] pub unsafe fn
            priority(& self) -> & Priority { self . priority }
        } #[allow(non_camel_case_types)] pub struct brightness_state < 'a >
        { priority : & 'a Priority, } impl < 'a > brightness_state < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { brightness_state { priority } } #[inline(always)] pub
            unsafe fn priority(& self) -> & Priority { self . priority }
        } #[allow(non_camel_case_types)] pub struct pot < 'a >
        { priority : & 'a Priority, } impl < 'a > pot < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { pot { priority } } #[inline(always)] pub unsafe fn
            priority(& self) -> & Priority { self . priority }
        } #[allow(non_camel_case_types)] pub struct pot_pos < 'a >
        { priority : & 'a Priority, } impl < 'a > pot_pos < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { pot_pos { priority } } #[inline(always)] pub unsafe fn
            priority(& self) -> & Priority { self . priority }
        } #[allow(non_camel_case_types)] pub struct adc1 < 'a >
        { priority : & 'a Priority, } impl < 'a > adc1 < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { adc1 { priority } } #[inline(always)] pub unsafe fn
            priority(& self) -> & Priority { self . priority }
        } #[allow(non_camel_case_types)] pub struct pot_dir < 'a >
        { priority : & 'a Priority, } impl < 'a > pot_dir < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { pot_dir { priority } } #[inline(always)] pub unsafe fn
            priority(& self) -> & Priority { self . priority }
        } #[allow(non_camel_case_types)] pub struct time_remaining < 'a >
        { priority : & 'a Priority, } impl < 'a > time_remaining < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { time_remaining { priority } } #[inline(always)] pub unsafe
            fn priority(& self) -> & Priority { self . priority }
        } #[allow(non_camel_case_types)] pub struct disp_call_cnt < 'a >
        { priority : & 'a Priority, } impl < 'a > disp_call_cnt < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { disp_call_cnt { priority } } #[inline(always)] pub unsafe
            fn priority(& self) -> & Priority { self . priority }
        } #[allow(non_camel_case_types)] pub struct sys_state < 'a >
        { priority : & 'a Priority, } impl < 'a > sys_state < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { sys_state { priority } } #[inline(always)] pub unsafe fn
            priority(& self) -> & Priority { self . priority }
        }
    } #[allow(non_snake_case)]
    #[doc = "Resources `handle_buttons` has access to"] pub struct
    __rtic_internal_handle_buttonsResources < 'a >
    {
        pub clocks : & 'a stm32f1xx_hal :: rcc :: Clocks, pub button_start :
        resources :: button_start < 'a >, pub button_brightness : resources ::
        button_brightness < 'a >, pub EXTI : resources :: EXTI < 'a >, pub
        display : resources :: display < 'a >, pub brightness_state :
        resources :: brightness_state < 'a >,
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod handle_buttons
    {
        #[allow(unused_imports)] use crate :: buttons :: * ;
        #[allow(unused_imports)] use crate :: pot :: * ;
        #[allow(unused_imports)] use crate :: ui :: * ;
        #[allow(unused_imports)] use crate :: types :: * ;
        #[allow(unused_imports)] use stm32f1xx_hal ::
        {
            adc :: { Adc, SampleTime }, prelude :: *, serial, gpio ::
            {
                gpiob :: { PB8, PB9, PB6, PB5 }, gpioa ::
                { PA0, PA1, PA4, PA9, PA10 }, { Output, PushPull },
                { Input, PullUp }, { Alternate, OpenDrain }, Edge :: *,
                ExtiPin,
            }, timer :: { Event, Timer }, pac :: { I2C1, USART1, ADC1 }, i2c
            :: { BlockingI2c, DutyCycle, Mode },
        } ; #[allow(unused_imports)] use dwt_systick_monotonic :: DwtSystick ;
        #[allow(unused_imports)] use cortex_m :: asm :: delay ;
        #[allow(unused_imports)] use ssd1306 ::
        { prelude :: *, Builder, I2CDIBuilder, } ; #[allow(unused_imports)]
        use embedded_graphics ::
        {
            fonts :: Text, pixelcolor :: BinaryColor, prelude :: *, style ::
            TextStyle,
        } ; #[allow(unused_imports)] use profont :: ProFont24Point ;
        #[allow(unused_imports)] use embedded_hal :: digital :: v2 ::
        { OutputPin, InputPin } ; #[allow(unused_imports)] use core :: ptr ::
        write_volatile ; #[allow(unused_imports)] use core :: fmt :: Write ;
        #[allow(unused_imports)] use core :: future :: Future ;
        #[allow(unused_imports)] use stm32f1xx_hal :: gpio :: Analog ;
        #[doc(inline)] pub use super ::
        __rtic_internal_handle_buttonsResources as Resources ;
        #[doc = r" Execution context"] pub struct Context < 'a >
        {
            #[doc = r" Resources this task has access to"] pub resources :
            Resources < 'a >,
        } impl < 'a > Context < 'a >
        {
            #[inline(always)] pub unsafe fn
            new(priority : & 'a rtic :: export :: Priority) -> Self
            { Context { resources : Resources :: new(priority), } }
        }
    } #[allow(non_snake_case)] #[doc = "Resources `handle_adc` has access to"]
    pub struct __rtic_internal_handle_adcResources < 'a >
    {
        pub pot : resources :: pot < 'a >, pub pot_pos : resources :: pot_pos
        < 'a >, pub adc1 : resources :: adc1 < 'a >, pub pot_dir : resources
        :: pot_dir < 'a >, pub time_remaining : resources :: time_remaining <
        'a >,
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod handle_adc
    {
        #[allow(unused_imports)] use crate :: buttons :: * ;
        #[allow(unused_imports)] use crate :: pot :: * ;
        #[allow(unused_imports)] use crate :: ui :: * ;
        #[allow(unused_imports)] use crate :: types :: * ;
        #[allow(unused_imports)] use stm32f1xx_hal ::
        {
            adc :: { Adc, SampleTime }, prelude :: *, serial, gpio ::
            {
                gpiob :: { PB8, PB9, PB6, PB5 }, gpioa ::
                { PA0, PA1, PA4, PA9, PA10 }, { Output, PushPull },
                { Input, PullUp }, { Alternate, OpenDrain }, Edge :: *,
                ExtiPin,
            }, timer :: { Event, Timer }, pac :: { I2C1, USART1, ADC1 }, i2c
            :: { BlockingI2c, DutyCycle, Mode },
        } ; #[allow(unused_imports)] use dwt_systick_monotonic :: DwtSystick ;
        #[allow(unused_imports)] use cortex_m :: asm :: delay ;
        #[allow(unused_imports)] use ssd1306 ::
        { prelude :: *, Builder, I2CDIBuilder, } ; #[allow(unused_imports)]
        use embedded_graphics ::
        {
            fonts :: Text, pixelcolor :: BinaryColor, prelude :: *, style ::
            TextStyle,
        } ; #[allow(unused_imports)] use profont :: ProFont24Point ;
        #[allow(unused_imports)] use embedded_hal :: digital :: v2 ::
        { OutputPin, InputPin } ; #[allow(unused_imports)] use core :: ptr ::
        write_volatile ; #[allow(unused_imports)] use core :: fmt :: Write ;
        #[allow(unused_imports)] use core :: future :: Future ;
        #[allow(unused_imports)] use stm32f1xx_hal :: gpio :: Analog ;
        #[doc(inline)] pub use super :: __rtic_internal_handle_adcResources as
        Resources ; #[doc = r" Execution context"] pub struct Context < 'a >
        {
            #[doc = r" Resources this task has access to"] pub resources :
            Resources < 'a >,
        } impl < 'a > Context < 'a >
        {
            #[inline(always)] pub unsafe fn
            new(priority : & 'a rtic :: export :: Priority) -> Self
            { Context { resources : Resources :: new(priority), } }
        } #[doc = r" Spawns the task directly"] pub fn spawn(_0 : bool,) ->
        Result < (), bool >
        {
            let input = _0 ; unsafe
            {
                if let Some(index) = rtic :: export :: interrupt ::
                free(| _ | crate :: app :: __rtic_internal_handle_adc_FQ .
                     dequeue())
                {
                    crate :: app :: __rtic_internal_handle_adc_INPUTS .
                    get_unchecked_mut(usize :: from(index)) . as_mut_ptr() .
                    write(input) ; rtic :: export :: interrupt ::
                    free(| _ |
                         {
                             crate :: app :: __rtic_internal_P1_RQ .
                             enqueue_unchecked((crate :: app :: P1_T ::
                                                handle_adc, index)) ;
                         }) ; rtic ::
                    pend(stm32f1xx_hal :: pac :: interrupt :: DMA1_CHANNEL1) ;
                    Ok(())
                } else { Err(input) }
            }
        } pub use MyMono :: spawn_after ; pub use MyMono :: spawn_at ; pub use
        MyMono :: SpawnHandle ;
        #[doc = r" Holds methods related to this monotonic"] pub mod MyMono
        {
            use super :: * ; #[allow(unused_imports)] use crate :: app ::
            __rtic_internal_TIMER_QUEUE_MARKER ; #[allow(unused_imports)] use
            crate :: app :: SCHED_T ; #[allow(unused_imports)] use crate ::
            buttons :: * ; #[allow(unused_imports)] use crate :: pot :: * ;
            #[allow(unused_imports)] use crate :: ui :: * ;
            #[allow(unused_imports)] use crate :: types :: * ;
            #[allow(unused_imports)] use stm32f1xx_hal ::
            {
                adc :: { Adc, SampleTime }, prelude :: *, serial, gpio ::
                {
                    gpiob :: { PB8, PB9, PB6, PB5 }, gpioa ::
                    { PA0, PA1, PA4, PA9, PA10 }, { Output, PushPull },
                    { Input, PullUp }, { Alternate, OpenDrain }, Edge :: *,
                    ExtiPin,
                }, timer :: { Event, Timer }, pac :: { I2C1, USART1, ADC1 },
                i2c :: { BlockingI2c, DutyCycle, Mode },
            } ; #[allow(unused_imports)] use dwt_systick_monotonic ::
            DwtSystick ; #[allow(unused_imports)] use cortex_m :: asm :: delay
            ; #[allow(unused_imports)] use ssd1306 ::
            { prelude :: *, Builder, I2CDIBuilder, } ;
            #[allow(unused_imports)] use embedded_graphics ::
            {
                fonts :: Text, pixelcolor :: BinaryColor, prelude :: *, style
                :: TextStyle,
            } ; #[allow(unused_imports)] use profont :: ProFont24Point ;
            #[allow(unused_imports)] use embedded_hal :: digital :: v2 ::
            { OutputPin, InputPin } ; #[allow(unused_imports)] use core :: ptr
            :: write_volatile ; #[allow(unused_imports)] use core :: fmt ::
            Write ; #[allow(unused_imports)] use core :: future :: Future ;
            #[allow(unused_imports)] use stm32f1xx_hal :: gpio :: Analog ; pub
            struct SpawnHandle { #[doc(hidden)] marker : u32, } impl
            SpawnHandle
            {
                pub fn cancel(self) -> Result < bool, () >
                {
                    rtic :: export :: interrupt ::
                    free(| _ | unsafe
                         {
                             let tq = & mut * crate :: app ::
                             __rtic_internal_TQ_MyMono . as_mut_ptr() ; if let
                             Some((_task, index)) = tq .
                             cancel_marker(self . marker)
                             {
                                 let msg = crate :: app ::
                                 __rtic_internal_handle_adc_INPUTS .
                                 get_unchecked(usize :: from(index)) .
                                 as_ptr() . read() ; crate :: app ::
                                 __rtic_internal_handle_adc_FQ . split() . 0 .
                                 enqueue_unchecked(index) ; Ok(msg)
                             } else { Err(()) }
                         })
                } #[inline] pub fn reschedule_after < D > (self, duration : D)
                -> Result < Self, () > where D : rtic :: time :: duration ::
                Duration + rtic :: time :: fixed_point :: FixedPoint, D :: T :
                Into << crate :: app :: MyMono as rtic :: time :: Clock > :: T
                >,
                {
                    self .
                    reschedule_at(crate :: app :: monotonics :: MyMono ::
                                  now() + duration)
                } pub fn
                reschedule_at(self, instant : rtic :: time :: Instant < crate
                              :: app :: MyMono >) -> Result < Self, () >
                {
                    rtic :: export :: interrupt ::
                    free(| _ | unsafe
                         {
                             let marker = __rtic_internal_TIMER_QUEUE_MARKER ;
                             __rtic_internal_TIMER_QUEUE_MARKER =
                             __rtic_internal_TIMER_QUEUE_MARKER .
                             wrapping_add(1) ; let tq = & mut * crate :: app
                             :: __rtic_internal_TQ_MyMono . as_mut_ptr() ; tq
                             .
                             update_marker(self . marker, marker, instant, ||
                                           cortex_m :: peripheral :: SCB ::
                                           set_pendst()) .
                             map(| _ | SpawnHandle { marker })
                         })
                }
            }
            #[doc =
              r" Spawns the task after a set duration relative to the current time"]
            #[doc = r""]
            #[doc =
              r" This will use the time `Instant::new(0)` as baseline if called in `#[init]`,"]
            #[doc =
              r" so if you use a non-resetable timer use `spawn_at` when in `#[init]`"]
            pub fn spawn_after < D > (duration : D, _0 : bool) -> Result <
            SpawnHandle, bool > where D : rtic :: time :: duration :: Duration
            + rtic :: time :: fixed_point :: FixedPoint, D :: T : Into <<
            crate :: app :: MyMono as rtic :: time :: Clock > :: T >,
            {
                let instant = if rtic :: export :: interrupt ::
                free(| _ | unsafe
                     {
                         crate :: app ::
                         __rtic_internal_MONOTONIC_STORAGE_MyMono . is_none()
                     }) { rtic :: time :: Instant :: new(0) } else
                { crate :: app :: monotonics :: MyMono :: now() } ;
                spawn_at(instant + duration, _0)
            } #[doc = r" Spawns the task at a fixed time instant"] pub fn
            spawn_at(instant : rtic :: time :: Instant < crate :: app ::
                     MyMono >, _0 : bool) -> Result < SpawnHandle, bool >
            {
                unsafe
                {
                    let input = _0 ; if let Some(index) = rtic :: export ::
                    interrupt ::
                    free(| _ | crate :: app :: __rtic_internal_handle_adc_FQ .
                         dequeue())
                    {
                        crate :: app :: __rtic_internal_handle_adc_INPUTS .
                        get_unchecked_mut(usize :: from(index)) . as_mut_ptr()
                        . write(input) ; crate :: app ::
                        __rtic_internal_handle_adc_MyMono_INSTANTS .
                        get_unchecked_mut(usize :: from(index)) . as_mut_ptr()
                        . write(instant) ; rtic :: export :: interrupt ::
                        free(| _ |
                             {
                                 let marker =
                                 __rtic_internal_TIMER_QUEUE_MARKER ; let nr =
                                 rtic :: export :: NotReady
                                 {
                                     instant, index, task : crate :: app ::
                                     SCHED_T :: handle_adc, marker,
                                 } ; __rtic_internal_TIMER_QUEUE_MARKER =
                                 __rtic_internal_TIMER_QUEUE_MARKER .
                                 wrapping_add(1) ; let tq = unsafe
                                 {
                                     & mut * crate :: app ::
                                     __rtic_internal_TQ_MyMono . as_mut_ptr()
                                 } ; tq .
                                 enqueue_unchecked(nr, || core :: mem ::
                                                   transmute :: < _, cortex_m
                                                   :: peripheral :: SYST >
                                                   (()) . enable_interrupt(),
                                                   || cortex_m :: peripheral
                                                   :: SCB :: set_pendst(),
                                                   crate :: app ::
                                                   __rtic_internal_MONOTONIC_STORAGE_MyMono
                                                   . as_mut()) ;
                                 Ok(SpawnHandle { marker })
                             })
                    } else { Err(input) }
                }
            }
        }
    } #[allow(non_snake_case)]
    #[doc = "Resources `update_display` has access to"] pub struct
    __rtic_internal_update_displayResources < 'a >
    {
        pub display : resources :: display < 'a >, pub time_remaining :
        resources :: time_remaining < 'a >, pub brightness_state : resources
        :: brightness_state < 'a >, pub disp_call_cnt : resources ::
        disp_call_cnt < 'a >,
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod update_display
    {
        #[allow(unused_imports)] use crate :: buttons :: * ;
        #[allow(unused_imports)] use crate :: pot :: * ;
        #[allow(unused_imports)] use crate :: ui :: * ;
        #[allow(unused_imports)] use crate :: types :: * ;
        #[allow(unused_imports)] use stm32f1xx_hal ::
        {
            adc :: { Adc, SampleTime }, prelude :: *, serial, gpio ::
            {
                gpiob :: { PB8, PB9, PB6, PB5 }, gpioa ::
                { PA0, PA1, PA4, PA9, PA10 }, { Output, PushPull },
                { Input, PullUp }, { Alternate, OpenDrain }, Edge :: *,
                ExtiPin,
            }, timer :: { Event, Timer }, pac :: { I2C1, USART1, ADC1 }, i2c
            :: { BlockingI2c, DutyCycle, Mode },
        } ; #[allow(unused_imports)] use dwt_systick_monotonic :: DwtSystick ;
        #[allow(unused_imports)] use cortex_m :: asm :: delay ;
        #[allow(unused_imports)] use ssd1306 ::
        { prelude :: *, Builder, I2CDIBuilder, } ; #[allow(unused_imports)]
        use embedded_graphics ::
        {
            fonts :: Text, pixelcolor :: BinaryColor, prelude :: *, style ::
            TextStyle,
        } ; #[allow(unused_imports)] use profont :: ProFont24Point ;
        #[allow(unused_imports)] use embedded_hal :: digital :: v2 ::
        { OutputPin, InputPin } ; #[allow(unused_imports)] use core :: ptr ::
        write_volatile ; #[allow(unused_imports)] use core :: fmt :: Write ;
        #[allow(unused_imports)] use core :: future :: Future ;
        #[allow(unused_imports)] use stm32f1xx_hal :: gpio :: Analog ;
        #[doc(inline)] pub use super ::
        __rtic_internal_update_displayResources as Resources ;
        #[doc = r" Execution context"] pub struct Context < 'a >
        {
            #[doc = r" Resources this task has access to"] pub resources :
            Resources < 'a >,
        } impl < 'a > Context < 'a >
        {
            #[inline(always)] pub unsafe fn
            new(priority : & 'a rtic :: export :: Priority) -> Self
            { Context { resources : Resources :: new(priority), } }
        } #[doc = r" Spawns the task directly"] pub fn spawn(_0 : ScreenPage,)
        -> Result < (), ScreenPage >
        {
            let input = _0 ; unsafe
            {
                if let Some(index) = rtic :: export :: interrupt ::
                free(| _ | crate :: app :: __rtic_internal_update_display_FQ .
                     dequeue())
                {
                    crate :: app :: __rtic_internal_update_display_INPUTS .
                    get_unchecked_mut(usize :: from(index)) . as_mut_ptr() .
                    write(input) ; rtic :: export :: interrupt ::
                    free(| _ |
                         {
                             crate :: app :: __rtic_internal_P1_RQ .
                             enqueue_unchecked((crate :: app :: P1_T ::
                                                update_display, index)) ;
                         }) ; rtic ::
                    pend(stm32f1xx_hal :: pac :: interrupt :: DMA1_CHANNEL1) ;
                    Ok(())
                } else { Err(input) }
            }
        } pub use MyMono :: spawn_after ; pub use MyMono :: spawn_at ; pub use
        MyMono :: SpawnHandle ;
        #[doc = r" Holds methods related to this monotonic"] pub mod MyMono
        {
            use super :: * ; #[allow(unused_imports)] use crate :: app ::
            __rtic_internal_TIMER_QUEUE_MARKER ; #[allow(unused_imports)] use
            crate :: app :: SCHED_T ; #[allow(unused_imports)] use crate ::
            buttons :: * ; #[allow(unused_imports)] use crate :: pot :: * ;
            #[allow(unused_imports)] use crate :: ui :: * ;
            #[allow(unused_imports)] use crate :: types :: * ;
            #[allow(unused_imports)] use stm32f1xx_hal ::
            {
                adc :: { Adc, SampleTime }, prelude :: *, serial, gpio ::
                {
                    gpiob :: { PB8, PB9, PB6, PB5 }, gpioa ::
                    { PA0, PA1, PA4, PA9, PA10 }, { Output, PushPull },
                    { Input, PullUp }, { Alternate, OpenDrain }, Edge :: *,
                    ExtiPin,
                }, timer :: { Event, Timer }, pac :: { I2C1, USART1, ADC1 },
                i2c :: { BlockingI2c, DutyCycle, Mode },
            } ; #[allow(unused_imports)] use dwt_systick_monotonic ::
            DwtSystick ; #[allow(unused_imports)] use cortex_m :: asm :: delay
            ; #[allow(unused_imports)] use ssd1306 ::
            { prelude :: *, Builder, I2CDIBuilder, } ;
            #[allow(unused_imports)] use embedded_graphics ::
            {
                fonts :: Text, pixelcolor :: BinaryColor, prelude :: *, style
                :: TextStyle,
            } ; #[allow(unused_imports)] use profont :: ProFont24Point ;
            #[allow(unused_imports)] use embedded_hal :: digital :: v2 ::
            { OutputPin, InputPin } ; #[allow(unused_imports)] use core :: ptr
            :: write_volatile ; #[allow(unused_imports)] use core :: fmt ::
            Write ; #[allow(unused_imports)] use core :: future :: Future ;
            #[allow(unused_imports)] use stm32f1xx_hal :: gpio :: Analog ; pub
            struct SpawnHandle { #[doc(hidden)] marker : u32, } impl
            SpawnHandle
            {
                pub fn cancel(self) -> Result < ScreenPage, () >
                {
                    rtic :: export :: interrupt ::
                    free(| _ | unsafe
                         {
                             let tq = & mut * crate :: app ::
                             __rtic_internal_TQ_MyMono . as_mut_ptr() ; if let
                             Some((_task, index)) = tq .
                             cancel_marker(self . marker)
                             {
                                 let msg = crate :: app ::
                                 __rtic_internal_update_display_INPUTS .
                                 get_unchecked(usize :: from(index)) .
                                 as_ptr() . read() ; crate :: app ::
                                 __rtic_internal_update_display_FQ . split() .
                                 0 . enqueue_unchecked(index) ; Ok(msg)
                             } else { Err(()) }
                         })
                } #[inline] pub fn reschedule_after < D > (self, duration : D)
                -> Result < Self, () > where D : rtic :: time :: duration ::
                Duration + rtic :: time :: fixed_point :: FixedPoint, D :: T :
                Into << crate :: app :: MyMono as rtic :: time :: Clock > :: T
                >,
                {
                    self .
                    reschedule_at(crate :: app :: monotonics :: MyMono ::
                                  now() + duration)
                } pub fn
                reschedule_at(self, instant : rtic :: time :: Instant < crate
                              :: app :: MyMono >) -> Result < Self, () >
                {
                    rtic :: export :: interrupt ::
                    free(| _ | unsafe
                         {
                             let marker = __rtic_internal_TIMER_QUEUE_MARKER ;
                             __rtic_internal_TIMER_QUEUE_MARKER =
                             __rtic_internal_TIMER_QUEUE_MARKER .
                             wrapping_add(1) ; let tq = & mut * crate :: app
                             :: __rtic_internal_TQ_MyMono . as_mut_ptr() ; tq
                             .
                             update_marker(self . marker, marker, instant, ||
                                           cortex_m :: peripheral :: SCB ::
                                           set_pendst()) .
                             map(| _ | SpawnHandle { marker })
                         })
                }
            }
            #[doc =
              r" Spawns the task after a set duration relative to the current time"]
            #[doc = r""]
            #[doc =
              r" This will use the time `Instant::new(0)` as baseline if called in `#[init]`,"]
            #[doc =
              r" so if you use a non-resetable timer use `spawn_at` when in `#[init]`"]
            pub fn spawn_after < D > (duration : D, _0 : ScreenPage) -> Result
            < SpawnHandle, ScreenPage > where D : rtic :: time :: duration ::
            Duration + rtic :: time :: fixed_point :: FixedPoint, D :: T :
            Into << crate :: app :: MyMono as rtic :: time :: Clock > :: T >,
            {
                let instant = if rtic :: export :: interrupt ::
                free(| _ | unsafe
                     {
                         crate :: app ::
                         __rtic_internal_MONOTONIC_STORAGE_MyMono . is_none()
                     }) { rtic :: time :: Instant :: new(0) } else
                { crate :: app :: monotonics :: MyMono :: now() } ;
                spawn_at(instant + duration, _0)
            } #[doc = r" Spawns the task at a fixed time instant"] pub fn
            spawn_at(instant : rtic :: time :: Instant < crate :: app ::
                     MyMono >, _0 : ScreenPage) -> Result < SpawnHandle,
            ScreenPage >
            {
                unsafe
                {
                    let input = _0 ; if let Some(index) = rtic :: export ::
                    interrupt ::
                    free(| _ | crate :: app ::
                         __rtic_internal_update_display_FQ . dequeue())
                    {
                        crate :: app :: __rtic_internal_update_display_INPUTS
                        . get_unchecked_mut(usize :: from(index)) .
                        as_mut_ptr() . write(input) ; crate :: app ::
                        __rtic_internal_update_display_MyMono_INSTANTS .
                        get_unchecked_mut(usize :: from(index)) . as_mut_ptr()
                        . write(instant) ; rtic :: export :: interrupt ::
                        free(| _ |
                             {
                                 let marker =
                                 __rtic_internal_TIMER_QUEUE_MARKER ; let nr =
                                 rtic :: export :: NotReady
                                 {
                                     instant, index, task : crate :: app ::
                                     SCHED_T :: update_display, marker,
                                 } ; __rtic_internal_TIMER_QUEUE_MARKER =
                                 __rtic_internal_TIMER_QUEUE_MARKER .
                                 wrapping_add(1) ; let tq = unsafe
                                 {
                                     & mut * crate :: app ::
                                     __rtic_internal_TQ_MyMono . as_mut_ptr()
                                 } ; tq .
                                 enqueue_unchecked(nr, || core :: mem ::
                                                   transmute :: < _, cortex_m
                                                   :: peripheral :: SYST >
                                                   (()) . enable_interrupt(),
                                                   || cortex_m :: peripheral
                                                   :: SCB :: set_pendst(),
                                                   crate :: app ::
                                                   __rtic_internal_MONOTONIC_STORAGE_MyMono
                                                   . as_mut()) ;
                                 Ok(SpawnHandle { marker })
                             })
                    } else { Err(input) }
                }
            }
        }
    } #[allow(non_snake_case)]
    #[doc = "Resources `reset_display` has access to"] pub struct
    __rtic_internal_reset_displayResources < 'a >
    {
        pub disp_call_cnt : resources :: disp_call_cnt < 'a >, pub sys_state :
        resources :: sys_state < 'a >,
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod reset_display
    {
        #[allow(unused_imports)] use crate :: buttons :: * ;
        #[allow(unused_imports)] use crate :: pot :: * ;
        #[allow(unused_imports)] use crate :: ui :: * ;
        #[allow(unused_imports)] use crate :: types :: * ;
        #[allow(unused_imports)] use stm32f1xx_hal ::
        {
            adc :: { Adc, SampleTime }, prelude :: *, serial, gpio ::
            {
                gpiob :: { PB8, PB9, PB6, PB5 }, gpioa ::
                { PA0, PA1, PA4, PA9, PA10 }, { Output, PushPull },
                { Input, PullUp }, { Alternate, OpenDrain }, Edge :: *,
                ExtiPin,
            }, timer :: { Event, Timer }, pac :: { I2C1, USART1, ADC1 }, i2c
            :: { BlockingI2c, DutyCycle, Mode },
        } ; #[allow(unused_imports)] use dwt_systick_monotonic :: DwtSystick ;
        #[allow(unused_imports)] use cortex_m :: asm :: delay ;
        #[allow(unused_imports)] use ssd1306 ::
        { prelude :: *, Builder, I2CDIBuilder, } ; #[allow(unused_imports)]
        use embedded_graphics ::
        {
            fonts :: Text, pixelcolor :: BinaryColor, prelude :: *, style ::
            TextStyle,
        } ; #[allow(unused_imports)] use profont :: ProFont24Point ;
        #[allow(unused_imports)] use embedded_hal :: digital :: v2 ::
        { OutputPin, InputPin } ; #[allow(unused_imports)] use core :: ptr ::
        write_volatile ; #[allow(unused_imports)] use core :: fmt :: Write ;
        #[allow(unused_imports)] use core :: future :: Future ;
        #[allow(unused_imports)] use stm32f1xx_hal :: gpio :: Analog ;
        #[doc(inline)] pub use super :: __rtic_internal_reset_displayResources
        as Resources ; #[doc = r" Execution context"] pub struct Context < 'a
        >
        {
            #[doc = r" Resources this task has access to"] pub resources :
            Resources < 'a >,
        } impl < 'a > Context < 'a >
        {
            #[inline(always)] pub unsafe fn
            new(priority : & 'a rtic :: export :: Priority) -> Self
            { Context { resources : Resources :: new(priority), } }
        } #[doc = r" Spawns the task directly"] pub fn spawn() -> Result < (),
        () >
        {
            let input = () ; unsafe
            {
                if let Some(index) = rtic :: export :: interrupt ::
                free(| _ | crate :: app :: __rtic_internal_reset_display_FQ .
                     dequeue())
                {
                    crate :: app :: __rtic_internal_reset_display_INPUTS .
                    get_unchecked_mut(usize :: from(index)) . as_mut_ptr() .
                    write(input) ; rtic :: export :: interrupt ::
                    free(| _ |
                         {
                             crate :: app :: __rtic_internal_P1_RQ .
                             enqueue_unchecked((crate :: app :: P1_T ::
                                                reset_display, index)) ;
                         }) ; rtic ::
                    pend(stm32f1xx_hal :: pac :: interrupt :: DMA1_CHANNEL1) ;
                    Ok(())
                } else { Err(input) }
            }
        } pub use MyMono :: spawn_after ; pub use MyMono :: spawn_at ; pub use
        MyMono :: SpawnHandle ;
        #[doc = r" Holds methods related to this monotonic"] pub mod MyMono
        {
            use super :: * ; #[allow(unused_imports)] use crate :: app ::
            __rtic_internal_TIMER_QUEUE_MARKER ; #[allow(unused_imports)] use
            crate :: app :: SCHED_T ; #[allow(unused_imports)] use crate ::
            buttons :: * ; #[allow(unused_imports)] use crate :: pot :: * ;
            #[allow(unused_imports)] use crate :: ui :: * ;
            #[allow(unused_imports)] use crate :: types :: * ;
            #[allow(unused_imports)] use stm32f1xx_hal ::
            {
                adc :: { Adc, SampleTime }, prelude :: *, serial, gpio ::
                {
                    gpiob :: { PB8, PB9, PB6, PB5 }, gpioa ::
                    { PA0, PA1, PA4, PA9, PA10 }, { Output, PushPull },
                    { Input, PullUp }, { Alternate, OpenDrain }, Edge :: *,
                    ExtiPin,
                }, timer :: { Event, Timer }, pac :: { I2C1, USART1, ADC1 },
                i2c :: { BlockingI2c, DutyCycle, Mode },
            } ; #[allow(unused_imports)] use dwt_systick_monotonic ::
            DwtSystick ; #[allow(unused_imports)] use cortex_m :: asm :: delay
            ; #[allow(unused_imports)] use ssd1306 ::
            { prelude :: *, Builder, I2CDIBuilder, } ;
            #[allow(unused_imports)] use embedded_graphics ::
            {
                fonts :: Text, pixelcolor :: BinaryColor, prelude :: *, style
                :: TextStyle,
            } ; #[allow(unused_imports)] use profont :: ProFont24Point ;
            #[allow(unused_imports)] use embedded_hal :: digital :: v2 ::
            { OutputPin, InputPin } ; #[allow(unused_imports)] use core :: ptr
            :: write_volatile ; #[allow(unused_imports)] use core :: fmt ::
            Write ; #[allow(unused_imports)] use core :: future :: Future ;
            #[allow(unused_imports)] use stm32f1xx_hal :: gpio :: Analog ; pub
            struct SpawnHandle { #[doc(hidden)] marker : u32, } impl
            SpawnHandle
            {
                pub fn cancel(self) -> Result < (), () >
                {
                    rtic :: export :: interrupt ::
                    free(| _ | unsafe
                         {
                             let tq = & mut * crate :: app ::
                             __rtic_internal_TQ_MyMono . as_mut_ptr() ; if let
                             Some((_task, index)) = tq .
                             cancel_marker(self . marker)
                             {
                                 let msg = crate :: app ::
                                 __rtic_internal_reset_display_INPUTS .
                                 get_unchecked(usize :: from(index)) .
                                 as_ptr() . read() ; crate :: app ::
                                 __rtic_internal_reset_display_FQ . split() .
                                 0 . enqueue_unchecked(index) ; Ok(msg)
                             } else { Err(()) }
                         })
                } #[inline] pub fn reschedule_after < D > (self, duration : D)
                -> Result < Self, () > where D : rtic :: time :: duration ::
                Duration + rtic :: time :: fixed_point :: FixedPoint, D :: T :
                Into << crate :: app :: MyMono as rtic :: time :: Clock > :: T
                >,
                {
                    self .
                    reschedule_at(crate :: app :: monotonics :: MyMono ::
                                  now() + duration)
                } pub fn
                reschedule_at(self, instant : rtic :: time :: Instant < crate
                              :: app :: MyMono >) -> Result < Self, () >
                {
                    rtic :: export :: interrupt ::
                    free(| _ | unsafe
                         {
                             let marker = __rtic_internal_TIMER_QUEUE_MARKER ;
                             __rtic_internal_TIMER_QUEUE_MARKER =
                             __rtic_internal_TIMER_QUEUE_MARKER .
                             wrapping_add(1) ; let tq = & mut * crate :: app
                             :: __rtic_internal_TQ_MyMono . as_mut_ptr() ; tq
                             .
                             update_marker(self . marker, marker, instant, ||
                                           cortex_m :: peripheral :: SCB ::
                                           set_pendst()) .
                             map(| _ | SpawnHandle { marker })
                         })
                }
            }
            #[doc =
              r" Spawns the task after a set duration relative to the current time"]
            #[doc = r""]
            #[doc =
              r" This will use the time `Instant::new(0)` as baseline if called in `#[init]`,"]
            #[doc =
              r" so if you use a non-resetable timer use `spawn_at` when in `#[init]`"]
            pub fn spawn_after < D > (duration : D) -> Result < SpawnHandle,
            () > where D : rtic :: time :: duration :: Duration + rtic :: time
            :: fixed_point :: FixedPoint, D :: T : Into << crate :: app ::
            MyMono as rtic :: time :: Clock > :: T >,
            {
                let instant = if rtic :: export :: interrupt ::
                free(| _ | unsafe
                     {
                         crate :: app ::
                         __rtic_internal_MONOTONIC_STORAGE_MyMono . is_none()
                     }) { rtic :: time :: Instant :: new(0) } else
                { crate :: app :: monotonics :: MyMono :: now() } ;
                spawn_at(instant + duration)
            } #[doc = r" Spawns the task at a fixed time instant"] pub fn
            spawn_at(instant : rtic :: time :: Instant < crate :: app ::
                     MyMono >) -> Result < SpawnHandle, () >
            {
                unsafe
                {
                    let input = () ; if let Some(index) = rtic :: export ::
                    interrupt ::
                    free(| _ | crate :: app ::
                         __rtic_internal_reset_display_FQ . dequeue())
                    {
                        crate :: app :: __rtic_internal_reset_display_INPUTS .
                        get_unchecked_mut(usize :: from(index)) . as_mut_ptr()
                        . write(input) ; crate :: app ::
                        __rtic_internal_reset_display_MyMono_INSTANTS .
                        get_unchecked_mut(usize :: from(index)) . as_mut_ptr()
                        . write(instant) ; rtic :: export :: interrupt ::
                        free(| _ |
                             {
                                 let marker =
                                 __rtic_internal_TIMER_QUEUE_MARKER ; let nr =
                                 rtic :: export :: NotReady
                                 {
                                     instant, index, task : crate :: app ::
                                     SCHED_T :: reset_display, marker,
                                 } ; __rtic_internal_TIMER_QUEUE_MARKER =
                                 __rtic_internal_TIMER_QUEUE_MARKER .
                                 wrapping_add(1) ; let tq = unsafe
                                 {
                                     & mut * crate :: app ::
                                     __rtic_internal_TQ_MyMono . as_mut_ptr()
                                 } ; tq .
                                 enqueue_unchecked(nr, || core :: mem ::
                                                   transmute :: < _, cortex_m
                                                   :: peripheral :: SYST >
                                                   (()) . enable_interrupt(),
                                                   || cortex_m :: peripheral
                                                   :: SCB :: set_pendst(),
                                                   crate :: app ::
                                                   __rtic_internal_MONOTONIC_STORAGE_MyMono
                                                   . as_mut()) ;
                                 Ok(SpawnHandle { marker })
                             })
                    } else { Err(input) }
                }
            }
        }
    } #[doc = r" app module"] #[allow(non_upper_case_globals)] #[doc(hidden)]
    #[link_section = ".uninit.rtic0"] static mut __rtic_internal_clocks : core
    :: mem :: MaybeUninit < stm32f1xx_hal :: rcc :: Clocks > = core :: mem ::
    MaybeUninit :: uninit() ; impl < 'a > rtic :: Mutex for resources ::
    clocks < 'a >
    {
        type T = stm32f1xx_hal :: rcc :: Clocks ; #[inline(always)] fn lock <
        RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut stm32f1xx_hal :: rcc :: Clocks) ->
         RTIC_INTERNAL_R) -> RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_clocks . as_mut_ptr(), self . priority(),
                     CEILING, stm32f1xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_upper_case_globals)] #[doc(hidden)]
    #[link_section = ".uninit.rtic1"] static mut __rtic_internal_button_start
    : core :: mem :: MaybeUninit < PB5 < Input < PullUp > > > = core :: mem ::
    MaybeUninit :: uninit() ; impl < 'a > rtic :: Mutex for resources ::
    button_start < 'a >
    {
        type T = PB5 < Input < PullUp > > ; #[inline(always)] fn lock <
        RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut PB5 < Input < PullUp > >) ->
         RTIC_INTERNAL_R) -> RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_button_start . as_mut_ptr(), self .
                     priority(), CEILING, stm32f1xx_hal :: pac ::
                     NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_upper_case_globals)] #[doc(hidden)]
    #[link_section = ".uninit.rtic2"] static mut
    __rtic_internal_button_brightness : core :: mem :: MaybeUninit < PB6 <
    Input < PullUp > > > = core :: mem :: MaybeUninit :: uninit() ; impl < 'a
    > rtic :: Mutex for resources :: button_brightness < 'a >
    {
        type T = PB6 < Input < PullUp > > ; #[inline(always)] fn lock <
        RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut PB6 < Input < PullUp > >) ->
         RTIC_INTERNAL_R) -> RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_button_brightness . as_mut_ptr(), self .
                     priority(), CEILING, stm32f1xx_hal :: pac ::
                     NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_upper_case_globals)] #[doc(hidden)]
    #[link_section = ".uninit.rtic3"] static mut __rtic_internal_EXTI : core
    :: mem :: MaybeUninit < stm32f1xx_hal :: pac :: EXTI > = core :: mem ::
    MaybeUninit :: uninit() ; impl < 'a > rtic :: Mutex for resources :: EXTI
    < 'a >
    {
        type T = stm32f1xx_hal :: pac :: EXTI ; #[inline(always)] fn lock <
        RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut stm32f1xx_hal :: pac :: EXTI) ->
         RTIC_INTERNAL_R) -> RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_EXTI . as_mut_ptr(), self . priority(),
                     CEILING, stm32f1xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_upper_case_globals)] #[doc(hidden)]
    #[link_section = ".uninit.rtic4"] static mut __rtic_internal_display :
    core :: mem :: MaybeUninit < GraphicsMode < I2CInterface < BlockingI2c <
    I2C1, (PB8 < Alternate < OpenDrain > >, PB9 < Alternate < OpenDrain > >) >
    >, DisplaySize128x64 > > = core :: mem :: MaybeUninit :: uninit() ; impl <
    'a > rtic :: Mutex for resources :: display < 'a >
    {
        type T = GraphicsMode < I2CInterface < BlockingI2c < I2C1,
        (PB8 < Alternate < OpenDrain > >, PB9 < Alternate < OpenDrain > >) >
        >, DisplaySize128x64 > ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl
         FnOnce(& mut GraphicsMode < I2CInterface < BlockingI2c < I2C1,
                (PB8 < Alternate < OpenDrain > >, PB9 < Alternate < OpenDrain
                 > >) > >, DisplaySize128x64 >) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_display . as_mut_ptr(), self .
                     priority(), CEILING, stm32f1xx_hal :: pac ::
                     NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_upper_case_globals)] #[doc(hidden)] static mut
    __rtic_internal_brightness_state : u8 = 0 ; impl < 'a > rtic :: Mutex for
    resources :: brightness_state < 'a >
    {
        type T = u8 ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut u8) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(& mut __rtic_internal_brightness_state, self .
                     priority(), CEILING, stm32f1xx_hal :: pac ::
                     NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_upper_case_globals)] #[doc(hidden)]
    #[link_section = ".uninit.rtic5"] static mut __rtic_internal_pot : core ::
    mem :: MaybeUninit < PA4 < Analog > > = core :: mem :: MaybeUninit ::
    uninit() ; impl < 'a > rtic :: Mutex for resources :: pot < 'a >
    {
        type T = PA4 < Analog > ; #[inline(always)] fn lock < RTIC_INTERNAL_R
        >
        (& mut self, f : impl FnOnce(& mut PA4 < Analog >) -> RTIC_INTERNAL_R)
        -> RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_pot . as_mut_ptr(), self . priority(),
                     CEILING, stm32f1xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_upper_case_globals)] #[doc(hidden)]
    #[link_section = ".uninit.rtic6"] static mut __rtic_internal_pot_pos :
    core :: mem :: MaybeUninit < u16 > = core :: mem :: MaybeUninit ::
    uninit() ; impl < 'a > rtic :: Mutex for resources :: pot_pos < 'a >
    {
        type T = u16 ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut u16) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_pot_pos . as_mut_ptr(), self .
                     priority(), CEILING, stm32f1xx_hal :: pac ::
                     NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_upper_case_globals)] #[doc(hidden)]
    #[link_section = ".uninit.rtic7"] static mut __rtic_internal_adc1 : core
    :: mem :: MaybeUninit < Adc < ADC1 > > = core :: mem :: MaybeUninit ::
    uninit() ; impl < 'a > rtic :: Mutex for resources :: adc1 < 'a >
    {
        type T = Adc < ADC1 > ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut Adc < ADC1 >) -> RTIC_INTERNAL_R)
        -> RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_adc1 . as_mut_ptr(), self . priority(),
                     CEILING, stm32f1xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_upper_case_globals)] #[doc(hidden)] static mut
    __rtic_internal_pot_dir : bool = false ; impl < 'a > rtic :: Mutex for
    resources :: pot_dir < 'a >
    {
        type T = bool ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut bool) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(& mut __rtic_internal_pot_dir, self . priority(),
                     CEILING, stm32f1xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_upper_case_globals)] #[doc(hidden)] static mut
    __rtic_internal_time_remaining : u16 = 0 ; impl < 'a > rtic :: Mutex for
    resources :: time_remaining < 'a >
    {
        type T = u16 ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut u16) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(& mut __rtic_internal_time_remaining, self . priority(),
                     CEILING, stm32f1xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_upper_case_globals)] #[doc(hidden)] static mut
    __rtic_internal_disp_call_cnt : u8 = 0 ; impl < 'a > rtic :: Mutex for
    resources :: disp_call_cnt < 'a >
    {
        type T = u8 ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut u8) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(& mut __rtic_internal_disp_call_cnt, self . priority(),
                     CEILING, stm32f1xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_upper_case_globals)] #[doc(hidden)] static mut
    __rtic_internal_sys_state : SysState = SysState :: Setup ; impl < 'a >
    rtic :: Mutex for resources :: sys_state < 'a >
    {
        type T = SysState ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut SysState) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(& mut __rtic_internal_sys_state, self . priority(),
                     CEILING, stm32f1xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_snake_case)] #[no_mangle] unsafe fn EXTI9_5()
    {
        const PRIORITY : u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
            {
                crate :: app ::
                handle_buttons(handle_buttons :: Context ::
                               new(& rtic :: export :: Priority ::
                                   new(PRIORITY)))
            }) ;
    } impl < 'a > __rtic_internal_handle_buttonsResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_handle_buttonsResources
            {
                clocks : & * __rtic_internal_clocks . as_ptr(), button_start :
                resources :: button_start :: new(priority), button_brightness
                : resources :: button_brightness :: new(priority), EXTI :
                resources :: EXTI :: new(priority), display : resources ::
                display :: new(priority), brightness_state : resources ::
                brightness_state :: new(priority),
            }
        }
    } #[doc(hidden)] static mut __rtic_internal_handle_adc_FQ : rtic :: export
    :: SCFQ < rtic :: export :: consts :: U1 > = rtic :: export ::
    Queue(unsafe { rtic :: export :: iQueue :: u8_sc() }) ;
    #[link_section = ".uninit.rtic8"] #[doc(hidden)] static mut
    __rtic_internal_handle_adc_MyMono_INSTANTS :
    [core :: mem :: MaybeUninit < rtic :: time :: Instant < DwtSystick <
     8_000_000 > >> ; 1] = [core :: mem :: MaybeUninit :: uninit(),] ;
    #[link_section = ".uninit.rtic9"] #[doc(hidden)] static mut
    __rtic_internal_handle_adc_INPUTS :
    [core :: mem :: MaybeUninit < bool > ; 1] =
    [core :: mem :: MaybeUninit :: uninit(),] ; impl < 'a >
    __rtic_internal_handle_adcResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_handle_adcResources
            {
                pot : resources :: pot :: new(priority), pot_pos : resources
                :: pot_pos :: new(priority), adc1 : resources :: adc1 ::
                new(priority), pot_dir : resources :: pot_dir ::
                new(priority), time_remaining : resources :: time_remaining ::
                new(priority),
            }
        }
    } #[doc(hidden)] static mut __rtic_internal_update_display_FQ : rtic ::
    export :: SCFQ < rtic :: export :: consts :: U2 > = rtic :: export ::
    Queue(unsafe { rtic :: export :: iQueue :: u8_sc() }) ;
    #[link_section = ".uninit.rtic10"] #[doc(hidden)] static mut
    __rtic_internal_update_display_MyMono_INSTANTS :
    [core :: mem :: MaybeUninit < rtic :: time :: Instant < DwtSystick <
     8_000_000 > >> ; 2] =
    [core :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
     uninit(),] ; #[link_section = ".uninit.rtic11"] #[doc(hidden)] static mut
    __rtic_internal_update_display_INPUTS :
    [core :: mem :: MaybeUninit < ScreenPage > ; 2] =
    [core :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
     uninit(),] ; impl < 'a > __rtic_internal_update_displayResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_update_displayResources
            {
                display : resources :: display :: new(priority),
                time_remaining : resources :: time_remaining :: new(priority),
                brightness_state : resources :: brightness_state ::
                new(priority), disp_call_cnt : resources :: disp_call_cnt ::
                new(priority),
            }
        }
    } #[doc(hidden)] static mut __rtic_internal_reset_display_FQ : rtic ::
    export :: SCFQ < rtic :: export :: consts :: U16 > = rtic :: export ::
    Queue(unsafe { rtic :: export :: iQueue :: u8_sc() }) ;
    #[link_section = ".uninit.rtic12"] #[doc(hidden)] static mut
    __rtic_internal_reset_display_MyMono_INSTANTS :
    [core :: mem :: MaybeUninit < rtic :: time :: Instant < DwtSystick <
     8_000_000 > >> ; 10] =
    [core :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
     uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
     MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core ::
     mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(),
     core :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
     uninit(), core :: mem :: MaybeUninit :: uninit(),] ;
    #[link_section = ".uninit.rtic13"] #[doc(hidden)] static mut
    __rtic_internal_reset_display_INPUTS :
    [core :: mem :: MaybeUninit < () > ; 10] =
    [core :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
     uninit(), core :: mem :: MaybeUninit :: uninit(), core :: mem ::
     MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(), core ::
     mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit :: uninit(),
     core :: mem :: MaybeUninit :: uninit(), core :: mem :: MaybeUninit ::
     uninit(), core :: mem :: MaybeUninit :: uninit(),] ; impl < 'a >
    __rtic_internal_reset_displayResources < 'a >
    {
        #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_reset_displayResources
            {
                disp_call_cnt : resources :: disp_call_cnt :: new(priority),
                sys_state : resources :: sys_state :: new(priority),
            }
        }
    } #[allow(non_camel_case_types)] #[derive(Clone, Copy)] #[doc(hidden)] pub
    enum P1_T { handle_adc, reset_display, update_display, } #[doc(hidden)]
    static mut __rtic_internal_P1_RQ : rtic :: export :: SCRQ < P1_T, rtic ::
    export :: consts :: U16 > = rtic :: export ::
    Queue(unsafe { rtic :: export :: iQueue :: u8_sc() }) ;
    #[allow(non_snake_case)]
    #[doc = "Interrupt handler to dispatch tasks at priority 1"] #[no_mangle]
    unsafe fn DMA1_CHANNEL1()
    {
        #[doc = r" The priority of this interrupt handler"] const PRIORITY :
        u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
            {
                while let Some((task, index)) = __rtic_internal_P1_RQ .
                split() . 1 . dequeue()
                {
                    match task
                    {
                        P1_T :: handle_adc =>
                        {
                            let _0 = __rtic_internal_handle_adc_INPUTS .
                            get_unchecked(usize :: from(index)) . as_ptr() .
                            read() ; __rtic_internal_handle_adc_FQ . split() .
                            0 . enqueue_unchecked(index) ; let priority = &
                            rtic :: export :: Priority :: new(PRIORITY) ;
                            crate :: app ::
                            handle_adc(handle_adc :: Context :: new(priority),
                                       _0)
                        } P1_T :: reset_display =>
                        {
                            let() = __rtic_internal_reset_display_INPUTS .
                            get_unchecked(usize :: from(index)) . as_ptr() .
                            read() ; __rtic_internal_reset_display_FQ .
                            split() . 0 . enqueue_unchecked(index) ; let
                            priority = & rtic :: export :: Priority ::
                            new(PRIORITY) ; crate :: app ::
                            reset_display(reset_display :: Context ::
                                          new(priority))
                        } P1_T :: update_display =>
                        {
                            let _0 = __rtic_internal_update_display_INPUTS .
                            get_unchecked(usize :: from(index)) . as_ptr() .
                            read() ; __rtic_internal_update_display_FQ .
                            split() . 0 . enqueue_unchecked(index) ; let
                            priority = & rtic :: export :: Priority ::
                            new(PRIORITY) ; crate :: app ::
                            update_display(update_display :: Context ::
                                           new(priority), _0)
                        }
                    }
                }
            }) ;
    } #[doc(hidden)] #[allow(non_camel_case_types)] static mut
    __rtic_internal_TIMER_QUEUE_MARKER : u32 = 0 ; #[doc(hidden)]
    #[allow(non_camel_case_types)] #[derive(Clone, Copy)] pub enum SCHED_T
    { handle_adc, update_display, reset_display, } #[doc(hidden)] static mut
    __rtic_internal_TQ_MyMono : core :: mem :: MaybeUninit < rtic :: export ::
    TimerQueue < DwtSystick < 8_000_000 >, SCHED_T, rtic :: export :: consts
    :: U13 >> = core :: mem :: MaybeUninit :: uninit() ; #[doc(hidden)] static
    mut __rtic_internal_MONOTONIC_STORAGE_MyMono : Option < DwtSystick <
    8_000_000 > > = None ; #[no_mangle] #[allow(non_snake_case)] unsafe fn
    SysTick()
    {
        while let Some((task, index)) = rtic :: export :: interrupt ::
        free(| _ | if let Some(mono) = crate :: app ::
             __rtic_internal_MONOTONIC_STORAGE_MyMono . as_mut()
             {
                 (& mut * __rtic_internal_TQ_MyMono . as_mut_ptr()) .
                 dequeue(|| core :: mem :: transmute :: < _, cortex_m ::
                         peripheral :: SYST > (()) . disable_interrupt(),
                         mono)
             } else { core :: hint :: unreachable_unchecked() })
        {
            match task
            {
                SCHED_T :: handle_adc =>
                {
                    rtic :: export :: interrupt ::
                    free(| _ | __rtic_internal_P1_RQ . split() . 0 .
                         enqueue_unchecked((P1_T :: handle_adc, index))) ;
                    rtic ::
                    pend(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                         :: interrupt :: DMA1_CHANNEL1) ;
                } SCHED_T :: update_display =>
                {
                    rtic :: export :: interrupt ::
                    free(| _ | __rtic_internal_P1_RQ . split() . 0 .
                         enqueue_unchecked((P1_T :: update_display, index))) ;
                    rtic ::
                    pend(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                         :: interrupt :: DMA1_CHANNEL1) ;
                } SCHED_T :: reset_display =>
                {
                    rtic :: export :: interrupt ::
                    free(| _ | __rtic_internal_P1_RQ . split() . 0 .
                         enqueue_unchecked((P1_T :: reset_display, index))) ;
                    rtic ::
                    pend(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                         :: interrupt :: DMA1_CHANNEL1) ;
                }
            }
        } rtic :: export :: interrupt ::
        free(| _ | if let Some(mono) = crate :: app ::
             __rtic_internal_MONOTONIC_STORAGE_MyMono . as_mut()
             { mono . on_interrupt() ; }) ;
    } #[doc(hidden)] mod rtic_ext
    {
        use super :: * ; #[no_mangle] unsafe extern "C" fn main() -> !
        {
            rtic :: export :: assert_send :: < GraphicsMode < I2CInterface <
            BlockingI2c < I2C1,
            (PB8 < Alternate < OpenDrain > >, PB9 < Alternate < OpenDrain > >)
            > >, DisplaySize128x64 > > () ; rtic :: export :: assert_send :: <
            PB5 < Input < PullUp > > > () ; rtic :: export :: assert_send :: <
            PB6 < Input < PullUp > > > () ; rtic :: export :: assert_send :: <
            stm32f1xx_hal :: pac :: EXTI > () ; rtic :: export :: assert_send
            :: < stm32f1xx_hal :: rcc :: Clocks > () ; rtic :: export ::
            assert_send :: < Adc < ADC1 > > () ; rtic :: export :: assert_send
            :: < PA4 < Analog > > () ; rtic :: export :: assert_send :: < u16
            > () ; rtic :: export :: assert_send :: < bool > () ; rtic ::
            export :: assert_send :: < ScreenPage > () ; rtic :: export ::
            assert_monotonic :: < DwtSystick < 8_000_000 > > () ; rtic ::
            export :: interrupt :: disable() ; (0 .. 1u8) .
            for_each(| i | __rtic_internal_handle_adc_FQ .
                     enqueue_unchecked(i)) ; (0 .. 2u8) .
            for_each(| i | __rtic_internal_update_display_FQ .
                     enqueue_unchecked(i)) ; (0 .. 10u8) .
            for_each(| i | __rtic_internal_reset_display_FQ .
                     enqueue_unchecked(i)) ; let mut core : rtic :: export ::
            Peripherals = rtic :: export :: Peripherals :: steal() . into() ;
            let _ =
            [() ;
             ((1 << stm32f1xx_hal :: pac :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core . NVIC .
            set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                         :: interrupt :: DMA1_CHANNEL1, rtic :: export ::
                         logical2hw(1u8, stm32f1xx_hal :: pac ::
                                    NVIC_PRIO_BITS),) ; rtic :: export :: NVIC
            ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: DMA1_CHANNEL1) ; let _ =
            [() ;
             ((1 << stm32f1xx_hal :: pac :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core . NVIC .
            set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                         :: interrupt :: EXTI9_5, rtic :: export ::
                         logical2hw(1u8, stm32f1xx_hal :: pac ::
                                    NVIC_PRIO_BITS),) ; rtic :: export :: NVIC
            ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
                   :: interrupt :: EXTI9_5) ; __rtic_internal_TQ_MyMono .
            as_mut_ptr() . write(rtic :: export :: TimerQueue :: new()) ; let
            _ =
            [() ;
             ((1 << stm32f1xx_hal :: pac :: NVIC_PRIO_BITS) - 1u8 as usize)] ;
            core . SCB .
            set_priority(rtic :: export :: SystemHandler :: SysTick, rtic ::
                         export ::
                         logical2hw(1u8, stm32f1xx_hal :: pac ::
                                    NVIC_PRIO_BITS),) ; if ! < DwtSystick <
            8_000_000 > as rtic :: Monotonic > ::
            DISABLE_INTERRUPT_ON_EMPTY_QUEUE
            {
                core :: mem :: transmute :: < _, cortex_m :: peripheral ::
                SYST > (()) . enable_interrupt() ;
            } let(late, mut monotonics) = crate :: app ::
            init(init :: Context :: new(core . into())) ; __rtic_internal_EXTI
            . as_mut_ptr() . write(late . EXTI) ; __rtic_internal_adc1 .
            as_mut_ptr() . write(late . adc1) ;
            __rtic_internal_button_brightness . as_mut_ptr() .
            write(late . button_brightness) ; __rtic_internal_button_start .
            as_mut_ptr() . write(late . button_start) ; __rtic_internal_clocks
            . as_mut_ptr() . write(late . clocks) ; __rtic_internal_display .
            as_mut_ptr() . write(late . display) ; __rtic_internal_pot .
            as_mut_ptr() . write(late . pot) ; __rtic_internal_pot_pos .
            as_mut_ptr() . write(late . pot_pos) ; monotonics . 0 . reset() ;
            __rtic_internal_MONOTONIC_STORAGE_MyMono = Some(monotonics . 0) ;
            rtic :: export :: interrupt :: enable() ; crate :: app ::
            idle(idle :: Context ::
                 new(& rtic :: export :: Priority :: new(0)))
        }
    }
}