use crate::app;
use crate::app::*;
use crate::types::{ScreenPage, SysState};
use crate::config::{HARD_BOILED, SOFT_BOILED};

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
};
use core::fmt::Write;
use heapless::String;
use heapless::consts::*;
use profont::ProFont24Point;
use profont::ProFont14Point;
use rtic::time::duration::Seconds;

pub fn update_display(cx: update_display::Context, screen_type:ScreenPage){
    // Bring resources into scope
    let (mut display, mut brightness_state) =
        (cx.resources.display, cx.resources.brightness_state);
    let mut time_remaining = cx.resources.time_remaining;
    let mut disp_call_cnt = cx.resources.disp_call_cnt;

    display.lock(|display| {
        match screen_type {
            // Display the time set screen
            ScreenPage::Setup => {
                disp_call_cnt.lock(|disp_call_cnt|{*disp_call_cnt = 0});
                time_remaining.lock(|time_remaining| {
                    display.clear();
                    // Format the text
                    let mut data = String::<U16>::from("");
                    let minutes = *time_remaining/60;
                    let seconds = *time_remaining%60;
                    let _ = write!(data, "{:>2}:{:>02}", minutes, seconds);

                    // Create the graphics object and draw it on the "buffer"
                    Text::new(&data[..], Point::new(20,16))
                        .into_styled(TextStyle::new(ProFont24Point, BinaryColor::On))
                        .draw(display)
                        .unwrap();

                    // Determine special status message for special time settings
                    let status_msg: &str = match *time_remaining {
                        SOFT_BOILED => "Soft-Boiled",
                        HARD_BOILED => "Hard-Boiled",
                        _ => "",
                    };

                    // Render special status message
                    Text::new(status_msg, Point::new(10,48))
                        .into_styled(TextStyle::new(ProFont14Point, BinaryColor::On))
                        .draw(display)
                        .unwrap();

                    // Render constant status message
                    /*Text::new("Timer time:", Point::new(10,0))
                        .into_styled(TextStyle::new(ProFont14Point, BinaryColor::On))
                        .draw(display)
                        .unwrap();*/

                    // Write buffer to display
                    display.flush().unwrap();
                });
            },
            // Display the countdown screen
            ScreenPage::Timer => {
                disp_call_cnt.lock(|disp_call_cnt|{*disp_call_cnt = 0});
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
            ScreenPage::Alarm => {
                // Schedule the screen to go back to what it was previously showing
                disp_call_cnt.lock(|disp_call_cnt|{*disp_call_cnt += 1});
                let _ = reset_display::spawn_after(Seconds(2_u32));
            },
            ScreenPage::Boot => {
                display.clear();
                Text::new("BOOT", Point::new(10,48))
                    .into_styled(TextStyle::new(ProFont14Point, BinaryColor::On))
                    .draw(display)
                    .unwrap();
                // Write buffer to display
                display.flush().unwrap();
                // Schedule the screen to go back to what it was previously showing
                disp_call_cnt.lock(|disp_call_cnt|{*disp_call_cnt += 1});
                let _ = reset_display::spawn_after(Seconds(2_u32));
            },
        }
    });
}

pub fn reset_display(cx: reset_display::Context) {
    let mut disp_call_cnt = cx.resources.disp_call_cnt;
    let mut sys_state = cx.resources.sys_state;
    disp_call_cnt.lock(|disp_call_cnt|{
       if *disp_call_cnt == 0 {
           return;
       } else if *disp_call_cnt == 1 {
           *disp_call_cnt = 0;
           sys_state.lock(|sys_state|{
               match *sys_state {
                   SysState::Setup => { let _ = update_display::spawn(ScreenPage::Setup); },
                   SysState::Timer => { let _ = update_display::spawn(ScreenPage::Timer); },
               }
           });
       } else {
           *disp_call_cnt -= 1;
       }
    });
}