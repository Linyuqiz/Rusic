mod settings;
use settings::Settings;

use crate::message::pages::{PageTo, SettingsTo};
pub enum PageState {
    Main,
    Settings(Settings),
}
impl From<PageTo> for PageState {
    fn from(page_to: PageTo) -> Self {
        match page_to {
            PageTo::Main => PageState::Main,
            PageTo::Settings(settings_to) => PageState::Settings(match settings_to {
                SettingsTo::Audio(_) => Settings::Audio,
                SettingsTo::About => Settings::About,
            }),
        }
    }
}
