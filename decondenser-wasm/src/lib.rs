/// The bindings are generated via `cargo component`:
/// <https://github.com/bytecodealliance/cargo-component>
#[allow(warnings)]
#[rustfmt::skip]
#[doc(hidden)]
mod bindings;

struct Component;

bindings::export!(Component with_types_in bindings);

impl bindings::Guest for Component {
    fn unescape(input: String) -> String {
        decondenser::unescape(&input)
    }

    fn decondense(input: String, indent: String) -> String {
        decondenser::decondense(&input, &indent)
    }
}
