pub enum Settings {
    Own,
    Audio//(Audio),
    Common(Common),
    Advanced(Advanced),
    About,
}
impl Default for Settings {
    fn default() -> Self {
        Self::Own
    }
}
// pub enum Audio {}
pub enum Common {
    Style,
}
pub enum Advanced {
    CustomStyle,
}
