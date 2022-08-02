//! Contains the parser module used for parsing the tokens into Python dataframe code.
pub mod parser {
    use std::collections::HashSet;
    use std::error::Error;

    use logos::Lexer;

    use crate::errors::BaseErr::BaseErr;
    use crate::errors::ParseErr::ParseErr;
    // use crate::errors::{ParseErr};
    use crate::lexer::lexer::Token;

    #[derive(Eq, Hash, PartialEq)]
    pub(crate) enum EntityType {
        Table(String),
        Column(String),
    }

    /// Parser struct.
    /// Lifetime of the parser is tied to the lifetime of the lexer.
    /// Hence, the lexer must live at least as long as the parser.
    pub struct RustyParser<'a> {
        current_token: Option<Token>,
        next_token: Option<Token>,
        lexer: logos::Lexer<'a, Token>,
        entities: HashSet<EntityType>,
        python_output: String,
    }

    impl<'a> RustyParser<'a> {
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
                    Some(i) => Err(ParseErr::WrongToken {
                        expected: vec![kind.clone()],
                        actual: i,
                        source: Box::new(BaseErr {}),
                    }),
                    None => Err(ParseErr::CustomParseError {
                        error_msg: "".to_string(),
                        source: Box::new(BaseErr {}),
                    }),
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

        fn new(lex: Lexer<'a, Token>) -> Self {
            let mut parser = RustyParser {
                current_token: None,
                next_token: None,
                lexer: lex,
                entities: HashSet::new(),
                python_output: "".to_string(),
            };
            parser.move_token(); // Moving the first token value into the "next_token" field of the struct.
            parser.move_token(); // Moving the second token value into the "next_token" field of the struct + moving the first token value into the "current_token" field of the struct.
            parser
        }

        fn program(&mut self) -> Result<(), ParseErr> {
            // Initial token - Should represent a table name
            let token = self.current_token.take();
            match token {
                Some(tok) => match tok {
                    Token::OpenSquareBracket => todo!(),
                    Token::Identity(identity) => {
                        self.entities.insert(EntityType::Table(identity));
                        Ok(())
                    }
                    other => {
                        return Err(ParseErr::WrongToken {
                            expected: vec![
                                Token::OpenSquareBracket,
                                Token::Identity("".to_string()),
                            ],
                            actual: other,
                            source: Box::new(BaseErr {}),
                        })
                    }
                },
                None => todo!(),
            }
        }
    }
}
