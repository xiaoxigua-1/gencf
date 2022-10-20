use crate::position::Position;

#[derive(Debug)]
pub struct GenCFError {
    pub error_message: &'static str,
    pub pos: Position,
}