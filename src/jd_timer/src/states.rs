use crate::app;
use crate::app::*;
use crate::types::SysState;
use crate::config::{SLEEP_TIME};

use rtic::Mutex;

use stm32f1xx_hal::{
    prelude::*,
    gpio::{
        gpioa::{PA9},
        {Output, PushPull},
    },
};

use embedded_hal::digital::v2::OutputPin;


pub fn to_state(cx: to_state::Context, target: SysState){
    let (mut rtc, mut sys_state, mut sleep_pin) =
        (cx.resources.rtc, cx.resources.sys_state, cx.resources.sleep_pin);
    let mut max_time = cx.resources.max_time;
    rtc.lock(|rtc|{
    sys_state.lock(|sys_state|{
    sleep_pin.lock(|sleep_pin|{
        match target {
            SysState::Setup => {
                *sys_state = SysState::Setup;
                rtc.set_time(0);
                rtc.set_alarm(SLEEP_TIME as u32);
                rtc.listen_alarm();
                rtc.unlisten_seconds();
                rtc.clear_alarm_flag();
                rtc.clear_second_flag();
                sleep_pin.set_high().unwrap();
            }
            SysState::Timer => {
                *sys_state = SysState::Timer;
                rtc.set_time(0);
                max_time.lock(|max_time| {
                    rtc.set_alarm(*max_time as u32);
                });
                rtc.listen_alarm();
                rtc.listen_seconds();
                rtc.clear_alarm_flag();
                rtc.clear_second_flag();
                sleep_pin.set_high().unwrap();
            }
            SysState::Sleep => {
                *sys_state = SysState::Sleep;
                rtc.unlisten_alarm();
                rtc.unlisten_seconds();
                rtc.clear_alarm_flag();
                rtc.clear_second_flag();
                sleep_pin.set_low().unwrap();
            }
        }
    });});});
}