use std::path::Path;

use crate::position::Position;

#[derive(Debug)]
pub struct GenCFError<'a> {
    pub error_message: &'static str,
    pub pos: Position,
    pub path: &'a Path,
}
