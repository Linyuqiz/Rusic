mod audiostate;
mod pagestate;
use audiostate::AudioState;
use pagestate::PageState;

pub struct State {
    audio_state: AudioState,
    page_state: PageState,
}
impl Default for State {
    fn default() -> Self {
        Self {
            audio_state: AudioState::Pausing,
            page_state: PageState::Main,
        }
    }
}
