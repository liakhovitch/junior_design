use crate::app;
use crate::app::*;

use rtic::Mutex;

use embedded_hal::PwmPin;

use rtic::time::duration::Milliseconds;

pub fn beep(cx: beep::Context, length: u32, count: u8) {
    let mut buzzer = cx.resources.buzzer;
    if count == 0 {return};
    buzzer.lock(|buzzer|{
        buzzer.enable();
    });
    let _ = unbeep::spawn_after(Milliseconds(length), length, count-1);
}

pub fn unbeep(cx: unbeep::Context, length: u32, count: u8) {
    let mut buzzer = cx.resources.buzzer;
    buzzer.lock(|buzzer|{
        buzzer.disable();
    });
    if count != 0 {let _ = beep::spawn_after(Milliseconds(length), length, count);};
}