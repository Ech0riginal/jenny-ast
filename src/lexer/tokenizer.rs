use crate::lexer::{whitespace, Token};

use anyhow::Result;

pub(crate) struct Tokenizer<'t> {
    buffer: &'t str,
}

#[allow(dead_code)]
impl<'t> Tokenizer<'t> {
    pub fn new(buffer: &'t str) -> Self {
        Self { buffer, }
    }

    pub fn next(&mut self) -> Result<Option<Token>> {
        if self.buffer.is_empty() {
            return Ok(None);
        }

        let cursor: usize = whitespace::skip(self.buffer);
        let eot: usize = whitespace::seek(&self.buffer[cursor..]);
        let step = cursor + eot;

        let tok = Token::from(&self.buffer[cursor..step]);

        // Don't discard the eof token
        if self.buffer[cursor..step].ends_with(';') && tok != Token::Eof {
            self.advance(step - 1);
        } else {
            self.advance(step);
        }

        Ok(Some(tok))
    }

    fn advance(&mut self, n: usize) {
        self.buffer = &self.buffer[n..];
    }
}