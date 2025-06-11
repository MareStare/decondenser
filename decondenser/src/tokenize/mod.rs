mod cursor;

use crate::error::Result;
use cursor::Cursor;
use std::borrow::Cow;
use std::marker::PhantomData;

type Str<'a> = Cow<'a, str>;

struct TokenTree<'a> {
    kind: TokenKind<'a>,

    /// Index of the token start in the input string. We assume the input fits
    /// into the range of a `u32`. Hopefully no one needs to decondense a string
    /// longer than 4GB.
    start: u32,

    /// Capture the lifetime of the input string since the [`Token`] technically
    /// references a substring in the input string, so the [`Token`] makes no
    /// sense without the input string, and mixing up the input string with some
    /// other string would be a bug.
    _phantom: PhantomData<&'a ()>,
}

impl<'a> TokenTree<'a> {
    fn new(kind: TokenKind<'a>, start: u32) -> Self {
        TokenTree {
            kind,
            start,
            _phantom: PhantomData,
        }
    }
}

enum TokenKind<'a> {
    Whitespace,
    Nesting(Vec<TokenTree<'a>>),
    Quoted,
}

struct Nesting<'a> {
    tokens: Box<[TokenTree<'a>]>,

    /// The start offset of the end delimiter.
    end: u32,
}

pub(crate) struct TokenizeParams<'a> {
    input: &'a str,
    escape_char: char,
    nestings: &'a [(Str<'a>, Str<'a>)],
    quotes: &'a [(Str<'a>, Str<'a>)],
}

pub(crate) fn tokenize(params: TokenizeParams<'_>) -> Result<Vec<TokenTree<'_>>> {
    let mut lexer = Lexer {
        escape_char: params.escape_char,
        nestings: params.nestings,
        quotes: params.quotes,
        cursor: Cursor::new(params.input)?,
        output: Vec::new(),
    };

    Ok(lexer.tokenize())
}

struct Lexer<'a> {
    escape_char: char,
    nestings: &'a [(Str<'a>, Str<'a>)],
    quotes: &'a [(Str<'a>, Str<'a>)],
    cursor: Cursor<'a>,
    output: Vec<TokenTree<'a>>,
}

impl<'a> Lexer<'a> {
    fn tokenize(mut self) -> Vec<TokenTree<'a>> {
        while let Some(char) = self.cursor.peek() {
            self.whitespace();

            let remainder = self.cursor.peek_str();

            let delim = self
                .nestings
                .iter()
                .find(|(start, end)| remainder.starts_with(start.as_ref()));

            if let Some(delim) = delim {
                let tokens = self.tokenize_nesting(delim);
            }
        }

        self.output
    }

    fn tokenize_nesting(&mut self, (start, end): &(Str<'a>, Str<'a>)) -> Nesting<'a> {
        // Consume the start delimiter
        for _ in start.chars() {
            self.cursor.next();
        }

        let mut output = Vec::new();

        while let Some(char) = self.cursor.peek() {
            if char == end.chars().next().unwrap() {
                // Consume the end delimiter
                for _ in end.chars() {
                    self.cursor.next();
                }
                break;
            } else {
                // Recursively tokenize the inner content
                self.whitespace();
            }
        }
    }

    fn whitespace(&mut self) {
        if let Some(char) = self.cursor.peek() {
            if !char.is_whitespace() {
                return;
            }
        }

        self.push_token(TokenKind::Whitespace);

        while let Some(char) = self.cursor.peek() {
            if !char.is_whitespace() {
                break;
            }
            self.cursor.next();
        }
    }

    fn push_token(&mut self, kind: TokenKind<'a>) {
        self.output.push(TokenTree::new(kind, self.cursor.offset()));
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
