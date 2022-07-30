pub mod errors {
    use std::{error::Error, fmt::Display};

    

    use crate::lexer::lexer::Token;

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

    #[derive(Debug, Clone)]
    pub(crate) struct ParseErr {
        expected: Token,
        actual: Token,
        source: BaseErr,
    }

    impl ParseErr {
        pub fn new(expected: Token, actual: Token) -> Self {
            ParseErr {
                expected,
                actual,
                source: BaseErr {},
            }
        }
    }

    impl Display for ParseErr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Expected {:#?} token, got {:#?} token.",
                self.expected, self.actual
            )
        }
    }

    impl Error for ParseErr {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&self.source)
        }

        fn cause(&self) -> Option<&dyn Error> {
            self.source()
        }
    }
}
