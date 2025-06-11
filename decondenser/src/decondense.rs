use std::borrow::Cow;

/// The main function of this crate that decondenses the input according to the
/// nesting of the following groups of characters: `()`, `[]`, `{}`.
#[must_use = "this is a pure function; ignoring its result is definitely a bug"]
pub fn decondense(input: &str, indent: &str) -> String {
    todo!()
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
