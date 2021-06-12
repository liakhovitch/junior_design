//--------
//RTC Util
//--------
// IMPORTANT!!!
// Most of this code was copied from the HAL's RTC object at:
// https://github.com/stm32-rs/stm32f1xx-hal/blob/master/src/rtc.rs
// and then modified to work more like a C-style library than an OOP object.

use stm32f1xx_hal::{
    pac::RTC
};

// This implements the RTC config register write algorithm found on page 485 of the manual.
// The function is a modified version of the one from the stm32f1xx_hal.
pub fn rtc_write(rtc: &mut RTC , func: impl Fn(&mut RTC)) {
    // Wait for the last write operation to be done
    while !(*rtc).crl.read().rtoff().bit() {}
    // Put the clock into config mode
    (*rtc).crl.modify(|_, w| w.cnf().set_bit());

    // Perform the write operation
    func(rtc);

    // Take the device out of config mode
    (*rtc).crl.modify(|_, w| w.cnf().clear_bit());
    // Wait for the write to be done
    while !(*rtc).crl.read().rtoff().bit() {}
}

// Set the current RTC counter value to the specified amount
pub fn set_time(rtc: &mut RTC, counter_value: u32) {
    rtc_write(rtc, |rtc| {
        (*rtc)
            .cnth
            .write(|w| unsafe { w.bits(counter_value >> 16) });
        (*rtc)
            .cntl
            .write(|w| unsafe { w.bits(counter_value as u16 as u32) });
    });
}

/*
  Sets the time at which an alarm will be triggered
  This also clears the alarm flag if it is set
*/
#[allow(dead_code)]
pub fn set_alarm(rtc: &mut RTC, counter_value: u32) {
    // Set alarm time
    // See section 18.3.5 for explanation
    let alarm_value = counter_value - 1;

        rtc_write(rtc, |rtc| {
        (*rtc)
            .alrh
            .write(|w|  w.alrh().bits((alarm_value >> 16) as u16) );
        (*rtc)
            .alrl
            .write(|w|  w.alrl().bits(alarm_value as u16) );
    });

    clear_alarm_flag(rtc);
}

// Enables the RTC interrupt to trigger when the counter reaches the alarm value. In addition,
// if the EXTI controller has been set up correctly, this function also enables the RTCALARM
// interrupt.
/*pub fn listen_alarm(rtc: &mut RTC) {
    // Enable alarm interrupt
    rtc_write(rtc, |rtc| {
        (*rtc).crh.modify(|_, w| w.alrie().set_bit());
    })
}*/

// Stops the RTC alarm from triggering the RTC and RTCALARM interrupts
pub fn unlisten_alarm(rtc: &mut RTC) {
    // Disable alarm interrupt
    rtc_write(rtc, |rtc| {
        (*rtc).crh.modify(|_, w| w.alrie().clear_bit());
    })
}

// Reads the current counter
pub fn current_time(rtc: &mut RTC) -> u32 {
    // Wait for the APB1 interface to be ready
    //rtc_write(rtc, |rtc| (*rtc).crl.modify(|_, w| w.rsf().clear_bit()));
    //while (*rtc).crl.read().rsf().bit() {}

    (*rtc).cnth.read().bits() << 16 | (*rtc).cntl.read().bits()
}

// Enables triggering the RTC interrupt every time the RTC counter is increased
pub fn listen_seconds(rtc: &mut RTC) {
    rtc_write(rtc, |rtc| (*rtc).crh.modify(|_, w| w.secie().set_bit()))
}

// Disables the RTC second interrupt
pub fn unlisten_seconds(rtc: &mut RTC) {
    rtc_write(rtc, |rtc| (*rtc).crh.modify(|_, w| w.secie().clear_bit()))
}

// Clears the RTC second interrupt flag
pub fn clear_second_flag(rtc: &mut RTC) {
    rtc_write(rtc, |rtc| (*rtc).crl.modify(|_, w| w.secf().clear_bit()))
}

// Clears the RTC alarm interrupt flag
pub fn clear_alarm_flag(rtc: &mut RTC) {
    rtc_write(rtc, |rtc| (*rtc).crl.modify(|_, w| w.alrf().clear_bit()))
}