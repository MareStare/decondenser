mod cursor;

use crate::{EscapeSpec, GroupSpec, LanguageSpec, QuoteSpec, Str};

use crate::error::Result;
use cursor::Cursor;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::mem;

#[derive(Debug)]
pub(crate) enum TokenTree {
    Whitespace { start: u32 },
    Group(Group),
    Quoted(Quoted),
    Raw { start: u32 },
    Punct { start: u32 },
}

#[derive(Debug)]
pub(crate) struct Quoted {
    /// Offset of the opening quote
    pub(crate) opening: u32,

    /// Offset of the content, which is also the offset of the character that
    /// follows the opening quote. Will be equal to `closing` if the content
    /// is empty.
    pub(crate) content: Vec<QuotedContent>,

    /// Offset of the closing quote. Can be `None` if the quotes are not closed
    /// (probably a malformed input).
    pub(crate) closing: Option<u32>,
}

#[derive(Debug)]
pub(crate) enum QuotedContent {
    Raw { start: u32 },
    Escape { start: u32 },
}

#[derive(Debug)]
pub(crate) struct Group {
    /// The start offset of the opening delimiter
    pub(crate) opening: u32,

    /// The first token contains the start offset of the content of the group,
    /// unless the group is empty.
    pub(crate) content: Vec<TokenTree>,

    /// Offset of the closing delimiter. Can be `None` if the group is not closed
    /// (probably a malformed input).
    pub(crate) closing: Option<u32>,
}

pub(crate) struct TokenizeParams<'a> {
    pub(crate) input: &'a str,
    pub(crate) lang: &'a LanguageSpec<'a>,
}

pub(crate) fn tokenize(params: TokenizeParams<'_>) -> Result<Vec<TokenTree>> {
    let mut lexer = Lexer {
        lang: params.lang,
        cursor: Cursor::new(params.input)?,
        output: Vec::new(),
    };

    lexer.tokenize(None);

    Ok(lexer.output)
}

struct Lexer<'a> {
    lang: &'a LanguageSpec<'a>,
    cursor: Cursor<'a>,
    output: Vec<TokenTree>,
}

impl<'a> Lexer<'a> {
    fn tokenize(&mut self, terminator: Option<&Str<'a>>) -> Option<u32> {
        while self.cursor.peek().is_some() {
            self.whitespace();

            if let Some(start) = terminator.and_then(|term| self.cursor.strip_prefix(term)) {
                return Some(start);
            }

            let group_spec = self.lang.groups.iter().find_map(|group_spec| {
                Some((self.cursor.strip_prefix(&group_spec.opening)?, group_spec))
            });

            if let Some((opening, group_spec)) = group_spec {
                self.tokenize_group(opening, group_spec);
                continue;
            }

            let quote_spec = self.lang.quotes.iter().find_map(|quote_spec| {
                Some((self.cursor.strip_prefix(&quote_spec.opening)?, quote_spec))
            });

            if let Some((opening, quote_spec)) = quote_spec {
                self.tokenize_quoted(opening, quote_spec);
                continue;
            }

            let punct = self
                .lang
                .puncts
                .iter()
                .find_map(|punct| self.cursor.strip_prefix(punct));

            if let Some(start) = punct {
                self.output.push(TokenTree::Punct { start });
                continue;
            }

            if !matches!(self.output.last(), Some(TokenTree::Raw { .. })) {
                let start = self.cursor.byte_offset();
                self.output.push(TokenTree::Raw { start });
            }

            self.cursor.next();
        }

        None
    }

    fn tokenize_group(&mut self, opening: u32, group_spec: &GroupSpec<'a>) {
        let prev = mem::take(&mut self.output);

        let closing = self.tokenize(Some(&group_spec.closing));

        let group = Group {
            opening,
            content: mem::replace(&mut self.output, prev).into(),
            closing,
        };

        self.output.push(TokenTree::Group(group));
    }

    fn tokenize_quoted(&mut self, opening: u32, quote_spec: &QuoteSpec<'a>) {
        let mut content = vec![];

        let closing = loop {
            let escape = quote_spec
                .escapes
                .iter()
                .find_map(|escape| self.cursor.strip_prefix(&escape.escaped));

            if let Some(start) = escape {
                content.push(QuotedContent::Escape { start });
                continue;
            }

            if let Some(closing) = self.cursor.strip_prefix(&quote_spec.closing) {
                break Some(closing);
            }

            if !matches!(content.last(), Some(QuotedContent::Raw { .. })) {
                let start = self.cursor.byte_offset();
                content.push(QuotedContent::Raw { start });
            }

            self.cursor.next();
        };

        let quoted = Quoted {
            opening,
            content,
            closing,
        };

        self.output.push(TokenTree::Quoted(quoted));
    }

    fn whitespace(&mut self) {
        if let Some(char) = self.cursor.peek() {
            if !char.is_whitespace() {
                return;
            }
        }

        let start = self.cursor.byte_offset();
        self.output.push(TokenTree::Whitespace { start });

        while let Some(char) = self.cursor.peek() {
            if !char.is_whitespace() {
                break;
            }
            self.cursor.next();
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    // Start state is assigned:
    // - at the start of parsing
    // - when met \ after comma: key: value,\
    // - when met " end of the string
    // - when met open bracket after comma: [[1],[2]]
    // - when met closing bracket after comma (no trailing comma added): [[1,2,]]
    // - when met any character after comma: {key1: value1,key2: [1,2,3]}
    Start,
    // String state is assigned:
    // - when met " start of the string: {key: "abc"}
    // - when met \ is escape state: "\\abc"
    // - when met " after comma: ["value1","value2", "value3"]
    // - when met any character in escape state: "\X"
    String,
    // Escape state is assigned:
    // - when met \ in string: "\"
    Escape,
    // AfterComma state is assigned:
    // - when met , in start state: [1,2,3,] | {key: value,}
    AfterComma,
}
