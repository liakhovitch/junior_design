#![allow(non_snake_case)]

use crate::app;
use crate::app::*;

use rtic::Mutex;

use stm32f1xx_hal::{
    prelude::*,
    gpio::{
        gpiob::{PB6, PB5},
        {Input, PullUp},
        ExtiPin,
    },
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
use profont::ProFont24Point;

use embedded_hal::digital::v2::InputPin;

pub fn handle_buttons(cx: app::handle_buttons::Context){
    // Bring resources into scope
    let (mut button_start, mut button_brightness) =
        (cx.resources.button_start, cx.resources.button_brightness);
    let (mut EXTI, mut display) =
    (cx.resources.EXTI, cx.resources.display);
    let mut brightness_state = cx.resources.brightness_state;
    //let clocks = cx.resources.clocks;

    // Clear interrupt bits and disable interrupts
    EXTI.lock(|EXTI| {
        button_start.lock(|button_start| {
            button_brightness.lock(|button_brightness| {
                button_start.disable_interrupt(&EXTI);
                button_brightness.disable_interrupt(&EXTI);
                button_start.clear_interrupt_pending_bit();
                button_brightness.clear_interrupt_pending_bit();
            })
        })
    });

    // Check button state
    let mut button_start_pressed:bool = false;
    let mut button_brightness_pressed:bool = false;
    button_start.lock(|button_start| {
        button_brightness.lock(|button_brightness| {
            button_start_pressed = button_start.is_low().unwrap();
            button_brightness_pressed = button_brightness.is_low().unwrap();
        })
    });

    // Handle button presses
    if button_brightness_pressed == true || button_start_pressed == true {
        display.lock(|display| {
            display.clear();
            if button_start_pressed == true{
                Text::new("Start!", Point::new(20,16))
                    .into_styled(TextStyle::new(ProFont24Point, BinaryColor::On))
                    .draw(display)
                    .unwrap();
            } else if button_brightness_pressed == true {
                let mut brightness:Brightness = Brightness::DIM;
                let mut disp_str: &'static str = "";
                brightness_state.lock(|brightness_state|{
                    *brightness_state = (*brightness_state+1)%3;
                    match brightness_state {
                        0 => {brightness = Brightness::DIM; disp_str = "Dim"},
                        1 => {brightness = Brightness::NORMAL; disp_str = "Med"},
                        _ => {brightness = Brightness::BRIGHTEST; disp_str = "Bright"},
                    }
                });
                display.set_brightness(brightness).unwrap();
                Text::new(disp_str, Point::new(20,16))
                    .into_styled(TextStyle::new(ProFont24Point, BinaryColor::On))
                    .draw(display)
                    .unwrap();
            }
            display.flush().unwrap();
        });
    }

    // Enable interrupts
    EXTI.lock(|EXTI| {
        button_start.lock(|button_start| {
            button_brightness.lock(|button_brightness| {
                button_start.enable_interrupt(&EXTI);
                button_brightness.enable_interrupt(&EXTI);
            })
        })
    });
}