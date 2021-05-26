// Enums used in this project

// State of the overarching state machine
#[derive(PartialEq)]
pub enum SysState {
    Setup, // User is setting time on the timer
    Timer, // Timer is ticking down
    Sleep, // MCU has asked PMIC to shut off power
}

pub enum ScreenPage {
    // Main status pages for the main system states
    Setup,
    Timer,
    Sleep,
    // Temporary notification pages
    Brightness,
    Alarm,
    Boot,
    Charging,
}