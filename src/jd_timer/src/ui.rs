use crate::app;
use crate::app::*;
use crate::types::{ScreenPage, SysState};
use crate::config::{HARD_BOILED, SOFT_BOILED};
use crate::logo::LOGO;
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
    let mut max_time = cx.resources.max_time;
    let mut disp_call_cnt = cx.resources.disp_call_cnt;
    let mut rtc = cx.resources.rtc;

    display.lock(|display| {
        match screen_type {
            // Display the time set screen
            ScreenPage::Setup => {
                disp_call_cnt.lock(|disp_call_cnt|{*disp_call_cnt = 0});
                max_time.lock(|max_time| {
                    display.clear();
                    // Format the text
                    let mut data = String::<U16>::from("");
                    let minutes = *max_time/60;
                    let seconds = *max_time%60;
                    let _ = write!(data, "{:>2}:{:>02}", minutes, seconds);

                    // Create the graphics object and draw it on the "buffer"
                    Text::new(&data[..], Point::new(20,16))
                        .into_styled(TextStyle::new(ProFont24Point, BinaryColor::On))
                        .draw(display)
                        .unwrap();

                    // Determine special status message for special time settings
                    let status_msg: &str = match *max_time {
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
                    Text::new("Set Time:", Point::new(10,0))
                        .into_styled(TextStyle::new(ProFont14Point, BinaryColor::On))
                        .draw(display)
                        .unwrap();

                    // Write buffer to display
                    display.flush().unwrap();
                });
            },
            // Display the countdown screen
            ScreenPage::Timer => {
                disp_call_cnt.lock(|disp_call_cnt|{*disp_call_cnt = 0});
                let time_remaining: u16 = rtc.lock(|rtc| {
                    return max_time.lock(|max_time|{
                        let current_time = rtc_util::current_time(rtc) as u16;
                        if current_time <= *max_time {
                            return *max_time - current_time
                        } else {
                            return 0
                        }
                    });
                });
                display.clear();
                // Format the text
                let mut data = String::<U16>::from("");
                let minutes = time_remaining/60;
                let seconds = time_remaining%60;
                let _ = write!(data, "{:>2}:{:>02}", minutes, seconds);

                // Create the graphics object and draw it on the "buffer"
                Text::new(&data[..], Point::new(20,16))
                    .into_styled(TextStyle::new(ProFont24Point, BinaryColor::On))
                    .draw(display)
                    .unwrap();

                // Render constant status message
                /*Text::new("Timer time:", Point::new(10,0))
                    .into_styled(TextStyle::new(ProFont14Point, BinaryColor::On))
                    .draw(display)
                    .unwrap();*/

                // Write buffer to display
                display.flush().unwrap();
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
                display.clear();
                Text::new("Alarm!", Point::new(20,16))
                    .into_styled(TextStyle::new(ProFont24Point, BinaryColor::On))
                    .draw(display)
                    .unwrap();
                display.flush().unwrap();
                // Schedule the screen to go back to what it was previously showing
                disp_call_cnt.lock(|disp_call_cnt|{*disp_call_cnt += 1});
                let _ = reset_display::spawn_after(Seconds(5_u32));
            },
            ScreenPage::Boot => {
                display.clear();
                let raw_image: ImageRaw<BinaryColor> = ImageRaw::new(LOGO, 128, 64);
                Image::new(&raw_image, Point::zero())
                    .draw(display)
                    .unwrap();
                Text::new("Egg Timer!", Point::new(20,44))
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
                let raw_image: ImageRaw<BinaryColor> = ImageRaw::new(LOGO, 128, 64);
                Image::new(&raw_image, Point::zero())
                    .draw(display)
                    .unwrap();
                Text::new("Charging...", Point::new(20,44))
                    .into_styled(TextStyle::new(ProFont14Point, BinaryColor::On))
                    .draw(display)
                    .unwrap();
                // Write buffer to display
                display.flush().unwrap();
            },
        }
    });
}

pub fn reset_display(cx: reset_display::Context) {
    let mut disp_call_cnt = cx.resources.disp_call_cnt;
    let mut sys_state = cx.resources.sys_state;
    disp_call_cnt.lock(|disp_call_cnt|{
        if *disp_call_cnt <= 1 {
           *disp_call_cnt = 0;
           sys_state.lock(|sys_state|{
               match *sys_state {
                   SysState::Setup => { let _ = update_display::spawn(ScreenPage::Setup); },
                   SysState::Timer => { let _ = update_display::spawn(ScreenPage::Timer); },
                   SysState::Sleep => { let _ = update_display::spawn(ScreenPage::Sleep); },
               }
           });
       } else {
           *disp_call_cnt -= 1;
       }
    });
}