use crate::app;
use crate::app::*;
use crate::types::{ScreenPage, SysState};

use rtic::Mutex;

use stm32f1xx_hal::{
    prelude::*,
    rtc::Rtc,
};

pub fn tick(cx: tick::Context) {
    let (mut rtc, mut sys_state, mut max_time) =
        (cx.resources.rtc, cx.resources.sys_state, cx.resources.max_time);
    let mut disp_call_cnt = cx.resources.disp_call_cnt;
    rtc.lock(|rtc|{rtc.clear_second_flag()});
    sys_state.lock(|sys_state|{
        match sys_state {
            SysState::Setup => {
                // This should never happen
            }
            SysState::Timer => {
                let cnt:u8 = disp_call_cnt.lock(|disp_call_cnt|{
                    return *disp_call_cnt;
                });
                if cnt == 0 {
                    let _ = update_display::spawn(ScreenPage::Timer);
                }
            }
            SysState::Sleep => {
                // This should never happen
            }
        }
    });
}

pub fn alarm(cx: alarm::Context) {
    let (mut rtc, mut sys_state, mut max_time) =
        (cx.resources.rtc, cx.resources.sys_state, cx.resources.max_time);
    rtc.lock(|rtc|{
        rtc.clear_alarm_flag();
        rtc.clear_second_flag();
    });
    sys_state.lock(|sys_state|{
        return;
    });
}

pub fn kick_dog(cx: kick_dog::Context) {
    let (mut rtc, mut sys_state) =
        (cx.resources.rtc, cx.resources.sys_state);
    sys_state.lock(|sys_state|{
        match sys_state {
            SysState::Setup => {
                rtc.lock(|rtc|{
                    // Put off sleep timer when user interacts with device
                    rtc.set_time(0);
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