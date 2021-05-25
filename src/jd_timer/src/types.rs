#[derive(PartialEq)]
pub enum SysState {
    Setup,
    Timer,
    Sleep,
}

pub enum ScreenPage {
    Setup,
    Timer,
    Brightness,
    Alarm,
    Boot,
    Sleep,
    Charging,
}