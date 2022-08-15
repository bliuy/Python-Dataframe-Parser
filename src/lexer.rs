pub mod lexer {

    use logos::{self, Lexer};

    fn capture_string(lex: &mut Lexer<Token>) -> Option<String> {
        let captured_string = lex.slice();
        Some(captured_string.to_string())
    }
    fn capture_float(lex: &mut Lexer<Token>) -> Option<f32> {
        let captured_string = lex.slice();
        let captured_float = captured_string.parse();
        captured_float.ok()
    }
    fn capture_int(lex: &mut Lexer<Token>) -> Option<i32> {
        let captured_string = lex.slice();
        let captured_int = captured_string.parse();
        captured_int.ok()
    }

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
        #[token("=")]
        EqualsOperator,
        #[token("[")]
        OpenSquareBracket,
        #[token("]")]
        CloseSquareBracket,
        #[token("(")]
        OpenBracket,
        #[token(")")]
        CloseBracket,
        #[token(">")]
        GreaterThan,
        #[token("<")]
        LessThan,
        #[token(">=")]
        GreaterThanEqualsTo,
        #[token("<=")]
        LessThanEqualsTo,
        #[token(r#"""#)]
        QuotationMark,
        #[token("| READ")]
        READ,
        #[token("| WHERE")]
        WHERE,
        #[token("| EXTEND")]
        EXTEND,
        #[token("isnotnull")]
        ISNOTNULL,
        #[token("isnull")]
        ISNULL,
        #[regex(r#"([A-z]+[0-9]*)"#, capture_string)]
        Identity(String),
        #[regex(r#"[0-9]+"#, capture_int)]
        Integer(i32),
        #[regex(r#"[0-9]+.[0-9]+"#, capture_float)]
        Float(f32),
        // #[regex(r#"\n"#)]
        // NewLine,sd
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
        let input = r#"
        sourceTable
        | READ csv
        | WHERE ["foo bar"] > 5
        "#;
        let lex = <lexer::Token as logos::Logos>::lexer(input);

        for token in lex {
            dbg!(token);
        }
    }
}
