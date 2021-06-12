// Enums used in this project

// State of the overarching state machine
#[derive(PartialEq)]
pub enum SysState {
    Setup, // User is setting time on the timer
    Sleep, // MCU has asked PMIC to shut off power
}

pub enum ScreenPage {
    // Main status pages for the main system states
    Setup,
    Sleep,
    // Temporary notification pages
    Brightness,
    Number,
    Boot,
    Charging,
}