use crate::app;
use crate::app::*;
use crate::types::SysState;

use rtic::Mutex;

use stm32f1xx_hal::{
    prelude::*,
    rtc::Rtc,
};

pub fn tick(cx: tick::Context) {
    let (mut rtc, mut sys_state, mut max_time) =
        (cx.resources.rtc, cx.resources.sys_state, cx.resources.max_time);
    rtc.lock(|rtc|{rtc.clear_second_flag()});
    sys_state.lock(|sys_state|{
        return;
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