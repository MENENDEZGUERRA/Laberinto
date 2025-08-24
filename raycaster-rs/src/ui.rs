#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Welcome,
    Playing,
    Success,
}

pub struct UiState {
    pub mode: GameMode,
    pub fps: f32,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            mode: GameMode::Welcome,
            fps: 0.0,
        }
    }
}
