use crate::app;
use crate::app::*;

use rtic::Mutex;

use embedded_hal::PwmPin;

use rtic::time::duration::Milliseconds;

// Play a beep the specified number of times. Each beep is [length] ms long.
pub fn beep(cx: beep::Context, length: u32, count: u8) {
    let mut buzzer = cx.resources.buzzer;
    // Check for end of "recursive algorithm"
    if count == 0 {return};
    // Turn on the buzzer
    buzzer.lock(|buzzer|{
        buzzer.enable();
    });
    // Schedule turning off the buzzer for later, so the program can go back to other stuff.
    let _ = unbeep::spawn_after(Milliseconds(length), length, count-1);
}

pub fn unbeep(cx: unbeep::Context, length: u32, count: u8) {
    let mut buzzer = cx.resources.buzzer;
    buzzer.lock(|buzzer|{
        buzzer.disable();
    });
    if count != 0 {let _ = beep::spawn_after(Milliseconds(length), length, count);};
}