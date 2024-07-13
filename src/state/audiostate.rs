use crate::message::audio::AudioTo;

pub enum AudioState {
    Playing,
    Pausing,
    Stopping,
}
impl From<AudioTo> for AudioState {
    fn from(audio_to: AudioTo) -> Self {
        match audio_to {
            AudioTo::Play => AudioState::Playing,
            AudioTo::Stop => AudioState::Stopping,
            AudioTo::Pause => AudioState::Pausing,
        }
    }
}
