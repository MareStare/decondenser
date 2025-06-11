use super::{TokenKind, TokenTree};
use crate::error::{Error, Result};
use std::str::Chars;

pub(crate) struct Cursor<'a> {
    /// Total number of bytes in the input.
    bytes: u32,
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &'a str) -> Result<Cursor<'a>> {
        let len = input.len();

        let Ok(bytes) = len.try_into() else {
            return Err(Error::new(format!(
                "Input string exceeds the limit of {} bytes. \
                Input length: {len} bytes",
                u32::MAX,
            )));
        };

        Ok(Cursor {
            bytes,
            chars: input.chars(),
        })
    }

    pub(crate) fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub(crate) fn peek_str(&self) -> &str {
        self.chars.as_str()
    }

    pub(crate) fn offset(&self) -> u32 {
        let remaining = self.chars.as_str().len();

        // We are sure `remaining` is within the limit due to the check in `new`
        self.bytes - remaining as u32
    }

    pub(crate) fn next(&mut self) -> Option<char> {
        self.chars.next()
    }
}
