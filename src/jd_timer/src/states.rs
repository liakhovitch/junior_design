use crate::app;
use crate::app::*;
use crate::types::{ScreenPage, SysState};
use crate::config::{SLEEP_TIME};
use crate::rtc_util;

use rtic::Mutex;

use stm32f1xx_hal::{
    prelude::*,
    gpio::{
        gpioa::{PA9},
        {Output, PushPull},
    },
};

use embedded_hal::digital::v2::OutputPin;

// This task is spawned by other parts of the program. It does some housekeeping to move the
//   system to a specific state.
pub fn to_state(cx: to_state::Context, target: SysState){
    // Bring resources into scope
    let (mut sys_state, mut sleep_pin) =
        (cx.resources.sys_state, cx.resources.sleep_pin);
    let mut max_time = cx.resources.max_time;
    let mut disp_call_cnt = cx.resources.disp_call_cnt;
    let mut rtc = cx.resources.rtc;
    // Acquire display status message status
    let cnt:u8 = disp_call_cnt.lock(|disp_call_cnt|{
        return *disp_call_cnt;
    });
    // Acquire resource locks
    rtc.lock(|rtc|{
    sys_state.lock(|sys_state|{
    sleep_pin.lock(|sleep_pin|{
        match target {
            SysState::Setup => {
                // Set new system state
                *sys_state = SysState::Setup;
                // Reset RTC
                rtc_util::set_time(rtc, 0);
                // Configure RTC for use as sleep timer
                rtc_util::listen_seconds(rtc);
                rtc_util::clear_second_flag(rtc);
                // Make sure we aren't shutting off
                sleep_pin.set_high().unwrap();
                // Update the display, unless there's a status message being shown
                if cnt == 0 {
                    let _ = update_display::spawn(ScreenPage::Setup);
                }
            }
            SysState::Timer => {
                // Set new system state
                *sys_state = SysState::Timer;
                // Reset RTC
                rtc_util::set_time(rtc,0);
                max_time.lock(|max_time| {
                    // Set RTC alarm to trigger when timer runs out
                    rtc_util::set_alarm(rtc,*max_time as u32);
                });
                // Configure RTC for use as egg timer
                rtc_util::listen_seconds(rtc);
                rtc_util::clear_second_flag(rtc);
                // Make sure we aren't shutting off
                sleep_pin.set_high().unwrap();
                // Update the display
                let _ = update_display::spawn(ScreenPage::Timer);
            }
            SysState::Sleep => {
                // Update system state
                *sys_state = SysState::Sleep;
                // Shut off RTC alarms
                rtc_util::unlisten_seconds(rtc);
                rtc_util::clear_second_flag(rtc);
                // Tell PMIC to shut us off
                sleep_pin.set_low().unwrap();
                // Show sleep message on display
                if cnt == 0 {
                    let _ = update_display::spawn(ScreenPage::Sleep);
                }
            }
        }
    });});});
}