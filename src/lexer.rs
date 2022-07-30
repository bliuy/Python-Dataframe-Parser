pub mod lexer {

    use logos;

    // Defining the token types
    #[derive(Debug, logos::Logos, PartialEq, Clone)]
    pub enum Token {
        #[token("+")]
        PlusOperator,
        #[token("-")]
        MinusOperator,
        #[token("*")]
        MulOperator,
        #[token("/")]
        DivOperator,
        #[token("[")]
        OpenSquareBracket,
        #[token("]")]
        CloseSquareBracket,
        #[token("| READ")]
        READ,
        #[token("| WHERE")]
        WHERE,
        #[token("| EXTEND")]
        EXTEND,
        #[regex(r#""([A-Za-z0-9])+""#)]
        Indentity,
        #[regex(r#"[0-9]+"#)]
        Integer,
        #[regex(r#"[0-9]+.[0-9]+"#)]
        Float,
        #[regex(r#"\n"#)]
        NewLine,
        EOF,
        #[error]
        #[regex(r#"[\n\t\s]"#, logos::skip)]
        Error,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn lexer_test() {
        let mut lex = <lexer::Token as logos::Logos>::lexer(r#"["Foo"]"#);

        while let Some(token) = lex.next() {
            dbg!(token);
        }
    }
}
