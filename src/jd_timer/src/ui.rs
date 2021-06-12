use crate::app;
use crate::app::*;
use crate::types::{ScreenPage, SysState};
use crate::logo::LOGO;
use crate::bigbolt::BIGBOLT;
use crate::smallbolt::SMALLBOLT;
use crate::rtc_util;

use rtic::Mutex;

use ssd1306::{
    prelude::*,
    brightness::Brightness,
};

use embedded_graphics::{
    fonts::Text,
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyle,
    image::{Image, ImageRaw},
};

use embedded_hal::digital::v2::InputPin;

use core::fmt::Write;
use heapless::String;
use heapless::consts::*;
use profont::ProFont24Point;
use profont::ProFont14Point;
use rtic::time::duration::Seconds;

use oorandom;

pub fn update_display(cx: update_display::Context, screen_type:ScreenPage){
    // Bring resources into scope
    let (mut display, mut brightness_state) =
        (cx.resources.display, cx.resources.brightness_state);
    let mut max_num = cx.resources.max_num;
    let mut disp_call_cnt = cx.resources.disp_call_cnt;
    let mut chg_pin = cx.resources.chg_pin;
    let mut rng = cx.resources.rng;

    display.lock(|display| {
        // Wipe the slate
        display.clear();
        let chg_state:bool = chg_pin.lock(|chg_pin|{
            return chg_pin.is_low().unwrap();
        });
        // Are we charging?
        if chg_state == true {
            // If we're charging, show little bolt icon in the corner
            let raw_image: ImageRaw<BinaryColor> = ImageRaw::new(SMALLBOLT, 14, 14);
            Image::new(&raw_image, Point::new(114,0))
                .draw(display)
                .unwrap();
        }
        match screen_type {
            // Display the time set screen
            ScreenPage::Setup => {
                disp_call_cnt.lock(|disp_call_cnt|{*disp_call_cnt = 0});
                max_num.lock(|max_num| {
                    // Format the text
                    let mut data = String::<U16>::from("");
                    //let minutes = *max_num /60;
                    //let seconds = *max_num %60;
                    //let _ = write!(data, "{:>2}:{:>02}", minutes, seconds);
                    let _ = write!(data, "{}", (*max_num+1));

                    // Create the graphics object and draw it on the "buffer"
                    Text::new(&data[..], Point::new(48,16))
                        .into_styled(TextStyle::new(ProFont24Point, BinaryColor::On))
                        .draw(display)
                        .unwrap();

                    // Render constant status message
                    Text::new("Max number:", Point::new(10,0))
                        .into_styled(TextStyle::new(ProFont14Point, BinaryColor::On))
                        .draw(display)
                        .unwrap();

                    // Write buffer to display
                    display.flush().unwrap();
                });
            },
            ScreenPage::Brightness => {
                display.clear();
                let mut disp_str: &'static str = "";
                brightness_state.lock(|brightness_state|{
                    match brightness_state {
                        0 => {disp_str = "Dim"},
                        1 => {disp_str = "Med"},
                        _ => {disp_str = "Bright"},
                    }
                });
                Text::new(disp_str, Point::new(20,16))
                    .into_styled(TextStyle::new(ProFont24Point, BinaryColor::On))
                    .draw(display)
                    .unwrap();
                Text::new("Brightness:", Point::new(10,0))
                    .into_styled(TextStyle::new(ProFont14Point, BinaryColor::On))
                    .draw(display)
                    .unwrap();
                display.flush().unwrap();
                disp_call_cnt.lock(|disp_call_cnt|{*disp_call_cnt += 1});
                let _ = reset_display::spawn_after(Seconds(2_u32));
            },
            ScreenPage::Number => {

                rng.lock(|rng| {
                max_num.lock(|max_num| {
                    // Format the text
                    let mut data = String::<U16>::from("");
                    let mut num:u32 = 0;
                    if *max_num > 0 {
                        num = rng.rand_u32() % (*max_num as u32 + 1);
                    }
                    let _ = write!(data, "{}", (num + 1));

                    // Create the graphics object and draw it on the "buffer"
                    Text::new(&data[..], Point::new(48, 16))
                        .into_styled(TextStyle::new(ProFont24Point, BinaryColor::On))
                        .draw(display)
                        .unwrap();

                    // Render constant status message
                    /*Text::new("Max number:", Point::new(10,0))
                        .into_styled(TextStyle::new(ProFont14Point, BinaryColor::On))
                        .draw(display)
                        .unwrap();*/

                    // Write buffer to display
                    display.flush().unwrap();
                });
                });

                // Schedule the screen to go back to what it was previously showing
                disp_call_cnt.lock(|disp_call_cnt|{*disp_call_cnt += 1});
                let _ = reset_display::spawn_after(Seconds(15_u32));
            },
            ScreenPage::Boot => {
                display.clear();
                let raw_image: ImageRaw<BinaryColor> = ImageRaw::new(LOGO, 128, 64);
                Image::new(&raw_image, Point::zero())
                    .draw(display)
                    .unwrap();
                Text::new("Dice Roller!", Point::new(10,44))
                    .into_styled(TextStyle::new(ProFont14Point, BinaryColor::On))
                    .draw(display)
                    .unwrap();
                // Write buffer to display
                display.flush().unwrap();
                // Schedule the screen to go back to what it was previously showing
                disp_call_cnt.lock(|disp_call_cnt|{*disp_call_cnt += 1});
                let _ = reset_display::spawn_after(Seconds(3_u32));
            },
            ScreenPage::Sleep => {
                display.clear();
                let raw_image: ImageRaw<BinaryColor> = ImageRaw::new(LOGO, 128, 64);
                Image::new(&raw_image, Point::zero())
                    .draw(display)
                    .unwrap();
                Text::new("Power Off", Point::new(20,44))
                    .into_styled(TextStyle::new(ProFont14Point, BinaryColor::On))
                    .draw(display)
                    .unwrap();
                // Write buffer to display
                display.flush().unwrap();
            },
            ScreenPage::Charging => {
                display.clear();
                let raw_image: ImageRaw<BinaryColor> = ImageRaw::new(BIGBOLT, 128, 64);
                Image::new(&raw_image, Point::zero())
                    .draw(display)
                    .unwrap();
                /*Text::new("", Point::new(20,44))
                    .into_styled(TextStyle::new(ProFont14Point, BinaryColor::On))
                    .draw(display)
                    .unwrap();*/
                // Write buffer to display
                display.flush().unwrap();
            },
        }
    });
}

// Figure out what the display should be showing for the current system state and show it
pub fn reset_display(cx: reset_display::Context) {
    let mut disp_call_cnt = cx.resources.disp_call_cnt;
    let mut sys_state = cx.resources.sys_state;
    disp_call_cnt.lock(|disp_call_cnt|{
        // Call count mechanism ensures we don't revert the screen if we're already showing
        //   another, newer status message.
        if *disp_call_cnt <= 1 {
           *disp_call_cnt = 0;
           sys_state.lock(|sys_state|{
               match *sys_state {
                   SysState::Setup => { let _ = update_display::spawn(ScreenPage::Setup); },
                   SysState::Sleep => { let _ = update_display::spawn(ScreenPage::Sleep); },
               }
           });
       } else {
           *disp_call_cnt -= 1;
       }
    });
}