#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
      r" Always include the device crate which contains the vector table"] use
    stm32f1xx_hal :: pac as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ; use
    stm32f1xx_hal ::
    {
        prelude :: *, serial, gpio ::
        {
            gpiob :: { PB8, PB9, PB6, PB5 }, gpioa ::
            { PA0, PA1, PA4, PA9, PA10 }, { Output, PushPull },
            { Input, PullUp }, { Alternate, OpenDrain },
        }, timer :: { Event, Timer }, pac :: { I2C1, USART1 }, i2c ::
        { BlockingI2c, DutyCycle, Mode },
    } ; use cortex_m :: asm :: delay ; use ssd1306 ::
    { prelude :: *, Builder, I2CDIBuilder, } ; use embedded_hal :: digital ::
    v2 :: { OutputPin, InputPin } ; use core :: ptr :: write_volatile ; use
    core :: fmt :: Write ; use core :: future :: Future ;
    #[doc = r" User code from within the module"] #[doc = r" User code end"]
    #[allow(non_snake_case)] fn init(cx : init :: Context) ->
    (init :: LateResources, init :: Monotonics)
    {
        let mut core = cx . core ; core . DWT . enable_cycle_counter() ; let
        mut rcc = cx . device . RCC . constrain() ; let mut flash = cx .
        device . FLASH . constrain() ; let mut afio = cx . device . AFIO .
        constrain(& mut rcc . apb2) ; let clocks = rcc . cfgr .
        use_hse(8 . mhz()) . sysclk(72 . mhz()) . pclk1(36 . mhz()) .
        pclk1(36 . mhz()) . freeze(& mut flash . acr) ; let mut gpioa = cx .
        device . GPIOA . split(& mut rcc . apb2) ; let mut gpiob = cx . device
        . GPIOB . split(& mut rcc . apb2) ; let button_start = gpiob . pb5 .
        into_pull_up_input(& mut gpiob . crl) ; let button_brightness = gpiob
        . pb6 . into_pull_up_input(& mut gpiob . crl) ; let tx1_pin = gpioa .
        pa9 . into_alternate_push_pull(& mut gpioa . crh) ; let rx1_pin =
        gpioa . pa10 . into_floating_input(& mut gpioa . crh) ; let cfg =
        serial :: Config :: default() . baudrate(115_200 . bps()) ; let usart1
        = serial :: Serial ::
        usart1(cx . device . USART1, (tx1_pin, rx1_pin), & mut afio . mapr,
               cfg, clocks, & mut rcc . apb2,) ; let(tx, rx) = usart1 .
        split() ; let scl = gpiob . pb8 .
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
        ; display . init() . unwrap() ; display . clear() ;
        (init :: LateResources
         {
             display, buttons : (button_start, button_brightness), EXTI : cx .
             device . EXTI, clocks, serial : (tx, rx),
         }, init :: Monotonics())
    } #[allow(non_snake_case)] #[doc = "Initialization function"] pub mod init
    {
        #[allow(unused_imports)] use stm32f1xx_hal ::
        {
            prelude :: *, serial, gpio ::
            {
                gpiob :: { PB8, PB9, PB6, PB5 }, gpioa ::
                { PA0, PA1, PA4, PA9, PA10 }, { Output, PushPull },
                { Input, PullUp }, { Alternate, OpenDrain },
            }, timer :: { Event, Timer }, pac :: { I2C1, USART1 }, i2c ::
            { BlockingI2c, DutyCycle, Mode },
        } ; #[allow(unused_imports)] use cortex_m :: asm :: delay ;
        #[allow(unused_imports)] use ssd1306 ::
        { prelude :: *, Builder, I2CDIBuilder, } ; #[allow(unused_imports)]
        use embedded_hal :: digital :: v2 :: { OutputPin, InputPin } ;
        #[allow(unused_imports)] use core :: ptr :: write_volatile ;
        #[allow(unused_imports)] use core :: fmt :: Write ;
        #[allow(unused_imports)] use core :: future :: Future ;
        #[doc = r" Resources initialized at runtime"] #[allow(non_snake_case)]
        pub struct LateResources
        {
            pub EXTI : stm32f1xx_hal :: pac :: EXTI, pub buttons :
            (PB5 < Input < PullUp > >, PB6 < Input < PullUp > >), pub clocks :
            stm32f1xx_hal :: rcc :: Clocks, pub display : GraphicsMode <
            I2CInterface < BlockingI2c < I2C1,
            (PB8 < Alternate < OpenDrain > >, PB9 < Alternate < OpenDrain > >)
            > >, DisplaySize128x64 >, pub serial :
            (serial :: Tx < USART1 >, serial :: Rx < USART1 >)
        } #[doc = r" Monotonics used by the system"] #[allow(non_snake_case)]
        pub struct Monotonics() ; #[doc = r" Execution context"] pub struct
        Context < 'a >
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
    } #[doc = r" app module"] #[doc(hidden)] mod rtic_ext
    {
        use super :: * ; #[no_mangle] unsafe extern "C" fn main() -> !
        {
            rtic :: export :: interrupt :: disable() ; let mut core : rtic ::
            export :: Peripherals = rtic :: export :: Peripherals :: steal() .
            into() ; core . SCB . scr . modify(| r | r | 1 << 1) ;
            let(late, mut monotonics) = crate :: app ::
            init(init :: Context :: new(core . into())) ; rtic :: export ::
            interrupt :: enable() ; loop { rtic :: export :: wfi() }
        }
    }
}