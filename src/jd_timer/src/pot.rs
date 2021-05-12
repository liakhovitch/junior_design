use crate::app;
use crate::app::*;

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

pub fn handle_adc(cx: app::handle_adc::Context){
    // Bring resources into scope
    let (mut display, mut pot, mut adc1) =
        (cx.resources.display, cx.resources.pot, cx.resources.adc1);
    let mut pot_pos = cx.resources.pot_pos;
    let mut pot_dir = cx.resources.pot_dir;
    //let clocks = cx.resources.clocks;

    let mut pot_pos_new:u16 = 0;
    let mut sample_sum:u16 = 0;
    let mut middle_sum:u16 = 0;
    let mut outer_sum:u16 = 0;

    // Read ADC
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

    pot_pos.lock(|pot_pos| {
        pot_dir.lock(|pot_dir|{
            let mut pot_changed:bool = false;
            if (*pot_dir == true) && (pot_pos_new > *pot_pos || pot_pos_new < *pot_pos-1){
                pot_changed = true;
            } else if (*pot_dir == false) && (pot_pos_new > *pot_pos+1 || pot_pos_new < *pot_pos){
                pot_changed = true;
            }
            if pot_changed == true {
                *pot_pos = pot_pos_new;
                display.lock(|display| {
                    display.clear();
                    let mut data = String::<U16>::from("Pot:");
                    let _=write!(data,"{}", *pot_pos);
                    Text::new(&data[..], Point::new(20,16))
                        .into_styled(TextStyle::new(ProFont14Point, BinaryColor::On))
                        .draw(display)
                        .unwrap();
                    display.flush().unwrap();
                });
            }
        })
    });

}