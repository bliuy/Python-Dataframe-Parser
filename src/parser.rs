//! Contains the parser module used for parsing the tokens into Python dataframe code.
pub mod parser {
    use std::error::Error;

    use crate::errors::errors::ParseErr;
    use crate::lexer::lexer::Token;

    /// Parser struct.
    /// Lifetime of the parser is tied to the lifetime of the source.
    pub struct RustyParser<'a> {
        current_token: Option<Token>,
        next_token: Option<Token>,
        lexer: logos::Lexer<'a, Token>,
    }

    impl RustyParser<'_> {
        /// Returns true if the current token matches the input token. Returns false otherwise.
        fn check_token(&self, kind: &Token) -> bool {
            match &self.current_token {
                Some(kind) => true,
                _ => false,
            }
        }

        /// Returns true if the next token matches the input token. Returns false otherwise.
        fn check_next_token(&self, kind: &Token) -> bool {
            match &self.next_token {
                Some(kind) => true,
                _ => false,
            }
        }

        /// If the current token matches the input token, will advance the current token to the next token.
        /// Else, will return a ParseErr type.
        fn match_token(&mut self, kind: &Token) -> Result<(), ParseErr> {
            match self.check_token(kind) {
                true => {
                    self.move_token();
                    Ok(())
                }
                false => match self.current_token.take() {
                    Some(i) => Err(ParseErr::new(kind.clone(), i)),
                    None => Err(ParseErr::new(kind.clone(), Token::EOF)),
                },
            }
        }

        /// Replaces the current token with the next token.
        /// Replaces the next token with the following token.
        fn move_token(&mut self) {
            let following_token = self.lexer.next();
            self.current_token = self.next_token.take();
            self.next_token = following_token;
        }
    }
}
