use crate::parse::{Token, TokenizeParams, tokenize};
use crate::{LanguageConfig, Result, Str};
use std::path::PathBuf;

pub struct DecondenseParams<'a> {
    pub input: &'a str,
    pub lang: &'a LanguageConfig<'a>,
}

pub struct DecondenseOutput {
    pub output: String,
}

/// Format any text according to brackets nesting and other simple rules.
#[must_use = "this is a pure function; ignoring its result is definitely a bug"]
pub fn decondense(params: &DecondenseParams<'_>) -> Result<DecondenseOutput> {
    let tokens = tokenize(TokenizeParams {
        input: params.input,
        lang: params.lang,
    })?;

    let mut doc = allman::Doc::new();

    for token in tokens {
        match token {
            Token::Whitespace { start } => doc.tag(allman::Tag::Space),
            Token::Group(group) => todo!(),
            Token::Quoted(quoted) => todo!(),
            Token::Raw { start } => todo!(),
            Token::Punct { start } => todo!(),
        }
    }
}

// TODO: add tests
// suite("Extension Test Suite", () => {
//     vscode.window.showInformationMessage("Start all tests.");

//     test("Smoke test", () => {
//         const text = `Test {key1:"value1","key2": "value2"}[[1],[2],[3],]{"key1":"value1","key2":[123]}`;
//         const expected = `\
// Test {
//     key1:"value1",
//     "key2": "value2"
// }[
//     [
//         1
//     ],
//     [
//         2
//     ],
//     [
//         3
//     ]
// ]{
//     "key1":"value1",
//     "key2":[
//         123
//     ]
// }`;
//         const output = decondenser.decondense(text, "    ");
//         assert.strictEqual(expected, output);
//     });

//     test("Escape test", () => {
//         const text = `{"key": "\\n\\r\\t"}`;
//         const output = decondenser.decondense(text, "    ");
//         assert.strictEqual(`{\n    "key": "\\n\\r\\t"\n}`, output);
//     });

//     test("Unescape test \\n", () => {
//         const text = `{"key": "val\nue"}`;
//         const output = formatUnescapedText(text);
//         assert.strictEqual(`{\n    "key": "val\nue"\n}`, output);
//     });

//     test("Unescape test \\r", () => {
//         const text = `{"key": "val\rue"}`;
//         const output = formatUnescapedText(text);
//         assert.strictEqual(`{\n    "key": "val\rue"\n}`, output);
//     });

//     test("Unescape test \\t", () => {
//         const text = `{"key": "val\tue"}`;
//         const output = formatUnescapedText(text);
//         assert.strictEqual(`{\n    "key": "val\tue"\n}`, output);
//     });
// });
