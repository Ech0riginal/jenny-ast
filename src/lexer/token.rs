use crate::symbol::Symbol;

use rust_decimal::Decimal;
use tracing::debug;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Metadata
    Noop(String),
    Newline,
    Eof,
    // Variable assignment
    Let,        // let
    Assignment,
    Identifier(String), // carries a variable name
    // Logic
    If,
    Else,
    // Symbols (not to be confused with a market symbol)
    Symbol(Symbol),     // Denoted by '$'
    Decimal(Decimal),      // Denoted by '@'
    Integer(usize),
    Order,
    Buy,
    Sell,
    // Unary operations
    Show,
    History,
    // Binary operations
    Less,
    Equal,
    Greater,
    // Operations
    Create,
    Delete,
    Open,
    Closed,
    For,
    // Triggers
    Monitor,
    Executes,
    Expires,
}

impl From<Decimal> for Token {
    fn from(d: Decimal) -> Self {
        Token::Decimal(d)
    }
}

impl From<&'_ str> for Token {
    fn from(data: &'_ str) -> Self {
        let data = if data.ends_with(';') && data != ";" {
            &data[0..data.len() - 1]
        } else {
            &data
        };

        let t = match data {
            "if" =>                         Token::If,
            "else" =>                       Token::Else,
            "order" | "orders" =>           Token::Order,
            "show" =>                       Token::Show,
            "history" =>                    Token::History,
            "for" =>                        Token::For,
            "<" =>                          Token::Less,
            "==" =>                         Token::Equal,
            ">" =>                          Token::Greater,
            "buy" =>                        Token::Buy,
            "sell" =>                       Token::Sell,
            "open" =>                       Token::Open,
            "closed" =>                     Token::Closed,
            "create" =>                     Token::Create,
            "delete" | "close" =>           Token::Delete,
            "executes" =>                   Token::Executes,
            "expires" =>                    Token::Expires,
            "monitor" | "monitors" =>       Token::Monitor,
            "let" =>                        Token::Let,
            "=" =>                          Token::Assignment,
            "\n" =>                         Token::Newline,
            ";" =>                          Token::Eof,
            s if s.starts_with("$") => {
                let s = crate::symbol::Symbol::from(&s[1..]);
                Token::Symbol(s)
            },
            string if string.starts_with("*") => {
                Token::Identifier(string.to_string())
            }
            string => if let Ok(i) = string.parse::<usize>() {
                Token::Integer(i)
            } else if let Ok(d) = Decimal::from_str(string) {
                Token::Decimal(d)
            } else {
                Token::Noop(string.to_string())
            },
        };

        #[cfg(debug_assertions)]
        debug!("'{}' generated {:?}'", data, t);

        t
    }
}

#[cfg(test)]
fn harness(d: &'_ str) -> Token { Token::from(d) }

macro_rules! lexer_test {
    (failure: $name:ident, $func:ident, $src:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            let func = $func;

            let got = func(src);
            assert!(got.is_err(), "{:?} should be an error", got);
        }
    };
    (success: $name:ident, $func:ident, $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            // let src: &str = $src;
            // let func = $func;
            // let got = func(src);
            assert_eq!($func($src), $should_be, "Input was {:?}", $src);
        }
    };
}
// Thank you https://michael-f-bryan.github.io/static-analyser-in-rust/book/lex.html#the-main-tokenizer-function
lexer_test!(success: tokenize_if, harness, "if" => Token::If);
lexer_test!(success: tokenize_else, harness, "else" => Token::Else);
lexer_test!(success: tokenize_symbol, harness, "$BTC" => Token::Symbol(Symbol::BTC));
lexer_test!(success: tokenize_decimal, harness, "3.50" => Token::Decimal(rust_decimal_macros::dec!(3.50)));
lexer_test!(success: tokenize_integer, harness, "420" => Token::Integer(420));
lexer_test!(success: tokenize_show, harness, "show" => Token::Show);
lexer_test!(success: tokenize_history, harness, "history" => Token::History);
lexer_test!(success: tokenize_less_than, harness, "<" => Token::Less);
lexer_test!(success: tokenize_equal_to, harness, "==" => Token::Equal);
lexer_test!(success: tokenize_greater_than, harness, ">" => Token::Greater);
lexer_test!(success: tokenize_let, harness, "let" => Token::Let);
lexer_test!(success: tokenize_assignment, harness, "=" => Token::Assignment);
lexer_test!(success: tokenize_open, harness, "open" => Token::Open);
lexer_test!(success: tokenize_create, harness, "create" => Token::Create);
lexer_test!(success: tokenize_order, harness, "order" => Token::Order);
lexer_test!(success: tokenize_for, harness, "for" => Token::For);
lexer_test!(success: tokenize_buy, harness, "buy" => Token::Buy);
lexer_test!(success: tokenize_sell, harness, "sell" => Token::Sell);
lexer_test!(success: tokenize_closed, harness, "closed" => Token::Closed);
lexer_test!(success: tokenize_close, harness, "close" => Token::Delete);
lexer_test!(success: tokenize_delete, harness, "delete" => Token::Delete);
lexer_test!(success: tokenize_executes, harness, "executes" => Token::Executes);
lexer_test!(success: tokenize_expires, harness, "expires" => Token::Expires);
lexer_test!(success: tokenize_monitor, harness, "monitor" => Token::Monitor);
lexer_test!(success: tokenize_newline, harness, "\n" => Token::Newline);
lexer_test!(success: tokenize_eof, harness, ";" => Token::Eof);
lexer_test!(success: tokenize_noop, harness, "hypnotoad" => Token::Noop("hypnotoad".to_string()));