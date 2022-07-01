use std::fmt;

/// Under utilized error system, it would be great to see this grown!
#[derive(Debug)]
pub struct GameError {
    pub error_type: GameErrorType,
    pub error_payload: String,
}

impl GameError {
    pub fn new(error_type: GameErrorType, payload: String) -> Self {
        GameError {
            error_type,
            error_payload: payload,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum GameErrorType {
    ItemMissing,
    CraftingFailed,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.error_payload)
    }
}
