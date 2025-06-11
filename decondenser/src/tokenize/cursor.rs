use crate::error::{Error, Result};
use std::str::Chars;

pub(super) struct Cursor<'a> {
    /// Total number of bytes in the input.
    bytes: u32,
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(input: &'a str) -> Result<Cursor<'a>> {
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

    pub(super) fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub(super) fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub(super) fn byte_offset(&self) -> u32 {
        let remaining = self.chars.as_str().len();

        // We are sure `remaining` is within the limit due to the check in `new`
        self.bytes - remaining as u32
    }

    pub(super) fn strip_prefix(&mut self, prefix: &str) -> Option<u32> {
        let stripped = self.chars.as_str().strip_prefix(prefix)?;

        let start = self.byte_offset();
        self.chars = stripped.chars();

        Some(start)
    }
}
