#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: Tokens,
    pub pos: Option<Position>,
}