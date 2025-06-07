//! TODO: add crate-level docs

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

/// Unescape the string by replacing the escape sequences with their actual characters.
/// Supported escape sequences are:
/// - `\n` for newline
/// - `\r` for carriage return
/// - `\t` for tab
/// - `\\` for backslash
///
/// Other `\*` sequences will be left as is.
#[must_use = "this is a pure function; ignoring its result is definitely a bug"]
pub fn unescape(input: &str) -> String {
    let mut output = String::new();
    let mut backslash = false;

    for c in input.chars() {
        if backslash {
            match c {
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                't' => output.push('\t'),
                '\\' => output.push('\\'),
                _ => output.extend(['\\', c]),
            }
            backslash = false;
            continue;
        }

        if c == '\\' {
            backslash = true;
            continue;
        }

        output.push(c);
    }

    output
}

/// The main function of this crate that decondenses the input according to the
/// nesting of the following groups of characters: `()`, `[]`, `{}`.
#[must_use = "this is a pure function; ignoring its result is definitely a bug"]
pub fn decondense(input: &str, indent: &str) -> String {
    fn push_after_comma_and_indent(
        output: Vec<String>,
        indent_level: i32,
        character: char,
        indentation: &str,
    ) -> Vec<String> {
        let mut result = output;
        result.push(",\n".to_string());
        result.push(indentation.repeat(indent_level.abs() as usize));
        result.push(character.to_string());
        result
    }

    fn create_indentation(indent_level: i32, indentation: &str) -> String {
        indentation.repeat(indent_level.abs() as usize)
    }

    let mut state = State::Start;
    let mut output = Vec::new();
    let mut indent_level = 0;

    for c in input.chars() {
        match c {
            '\\' => match state {
                State::Start => output.push("\\".to_string()),
                State::String => {
                    state = State::Escape;
                    output.push("\\".to_string());
                }
                State::Escape => {
                    state = State::String;
                    output.push("\\".to_string());
                }
                State::AfterComma => {
                    state = State::Start;
                    output = push_after_comma_and_indent(output, indent_level, '\\', indent);
                }
            },

            '"' => match state {
                State::Start => {
                    state = State::String;
                    output.push("\"".to_string());
                }
                State::String => {
                    state = State::Start;
                    output.push("\"".to_string());
                }
                State::Escape => {
                    state = State::String;
                    output.push("\"".to_string());
                }
                State::AfterComma => {
                    state = State::String;
                    output = push_after_comma_and_indent(output, indent_level, '"', indent);
                }
            },

            ' ' => match state {
                State::Start | State::String => output.push(" ".to_string()),
                State::Escape => {
                    state = State::String;
                    output.push(" ".to_string());
                }
                State::AfterComma => {}
            },

            ',' => match state {
                State::Start => state = State::AfterComma,
                State::String => output.push(",".to_string()),
                State::Escape => {
                    state = State::String;
                    output.push(",".to_string());
                }
                State::AfterComma => output.push(",".to_string()),
            },

            '(' | '[' | '{' => match state {
                State::Start => {
                    indent_level += 1;
                    output.push(c.to_string());
                    output.push("\n".to_string());
                    output.push(create_indentation(indent_level, indent));
                }
                State::String => output.push(c.to_string()),
                State::Escape => {
                    state = State::String;
                    output.push(c.to_string());
                }

                State::AfterComma => {
                    state = State::Start;

                    output = push_after_comma_and_indent(output, indent_level, c, indent);
                    indent_level += 1;
                    output.push("\n".to_string());
                    output.push(create_indentation(indent_level, indent));
                }
            },

            ')' | ']' | '}' => match state {
                State::Start => {
                    indent_level -= 1;
                    output.push("\n".to_string());
                    output.push(create_indentation(indent_level, indent));
                    output.push(c.to_string());
                }
                State::String => output.push(c.to_string()),
                State::Escape => {
                    state = State::String;
                    output.push(c.to_string());
                }
                State::AfterComma => {
                    state = State::Start;
                    indent_level -= 1;
                    output.push("\n".to_string());
                    output.push(create_indentation(indent_level, indent));
                    output.push(c.to_string());
                }
            },

            _ => match state {
                State::Start => output.push(c.to_string()),
                State::String => output.push(c.to_string()),
                State::Escape => {
                    state = State::String;
                    output.push(c.to_string());
                }
                State::AfterComma => {
                    state = State::Start;
                    output = push_after_comma_and_indent(output, indent_level, c, indent);
                }
            },
        }
    }

    output.join("")
}
