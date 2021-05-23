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
    let (mut display, mut sys_state, mut brightness_state) =
        (cx.resources.display, cx.resources.sys_state, cx.resources.brightness_state);
    let mut time_remaining = cx.resources.time_remaining;

    display.lock(|display| {
        match screen_type {
            // Display whatever screen should currently be displayed
            ScreenPage::Main => {
                sys_state.lock(|sys_state| {
                    match *sys_state {
                        SysState::Setup => { let _ = update_display::spawn(ScreenPage::Setup); },
                        SysState::Timer => { let _ = update_display::spawn(ScreenPage::Timer); },
                    }
                });
            },
            // Display the time set screen
            ScreenPage::Setup => {
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
                    // Write buffer to display
                    display.flush().unwrap();
                });
            },
            // Display the countdown screen
            ScreenPage::Timer => {},
            ScreenPage::Brightness => {
                // Schedule the screen to go back to what it was previously showing
                let _ = update_display::spawn_after(Seconds(3_u32), ScreenPage::Main);
            },
            ScreenPage::Alarm => {
                // Schedule the screen to go back to what it was previously showing
                let _ = update_display::spawn_after(Seconds(1_u32), ScreenPage::Main);
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
                let _ = update_display::spawn_after(Seconds(2_u32), ScreenPage::Main);
            },
        }
    });
}