use std::path::Path;

use crate::position::Position;

#[derive(Debug)]
pub struct GenCFError<'a> {
    pub error_message: &'static str,
    pub pos: Position,
    pub path: &'a Path,
}

impl GenCFError<'_> {
    pub fn new<'a>(message: &'static str, pos: Position, path: &'a Path) -> GenCFError<'a> {
        GenCFError { error_message: message, pos, path }
    }
}
