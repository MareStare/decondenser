mod cursor;

use crate::{EscapeConfig, GroupConfig, LanguageConfig, QuoteConfig, Str};

use crate::error::Result;
use cursor::Cursor;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::mem;

#[derive(Debug)]
pub(crate) enum AstNode {
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

    /// The first node contains the start offset of the content of the group,
    /// unless the group is empty.
    pub(crate) content: Vec<AstNode>,

    /// Offset of the closing delimiter. Can be `None` if the group is not closed
    /// (probably a malformed input).
    pub(crate) closing: Option<u32>,
}

pub(crate) struct ParseParams<'a> {
    pub(crate) input: &'a str,
    pub(crate) lang: &'a LanguageConfig<'a>,
}

pub(crate) fn parse(params: ParseParams<'_>) -> Result<Vec<AstNode>> {
    let mut lexer = Parser {
        lang: params.lang,
        cursor: Cursor::new(params.input)?,
        output: Vec::new(),
    };

    lexer.parse(None);

    Ok(lexer.output)
}

struct Parser<'a> {
    lang: &'a LanguageConfig<'a>,
    cursor: Cursor<'a>,
    output: Vec<AstNode>,
}

impl Parser<'_> {
    fn parse(&mut self, terminator: Option<&Str<'_>>) -> Option<u32> {
        while self.cursor.peek().is_some() {
            self.whitespace();

            if let Some(start) = terminator.and_then(|term| self.cursor.strip_prefix(term)) {
                return Some(start);
            }

            let group_cfg = self.lang.groups.iter().find_map(|group_cfg| {
                Some((self.cursor.strip_prefix(&group_cfg.opening)?, group_cfg))
            });

            if let Some((opening, group_cfg)) = group_cfg {
                self.parse_group(opening, group_cfg);
                continue;
            }

            let quote_cfg = self.lang.quotes.iter().find_map(|quote_cfg| {
                Some((self.cursor.strip_prefix(&quote_cfg.opening)?, quote_cfg))
            });

            if let Some((opening, quote_cfg)) = quote_cfg {
                self.parse_quoted(opening, quote_cfg);
                continue;
            }

            let punct = self
                .lang
                .puncts
                .iter()
                .find_map(|punct| self.cursor.strip_prefix(punct));

            if let Some(start) = punct {
                self.output.push(AstNode::Punct { start });
                continue;
            }

            if !matches!(self.output.last(), Some(AstNode::Raw { .. })) {
                let start = self.cursor.byte_offset();
                self.output.push(AstNode::Raw { start });
            }

            self.cursor.next();
        }

        None
    }

    fn parse_group(&mut self, opening: u32, group_cfg: &GroupConfig<'_>) {
        let prev = mem::take(&mut self.output);

        let closing = self.parse(Some(&group_cfg.closing));

        let group = Group {
            opening,
            content: mem::replace(&mut self.output, prev).into(),
            closing,
        };

        self.output.push(AstNode::Group(group));
    }

    fn parse_quoted(&mut self, opening: u32, quote_cfg: &QuoteConfig<'_>) {
        let mut content = vec![];

        let closing = loop {
            let escape = quote_cfg
                .escapes
                .iter()
                .find_map(|escape| self.cursor.strip_prefix(&escape.escaped));

            if let Some(start) = escape {
                content.push(QuotedContent::Escape { start });
                continue;
            }

            if let Some(closing) = self.cursor.strip_prefix(&quote_cfg.closing) {
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

        self.output.push(AstNode::Quoted(quoted));
    }

    fn whitespace(&mut self) {
        if let Some(char) = self.cursor.peek() {
            if !char.is_whitespace() {
                return;
            }
        }

        let start = self.cursor.byte_offset();
        self.output.push(AstNode::Whitespace { start });

        while let Some(char) = self.cursor.peek() {
            if !char.is_whitespace() {
                break;
            }
            self.cursor.next();
        }
    }
}
