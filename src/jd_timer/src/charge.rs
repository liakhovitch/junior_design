use crate::app;
use crate::app::*;

use rtic::Mutex;

use stm32f1xx_hal::gpio::ExtiPin;

pub fn handle_charge(cx: handle_charge::Context){
    let (mut chg_pin, mut disp_call_cnt) =
        (cx.resources.chg_pin, cx.resources.disp_call_cnt);

    // Ignore any pending notification messages on screen
    disp_call_cnt.lock(|disp_call_cnt|{
        *disp_call_cnt = 0;
    });

    // Kick the dog
    let _ = kick_dog::spawn();

    // Refresh the display
    let _ = reset_display::spawn();

    // Clear interrupt flag
    chg_pin.lock(|chg_pin| {
        chg_pin.clear_interrupt_pending_bit();
    });

}