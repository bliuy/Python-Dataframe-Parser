pub mod BaseErr {
    use std::{error::Error, fmt::Display};
    #[derive(Debug, Clone)]
    pub(crate) struct BaseErr;

    /// BaseErr type
    /// Root error type - Serves as the terminal source for errors.
    impl Display for BaseErr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Base Error type.")
        }
    }

    impl Error for BaseErr {}
}

pub mod ParseErr {
    use std::error::Error;
    use std::fmt::Display;

    use crate::lexer::lexer::Token;

    

    #[derive(Debug)]
    pub(crate) enum ParseErr {
        WrongToken {
            expected: Vec<Token>,
            actual: Token,
            source: Box<dyn Error>,
        },
        CustomParseError {
            error_msg: String,
            source: Box<dyn Error>,
        },
    }

    impl Display for ParseErr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ParseErr::WrongToken {
                    expected,
                    actual,
                    source: _,
                } => {
                    let mut err_msg = String::from("Expected ");
                    let mut i = 0;
                    loop {
                        let _tok = &expected[i];
                        if i == expected.len() - 1 {
                            let msg = format!(
                                "{:#?} token, got {:#?} token instead.",
                                expected[i], actual
                            );
                            err_msg.push_str(&msg);
                            break;
                        } else {
                            let msg = format!("{:#?} token or ", expected[i]);
                            err_msg.push_str(&msg);
                            i += 1;
                        }
                    }

                    write!(f, "{}", err_msg)
                }
                ParseErr::CustomParseError { error_msg, source: _ } => {
                    write!(
                        f,
                        "Error raised when parsing. See error message: {}",
                        error_msg
                    )
                }
            }
        }
    }

    impl Error for ParseErr {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                ParseErr::WrongToken {
                    expected: _,
                    actual: _,
                    source,
                } => {
                    let error = &**source; // Reference to the trait obejct within the Box
                    Some(error)
                }
                ParseErr::CustomParseError { error_msg: _, source } => {
                    let error = &**source; // Reference to the trait obejct within the Box
                    Some(error)
                }
            }
        }

        fn cause(&self) -> Option<&dyn Error> {
            self.source()
        }
    }
}
