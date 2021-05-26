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

use cortex_m::asm::delay;

// tick: triggered via interrupt every time the RTC counts down one second
pub fn tick(cx: tick::Context) {
    // Bring resources into scope
    let (mut sys_state, mut max_time) =
        (cx.resources.sys_state, cx.resources.max_time);
    let mut disp_call_cnt = cx.resources.disp_call_cnt;
    let mut rtc = cx.resources.rtc;
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
                // Is it time to fall asleep?
                if current_time >= SLEEP_TIME {
                    let _ = to_state::spawn(SysState::Sleep);
                }
            }
            SysState::Timer => {
                // Get current time
                let current_time:u16 = rtc.lock(|rtc|{
                    return rtc_util::current_time(rtc) as u16;
                });
                // Get maximum time (what the timer was set to)
                let maximum_time:u16 = max_time.lock(|max_time|{return *max_time});
                // Is the timer down to 0?
                if current_time < maximum_time {
                    // If timer is still running, update the display but only if nothing more
                    //   important is currently showing
                    let cnt:u8 = disp_call_cnt.lock(|disp_call_cnt|{
                        return *disp_call_cnt;
                    });
                    if cnt == 0 {
                        let _ = update_display::spawn(ScreenPage::Timer);
                    }
                } else {
                    // If timer is down to zero, trigger alarm and return to main menu
                    let _ = update_display::spawn(ScreenPage::Alarm);
                    let _ = beep::spawn(500, 5);
                    let _ = to_state::spawn(SysState::Setup);
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
            SysState::Timer => {
                // Nothing to do
            }
            SysState::Sleep => {
                // If the user moves some input while the system is going to sleep,
                //   cancel sleep.
                let _ = to_state::spawn(SysState::Setup);
            }
        }
    });
}