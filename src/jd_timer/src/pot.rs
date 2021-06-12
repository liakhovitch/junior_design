use crate::app;
use crate::app::*;
use crate::types::ScreenPage;
use crate::config::{MAX_NUM, NUM_STEPS};

use rtic::Mutex;

use stm32f1xx_hal::{
    prelude::*,
    gpio::{
        gpioa::{PA4},
        {Input, PullUp},
        {Alternate, OpenDrain},
    },
    timer::{Event, Timer},
    pac::{ADC1},
};

use ssd1306::{
    prelude::*,
    brightness::Brightness,
};

use embedded_graphics::{
    fonts::Text,
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyle,
};
use core::fmt::Write;
use heapless::String;
use heapless::consts::*;
use profont::ProFont24Point;
use profont::ProFont14Point;

use embedded_hal::digital::v2::InputPin;



pub fn handle_adc(cx: app::handle_adc::Context, silent:bool){
    // Bring resources into scope
    let (mut pot, mut adc1) =
        (cx.resources.pot, cx.resources.adc1);
    let mut pot_pos = cx.resources.pot_pos;
    let mut pot_dir = cx.resources.pot_dir;
    let mut max_num = cx.resources.max_num;

    let mut pot_pos_new:u16 = 0;
    let mut sample_sum:u16 = 0;
    let mut middle_sum:u16 = 0;
    let mut outer_sum:u16 = 0;

    // Read ADC. This is a quick and dirty averaging algorithm, will improve if I have time.
    pot.lock(|pot| {
        adc1.lock(|adc1| {
            for _i in 0..4 {
                middle_sum = 0;
                for _p in 0..4 {
                    sample_sum = 0;
                    for _n in 0..4 {
                        pot_pos_new = adc1.read(pot).unwrap();
                        pot_pos_new = pot_pos_new >> 4;
                        sample_sum += pot_pos_new;
                    }
                    middle_sum += sample_sum >> 2;
                }
                outer_sum += middle_sum >> 2;
            }
            pot_pos_new = outer_sum >> 2;
        })
    });

    // Another awful algorithm to prevent jitter.
    // If pot turn has changed direction, will not update pot_pos until pot has moved 2 positions.
    pot_pos.lock(|pot_pos| {
        pot_dir.lock(|pot_dir|{
            // Just pretend this code doesn't exist, you will sleep more soundly.
            let mut pot_changed:bool = false;
            if (*pot_dir == true) && (pot_pos_new > *pot_pos || pot_pos_new < *pot_pos-1){
                pot_changed = true;
            } else if (*pot_dir == false) && (pot_pos_new > *pot_pos+1 || pot_pos_new < *pot_pos){
                pot_changed = true;
            }
            // Handle a registered change in pot position
            if pot_changed == true || silent == true {
                if pot_pos_new > *pot_pos {*pot_dir = true}
                else {*pot_dir = false}
                // Convert old pot position into time
                let max_num_old:u16 = (((MAX_NUM / NUM_STEPS)*(255_u16-*pot_pos))/255)* NUM_STEPS;
                // Update pot position
                *pot_pos = pot_pos_new;
                // Convert new pot position into time
                let max_num_new:u16 = (((MAX_NUM / NUM_STEPS)*(255_u16-*pot_pos))/255)* NUM_STEPS;
                // Only update the display if the pot has moved enough to change the time
                if max_num_old != max_num_new {
                    // Update the time remaining on the clock
                    max_num.lock(|max_num|{
                        *max_num = max_num_new;
                    });
                    // Kick the dog
                    let _ = kick_dog::spawn();
                    // Update the display with the new time
                    if silent == false {
                        let _ = update_display::spawn(ScreenPage::Setup);
                    }
                }

            }
        })
    });

}