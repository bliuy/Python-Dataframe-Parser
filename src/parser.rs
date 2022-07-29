pub mod parser {
    use crate::lexer::lexer::Token;

    pub struct RustyParser {
        current_token: Token,
        next_token: Token,
    }
}
