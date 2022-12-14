//! Contains the parser module used for parsing the tokens into Python dataframe code.
pub mod parser {
    use std::collections::HashSet;
    
    

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
            if let Some(token) = self.current_token.as_ref() {
                let result = token == kind;
                return result;
            }
            false
        }

        /// Returns true if the next token matches the input token. Returns false otherwise.
        fn check_next_token(&self, kind: &Token) -> bool {
            if let Some(token) = self.next_token.as_ref() {
                let result = token == kind;
                return result;
            }
            false
        }

        /// If the current token matches the input token, will advance the current token to the next token.
        /// Else, will return a ParseErr type.
        fn match_token(&mut self, kind: &Token) -> Result<(), ParseErr> {
            match self.check_token(kind) {
                true => {
                    self.move_token();
                    Ok(())
                }
                false => match self.current_token.as_ref() {
                    Some(i) => Err(ParseErr::WrongToken {
                        expected: vec![kind.clone()],
                        actual: i.clone(),
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
            let token = self.current_token.as_ref();
            match token {
                Some(tok) => match tok {
                    Token::Identity(identity) => {
                        self.entities.insert(EntityType::Table(identity.clone())); // Adding into the HashSet of tables
                        self.main_table_name.push_str(identity); // Indicating that this will be the main table.
                        let gen_code = format!("{} = <filepath> \n", identity);
                        self.python_output.push_str(&gen_code);
                        self.move_token();
                    }
                    other => {
                        return Err(ParseErr::WrongToken {
                            expected: vec![Token::Identity("<variable name>".to_string())],
                            actual: other.clone(),
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

            while self.current_token.is_some() {
                self.statement()?;
            }

            Ok(())
        }

        fn statement(&mut self) -> Result<(), ParseErr> {
            match self.current_token.as_ref() {
                Some(Token::READ) => {
                    self.move_token();
                    self.read_statement()?;
                    Ok(())
                }
                Some(Token::WHERE) => {
                    self.move_token();
                    self.where_statement()?;
                    Ok(())
                }
                Some(Token::EXTEND) => {
                    self.move_token();
                    self.extend_statement()?;
                    Ok(())
                }
                Some(tok) => Err(ParseErr::WrongToken {
                    expected: vec![Token::READ, Token::WHERE, Token::WHERE],
                    actual: tok.clone(),
                    source: Box::new(BaseErr {}),
                }),
                None => Err(ParseErr::CustomParseError {
                    error_msg: "Expected a statement!".to_string(),
                    source: Box::new(BaseErr {}),
                }),
            }
        }

        fn read_statement(&mut self) -> Result<(), ParseErr> {
            match self.current_token.as_ref() {
                Some(tok) => match tok {
                    Token::Identity(identity) => match identity.to_lowercase().as_str() {
                        "csv" => {
                            let code_gen = format!(
                                "{} = pd.DataFrame.read_csv({}) \n",
                                self.main_table_name, self.main_table_name
                            );
                            self.python_output.push_str(&code_gen);
                            self.move_token();
                            Ok(())
                        }
                        "excel" => {
                            let code_gen = format!(
                                "{} = pd.DataFrame.read_excel({}) \n",
                                self.main_table_name, self.main_table_name
                            );
                            self.python_output.push_str(&code_gen);
                            self.move_token();
                            Ok(())
                        }
                        _ => {
                            Err(ParseErr::CustomParseError {
                                error_msg: "Expected either 'csv' or 'excel'.".to_string(),
                                source: Box::new(BaseErr {}),
                            })
                        }
                    },
                    _other => todo!(),
                },
                None => {
                    Err(ParseErr::CustomParseError {
                        error_msg: "Expected either 'csv' or 'excel'.".to_string(),
                        source: Box::new(BaseErr {}),
                    })
                }
            }
        }

        fn where_statement(&mut self) -> Result<(), ParseErr> {
            self.python_output.push_str("cond = (");
            match self.current_token.as_ref() {
                // NOTE: Error here due to the inconsistent matching syntax. This should be inspecting the self.current_token instead of the next token.
                Some(Token::ISNOTNULL) => {
                    self.isnotnull()?;
                    self.python_output.push_str(")\n");
                    let code_gen = format!(
                        "{} = {}[cond]\n",
                        self.main_table_name, self.main_table_name
                    );
                    self.python_output.push_str(&code_gen);
                    Ok(())
                }
                _ => {
                    self.comparison()?;
                    self.python_output.push_str(")\n");
                    let code_gen = format!(
                        "{} = {}[cond]\n",
                        self.main_table_name, self.main_table_name
                    );
                    self.python_output.push_str(&code_gen);
                    Ok(())
                }
            }
        }

        fn extend_statement(&mut self) -> Result<(), ParseErr> {
            self.column()?;
            self.match_token(&Token::EqualsOperator)?;
            self.python_output.push_str(" = ");
            self.expression()?;
            self.python_output.push('\n');
            Ok(())
        }

        fn isnotnull(&mut self) -> Result<(), ParseErr> {
            self.match_token(&Token::ISNOTNULL)?;
            self.match_token(&Token::OpenBracket)?;
            self.column()?;
            self.match_token(&Token::CloseBracket)?;
            self.python_output.push_str(".notna()");

            Ok(())
        }

        // fn isnull(&mut self) -> Result<(), ParseErr> {
        //     match self.current_token.take() {
        //         Some(Token::ISNULL) => {
        //             self.move_token();
        //             match self.current_token.take() {
        //                 Some(Token::OpenSquareBracket) => {
        //                     self.move_token();
        //                     self.column()?
        //                 }
        //                 _ => {
        //                     return Err(ParseErr::CustomParseError {
        //                         error_msg: "Expected '(' after the 'isnull' function.".to_string(),
        //                         source: Box::new(BaseErr {}),
        //                     })
        //                 }
        //             }
        //             Ok(())
        //         }
        //         _ => Err(ParseErr::CustomParseError {
        //             error_msg: "Expected isnull function.".to_string(),
        //             source: Box::new(BaseErr {}),
        //         }),
        //     }
        // }

        fn comparison(&mut self) -> Result<(), ParseErr> {
            // self.python_output.push_str("cond = (");
            self.expression()?;
            match self.current_token.as_ref() {
                Some(Token::GreaterThan) => {
                    self.python_output.push('>');
                    self.move_token();
                }
                Some(Token::GreaterThanEqualsTo) => {
                    self.python_output.push_str(">=");
                    self.move_token();
                }
                Some(Token::LessThan) => {
                    self.python_output.push('<');
                    self.move_token();
                }
                Some(Token::LessThanEqualsTo) => {
                    self.python_output.push_str("<=");
                    self.move_token();
                }
                Some(tok) => {
                    return Err(ParseErr::WrongToken {
                        expected: vec![
                            Token::GreaterThan,
                            Token::GreaterThanEqualsTo,
                            Token::LessThan,
                            Token::LessThanEqualsTo,
                        ],
                        actual: tok.clone(),
                        source: Box::new(BaseErr {}),
                    })
                }
                None => {
                    return Err(ParseErr::NoTokenLeftError {
                        source: Box::new(BaseErr {}),
                    })
                }
            }
            self.expression()?;
            // self.python_output.push_str(")");
            Ok(())
        }

        fn expression(&mut self) -> Result<(), ParseErr> {
            // self.python_output.push_str("(");
            self.term()?;
            loop {
                match self.current_token.as_ref() {
                    Some(Token::PlusOperator) => {
                        self.python_output.push('+');
                        self.move_token();
                        self.term()?;
                    }
                    Some(Token::MinusOperator) => {
                        self.python_output.push('-');
                        self.move_token();
                        self.term()?;
                    }
                    _ => break,
                }
            }
            // self.python_output.push_str(")");
            Ok(())
        }

        fn term(&mut self) -> Result<(), ParseErr> {
            // self.python_output.push_str("(");
            self.unary()?;
            loop {
                match self.current_token.as_ref() {
                    Some(Token::MulOperator) => {
                        self.python_output.push('*');
                        self.move_token();
                        self.unary()?;
                    }
                    Some(Token::DivOperator) => {
                        self.python_output.push('/');
                        self.move_token();
                        self.unary()?;
                    }
                    _ => break,
                }
            }

            // self.python_output.push_str(")");
            Ok(())
        }

        fn unary(&mut self) -> Result<(), ParseErr> {
            if self.match_token(&Token::PlusOperator).is_ok() {
                self.primary()?;
            }

            if self.match_token(&Token::MinusOperator).is_ok() {
                self.python_output.push_str("-1 * ");
                self.primary()?;
            }

            self.primary()
        }

        fn primary(&mut self) -> Result<(), ParseErr> {
            match &self.current_token {
                Some(Token::OpenSquareBracket) => {
                    self.column()?;
                    Ok(())
                }
                Some(Token::Integer(_)) => {
                    self.number()?;
                    Ok(())
                }
                Some(Token::Float(_)) => {
                    self.float()?;
                    Ok(())
                }
                Some(tok) => Err(ParseErr::WrongToken {
                    expected: vec![Token::OpenSquareBracket],
                    actual: tok.clone(),
                    source: Box::new(BaseErr {}),
                }),
                _ => Err(ParseErr::CustomParseError {
                    error_msg: "Expected a column, number or float.".to_string(),
                    source: Box::new(BaseErr {}),
                }),
            }
        }

        fn column(&mut self) -> Result<(), ParseErr> {
            if self.match_token(&Token::OpenSquareBracket).is_ok() {
                self.python_output.push_str("df.loc[:,");
                self.str()?;
                self.match_token(&Token::CloseSquareBracket)?;
                self.python_output.push(']');
                return Ok(());
            }

            match self.current_token.as_ref() {
                Some(Token::Identity(_identity)) => {
                    self.move_token();
                    todo!();
                    Ok(())
                }
                Some(tok) => {
                    Err(ParseErr::WrongToken {
                        expected: vec![
                            Token::OpenSquareBracket,
                            Token::Identity("Identity".to_string()),
                        ],
                        actual: tok.clone(),
                        source: Box::new(BaseErr {}),
                    })
                }
                None => {
                    Err(ParseErr::NoTokenLeftError {
                        source: Box::new(BaseErr {}),
                    })
                }
            }
        }

        fn number(&mut self) -> Result<(), ParseErr> {
            match self.current_token.as_ref() {
                Some(Token::Integer(int)) => {
                    let code_gen = format!("{}", int);
                    self.python_output.push_str(&code_gen);
                    self.move_token();
                    Ok(())
                }
                Some(tok) => {
                    Err(ParseErr::WrongToken {
                        expected: vec![Token::Integer(0)],
                        actual: tok.clone(),
                        source: Box::new(BaseErr {}),
                    })
                }
                None => {
                    Err(ParseErr::NoTokenLeftError {
                        source: Box::new(BaseErr {}),
                    })
                }
            }
        }

        fn float(&mut self) -> Result<(), ParseErr> {
            match self.current_token.as_ref() {
                Some(Token::Float(float)) => {
                    let code_gen = format!("{}", float);
                    self.python_output.push_str(&code_gen);
                    self.move_token();
                    Ok(())
                }
                Some(tok) => {
                    Err(ParseErr::WrongToken {
                        expected: vec![Token::Float(0.0)],
                        actual: tok.clone(),
                        source: Box::new(BaseErr {}),
                    })
                }
                None => {
                    Err(ParseErr::NoTokenLeftError {
                        source: Box::new(BaseErr {}),
                    })
                }
            }
        }

        fn str(&mut self) -> Result<(), ParseErr> {
            self.match_token(&Token::QuotationMark)?;
            self.python_output.push('"');

            let mut current_str = String::from("");

            loop {
                match self.current_token.as_ref() {
                    Some(Token::Identity(identity)) => {
                        current_str.push_str(identity); // Appending the token string
                        current_str.push(' '); // Appending a whitespace character
                        self.move_token();
                    }
                    Some(Token::QuotationMark) => {
                        if !current_str.is_empty() {
                            current_str.pop(); // Removing the trailing whitespace added during the while-let loop above.
                        }
                        self.python_output.push_str(&current_str);
                        self.python_output.push('"');
                        self.move_token();
                        break;
                    }
                    Some(tok) => {
                        return Err(ParseErr::WrongToken {
                            expected: vec![
                                Token::QuotationMark,
                                Token::Identity("Identity".to_string()),
                            ],
                            actual: tok.clone(),
                            source: Box::new(BaseErr {}),
                        })
                    }
                    None => {
                        return Err(ParseErr::NoTokenLeftError {
                            source: Box::new(BaseErr {}),
                        })
                    }
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{parser::RustyParser};

    /// Extremely basic test to see if the parsing even works.
    /// If this fails, this means that there are serious underlying problems that needs to be fixed even before addressing any other failed tests.
    #[test]
    fn basic_parsing_test() {
        let input = r#"
        sourceTable
        | READ csv
        | WHERE ["foo bar"] > 5
        "#;

        let expected_output =
            "sourceTable = <filepath> \nsourceTable = pd.DataFrame.read_csv(sourceTable) \ncond = (df.loc[:,\"foo bar\"]>5)\nsourceTable = sourceTable[cond]\n";

        let lex = <crate::lexer::lexer::Token as logos::Logos>::lexer(input);
        let mut pars = RustyParser::new(lex);
        pars.program().unwrap();
        assert_eq!(expected_output, &pars.python_output);
        println!("{}", &pars.python_output);
    }

    #[test]
    fn isnotnull_test() {
        let input = r#"
        sourceTable
        | READ csv
        | WHERE isnotnull(["foo bar baz"])
        "#;

        let expected_output =
            "sourceTable = <filepath> \nsourceTable = pd.DataFrame.read_csv(sourceTable) \ncond = (df.loc[:,\"foo bar baz\"].notna())\nsourceTable = sourceTable[cond]\n";

        let lex = <crate::lexer::lexer::Token as logos::Logos>::lexer(input);
        let mut pars = RustyParser::new(lex);
        pars.program().unwrap();
        assert_eq!(expected_output, &pars.python_output);
        println!("{}", &pars.python_output);
    }

    #[test]
    fn extend_test() {
        let input = r#"
        sourceTable
        | READ csv
        | EXTEND ["foo"] = ["bar"] * 2
        | EXTEND ["baz"] = ["qux"] * 5.1
        "#;

        let expected_output =
            "sourceTable = <filepath> \nsourceTable = pd.DataFrame.read_csv(sourceTable) \ndf.loc[:,\"foo\"] = df.loc[:,\"bar\"]*2\ndf.loc[:,\"baz\"] = df.loc[:,\"qux\"]*5.1\n";

        let lex = <crate::lexer::lexer::Token as logos::Logos>::lexer(input);
        let mut pars = RustyParser::new(lex);
        pars.program().unwrap();
        assert_eq!(expected_output, &pars.python_output);
        println!("{}", &pars.python_output);
    }
}
