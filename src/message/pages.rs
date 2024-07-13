#[derive(Debug, Clone)]
pub enum PageTo {
    Main,
    Settings(SettingsTo),
}
#[derive(Debug, Clone)]

pub enum SettingsTo {
    Audio(AudioTo),
    About,
    Common(CommonTo),
    Advanced(AdvancedTo),
}
#[derive(Debug, Clone)]

pub enum AudioTo {
    Own,
}
#[derive(Debug, Clone)]

pub enum CommonTo {
    Own,
}
#[derive(Debug, Clone)]

pub enum AdvancedTo {
    Own,
}
#[derive(Debug, Clone)]

pub enum Common {
    Own,
}
