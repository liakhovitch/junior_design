use crate::app;
use crate::app::*;
use crate::types::{ScreenPage, SysState};
use crate::rtc_util;
use crate::config::SLEEP_TIME;

use rtic::Mutex;

use stm32f1xx_hal::{
    prelude::*,
    rtc::Rtc,
};

use embedded_hal::digital::v2::InputPin;

use cortex_m::asm::delay;

// tick: triggered via interrupt every time the RTC counts down one second
pub fn tick(cx: tick::Context) {
    // Bring resources into scope
    let mut sys_state = cx.resources.sys_state;
    let mut rtc = cx.resources.rtc;
    let mut chg_pin = cx.resources.chg_pin;
    // Tiny arbitrary delay to ensure the RTC counter is updated.
    // For some reason the tick interrupt gets triggered just before the counter is updated.
    // It's in the manual but still odd.
    delay(100);
    sys_state.lock(|sys_state|{
        match sys_state {
            SysState::Setup => {
                // Get time since last user interaction
                let current_time:u16 = rtc.lock(|rtc|{
                    return rtc_util::current_time(rtc) as u16;
                });
                let chg_state:bool = chg_pin.lock(|chg_pin|{
                    return chg_pin.is_low().unwrap();
                });

                // Is it time to fall asleep?
                if current_time >= SLEEP_TIME {
                    // Is the device charging right now?
                    if chg_state == true {
                        // If charging, show icon fullscreen and reset sleep timer
                        rtc.lock(|rtc|{
                            rtc_util::set_time(rtc,0);
                        });
                        let _ = update_display::spawn(ScreenPage::Charging);

                    } else {
                        // If not charging, got to sleep
                        let _ = to_state::spawn(SysState::Sleep);
                    }
                }
            }
            SysState::Sleep => {
                // We dont really care what time it is when we're sleeping
            }
        }
    });
    rtc.lock(|rtc|{rtc_util::clear_second_flag(rtc)});
}

// kick_dog: spawned by other tasks when user interacts with the main menu.
//   Resets the rtc counter delay auto-sleep.
pub fn kick_dog(cx: kick_dog::Context) {
    let mut sys_state =
        cx.resources.sys_state;
    let mut rtc = cx.resources.rtc;
    sys_state.lock(|sys_state|{
        match sys_state {
            SysState::Setup => {
                rtc.lock(|rtc|{
                    // Put off sleep timer when user interacts with device
                    rtc_util::set_time(rtc,0);
                });
            }
            SysState::Sleep => {
                // If the user moves some input while the system is going to sleep,
                //   cancel sleep.
                let _ = to_state::spawn(SysState::Setup);
            }
        }
    });
}