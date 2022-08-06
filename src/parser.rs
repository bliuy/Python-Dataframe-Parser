//! Contains the parser module used for parsing the tokens into Python dataframe code.
pub mod parser {
    use std::collections::HashSet;
    use std::error::Error;
    use std::fmt::format;

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
        main_table_name: String,
        pub(crate) python_output: String,
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

        pub fn new(lex: Lexer<'a, Token>) -> Self {
            let mut parser = RustyParser {
                current_token: None,
                next_token: None,
                lexer: lex,
                entities: HashSet::new(),
                main_table_name: "".to_string(),
                python_output: "".to_string(),
            };
            parser.move_token(); // Moving the first token value into the "next_token" field of the struct.
            parser.move_token(); // Moving the second token value into the "next_token" field of the struct + moving the first token value into the "current_token" field of the struct.
            parser
        }

        pub fn program(&mut self) -> Result<(), ParseErr> {
            // Initial token - Should be an identity that represents a token name
            let token = self.current_token.take();
            match token {
                Some(tok) => match tok {
                    Token::Identity(identity) => {
                        self.entities.insert(EntityType::Table(identity.clone())); // Adding into the HashSet of tables
                        self.main_table_name.push_str(&identity); // Indicating that this will be the main table.
                        let gen_code = format!("{} = df \n", identity);
                        self.python_output.push_str(&gen_code);
                        self.move_token();
                    }
                    other => {
                        return Err(ParseErr::WrongToken {
                            expected: vec![Token::Identity("<variable name>".to_string())],
                            actual: other,
                            source: Box::new(BaseErr {}),
                        })
                    }
                },
                None => {
                    return Err(ParseErr::CustomParseError {
                        error_msg: "Expected a identity for the first input.".to_string(),
                        source: Box::new(BaseErr {}),
                    })
                }
            }

            while let Some(_) = &self.current_token {
                self.statement()?;
            }

            Ok(())
        }

        fn statement(&mut self) -> Result<(), ParseErr> {
            match self.current_token.take() {
                Some(tok) => match tok {
                    Token::READ => {
                        self.move_token();
                        self.read_statement();
                        Ok(())
                    }
                    Token::WHERE => todo!(),
                    Token::EXTEND => todo!(),
                    other => {
                        return Err(ParseErr::CustomParseError {
                            error_msg: "Expected a statement!".to_string(),
                            source: Box::new(BaseErr {}),
                        })
                    }
                },
                None => todo!(),
            }
        }

        fn read_statement(&mut self) -> Result<(), ParseErr> {
            match self.current_token.take() {
                Some(tok) => match tok {
                    Token::Identity(identity) => match identity.to_lowercase().as_str() {
                        "csv" => {
                            let code_gen = format!(
                                "{} = pd.DataFrame.read_csv({}) \n",
                                self.main_table_name, self.main_table_name
                            );
                            self.python_output.push_str(&code_gen);
                            self.move_token();
                            return Ok(());
                        }
                        "excel" => {
                            let code_gen = format!(
                                "{} = pd.DataFrame.read_excel({}) \n",
                                self.main_table_name, self.main_table_name
                            );
                            self.python_output.push_str(&code_gen);
                            self.move_token();
                            return Ok(());
                        }
                        _ => {
                            return Err(ParseErr::CustomParseError {
                                error_msg: "Expected either 'csv' or 'excel'.".to_string(),
                                source: Box::new(BaseErr {}),
                            })
                        }
                    },
                    other => todo!(),
                },
                None => {
                    return Err(ParseErr::CustomParseError {
                        error_msg: "Expected either 'csv' or 'excel'.".to_string(),
                        source: Box::new(BaseErr {}),
                    })
                }
            }
        }

        fn where_statement(&mut self) -> Result<(), ParseErr> {
            match self.current_token.take() {
                Some(tok) => {
                    self.move_token();
                    if let Ok(_) = self.isnotnull() {
                        let code_gen = format!(
                            "{} = {}[cond]\n",
                            self.main_table_name, self.main_table_name
                        );
                        self.python_output.push_str(&code_gen);
                        return Ok(());
                    }
                    if let Ok(_) = self.isnull() {
                        return Ok(());
                    }
                    if let Ok(_) = self.expression() {
                        return Ok(());
                    }

                    return Err(ParseErr::CustomParseError {
                        error_msg: "Expected at least 1 comparison operator'.".to_string(),
                        source: Box::new(BaseErr {}),
                    });
                }
                None => {
                    return Err(ParseErr::NoTokenLeftError {
                        source: Box::new(BaseErr {}),
                    })
                }
            }
            todo!()
        }

        fn isnotnull(&mut self) -> Result<(), ParseErr> {
            match self.current_token.take() {
                Some(Token::ISNOTNULL) => {
                    let code_gen = "cond = df.loc[:,";
                    self.python_output.push_str(code_gen);

                    self.move_token();
                    self.column()?;

                    let code_gen = "]";
                    self.python_output.push_str(code_gen);
                }
                Some(tok) => {
                    return Err(ParseErr::WrongToken {
                        expected: vec![Token::ISNOTNULL],
                        actual: tok,
                        source: Box::new(BaseErr {}),
                    })
                }
                _ => {
                    return Err(ParseErr::NoTokenLeftError {
                        source: Box::new(BaseErr {}),
                    })
                }
            }
            Ok(())
        }

        fn isnull(&mut self) -> Result<(), ParseErr> {
            match self.current_token.take() {
                Some(Token::ISNULL) => {
                    self.move_token();
                    match self.current_token.take() {
                        Some(Token::OpenSquareBracket) => {
                            self.move_token();
                            self.column()?
                        }
                        _ => {
                            return Err(ParseErr::CustomParseError {
                                error_msg: "Expected '(' after the 'isnull' function.".to_string(),
                                source: Box::new(BaseErr {}),
                            })
                        }
                    }
                    Ok(())
                }
                _ => Err(ParseErr::CustomParseError {
                    error_msg: "Expected isnull function.".to_string(),
                    source: Box::new(BaseErr {}),
                }),
            }
        }

        fn expression(&mut self) -> Result<(), ParseErr> {
            todo!()
        }
        fn column(&mut self) -> Result<(), ParseErr> {
            match self.current_token.take() {
                Some(Token::OpenSquareBracket) => {
                    self.move_token();
                    self.str()?;

                    Ok(())
                }
                Some(Token::Identity(identity)) => Ok(()),
                _ => todo!(),
            }
        }

        fn str(&mut self) -> Result<(), ParseErr> {
            // Matching the quotation mark at the start of the string
            match self.current_token.take() {
                Some(Token::QuotationMark) => {
                    self.python_output.push_str(r#"""#);
                    self.move_token(); // Start of string
                }
                Some(tok) => {
                    return Err(ParseErr::WrongToken {
                        expected: vec![Token::QuotationMark],
                        actual: tok,
                        source: Box::new(BaseErr {}),
                    })
                }
                _ => {
                    return Err(ParseErr::CustomParseError {
                        error_msg: String::from("No tokens left!"),
                        source: Box::new(BaseErr {}),
                    })
                }
            }

            let mut current_str = String::from("");
            while let Some(Token::Identity(identity)) = self.current_token.take() {
                current_str.push_str(&identity); // Appending the token string
                current_str.push_str(" "); // Appending a whitespace character
            }

            if current_str.len() > 0 {
                current_str.pop(); // Removing the trailing whitespace added during the while-let loop above.
            }

            self.python_output.push_str(&current_str);

            // Matching the quotation mark at the end of the string
            match self.current_token.take() {
                Some(Token::QuotationMark) => {
                    self.python_output.push_str(r#"""#);
                    self.move_token(); // Start of string
                }
                Some(tok) => {
                    return Err(ParseErr::WrongToken {
                        expected: vec![Token::QuotationMark],
                        actual: tok,
                        source: Box::new(BaseErr {}),
                    })
                }
                _ => {
                    return Err(ParseErr::CustomParseError {
                        error_msg: String::from("No tokens left!"),
                        source: Box::new(BaseErr {}),
                    })
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{parser::RustyParser, *};

    /// Extremely basic test to see if the parsing even works.
    /// If this fails, this means that there are serious underlying problems that needs to be fixed even before addressing any other failed tests.
    #[test]
    fn basic_parsing_test() {
        let input = "
        sourceTable
        | READ csv
        ";

        let expected_output =
            "sourceTable = df \nsourceTable = pd.DataFrame.read_csv(sourceTable) \n";

        let lex = <crate::lexer::lexer::Token as logos::Logos>::lexer(input);
        let mut pars = RustyParser::new(lex);
        pars.program().unwrap();
        assert_eq!(expected_output, &pars.python_output);
    }
}
