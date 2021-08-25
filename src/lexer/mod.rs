pub mod whitespace;
pub mod token;
mod tokenizer;

use crate::lexer::token::Token;

use anyhow::Result;

// https://michael-f-bryan.github.io/static-analyser-in-rust/book/lex.html

#[allow(dead_code)]
pub fn tokenize(data: &str) -> Result<Vec<Token>> {
    let mut tokenizer = tokenizer::Tokenizer::new(data);
    let mut tokens = Vec::with_capacity(8);

    while let Some(token) = tokenizer.next() {
        tokens.push(token);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::Symbol;

    #[test]
    fn tokenize_basic_expression() {
        let data = "create order 66;";
        let should_be = vec![
            Token::Create,
            Token::Order,
            Token::Integer(66),
            Token::Eof
        ];

        let tokens = tokenize(data).unwrap();

        assert_eq!(should_be, tokens);

    }

    #[test]
    fn tokenize_complex_expression() {
        let data = "create buy order 100 $BTC for 2.5;";
        let should_be = vec![
            Token::Create,
            Token::Buy,
            Token::Order,
            Token::Integer(100),
            Token::Symbol(Symbol::BTC),
            Token::For,
            Token::Decimal(rust_decimal_macros::dec!(2.5)),
            Token::Eof,
        ];

        let tokens = tokenize(data).unwrap();

        assert_eq!(should_be, tokens);
    }

    #[test]
    fn tokenize_variable_assignment() {
        let data = "let furby = buy order 100 $BTC for 2.5; create furby;";
        let should_be = vec![
            Token::Let,
            Token::Noop("furby".to_string()),
            Token::Assignment,
            Token::Buy,
            Token::Order,
            Token::Integer(100),
            Token::Symbol(Symbol::BTC),
            Token::For,
            Token::Decimal(rust_decimal_macros::dec!(2.5)),
            Token::Eof,
            Token::Create,
            Token::Noop("furby".to_string()),
            Token::Eof,
        ];

        let tokens = tokenize(data).unwrap();

        assert_eq!(should_be, tokens);
    }
}

