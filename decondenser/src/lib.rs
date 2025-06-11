//! The API of this crate is not stable yet! It's not yet intended for public use.
#![allow(warnings)]

#[cfg(test)]
mod tests;

mod decondense;
mod error;
mod str;
mod tokenize;
mod unescape;

pub use self::decondense::*;
pub use self::str::*;
pub use self::unescape::*;

pub struct EscapeSpec<'a> {
    pub escaped: Str<'a>,
    pub unescaped: Str<'a>,
}

pub struct GroupSpec<'a> {
    pub opening: Str<'a>,
    pub closing: Str<'a>,
}

pub struct QuoteSpec<'a> {
    pub opening: Str<'a>,
    pub closing: Str<'a>,
    pub escapes: &'a [EscapeSpec<'a>],
}

pub struct LanguageSpec<'a> {
    pub groups: &'a [GroupSpec<'a>],
    pub quotes: &'a [QuoteSpec<'a>],
    pub puncts: &'a [Str<'a>],
}

impl LanguageSpec<'_> {
    pub fn generic() -> LanguageSpec<'static> {
        const {
            LanguageSpec {
                groups: &[
                    GroupSpec {
                        opening: Str::borrowed("("),
                        closing: Str::borrowed(")"),
                    },
                    GroupSpec {
                        opening: Str::borrowed("["),
                        closing: Str::borrowed("]"),
                    },
                    GroupSpec {
                        opening: Str::borrowed("{"),
                        closing: Str::borrowed("}"),
                    },
                    GroupSpec {
                        opening: Str::borrowed("<"),
                        closing: Str::borrowed(">"),
                    },
                ],
                quotes: &[
                    QuoteSpec {
                        opening: Str::borrowed("\""),
                        closing: Str::borrowed("\""),
                        escapes: &[
                            EscapeSpec {
                                escaped: Str::borrowed("\\n"),
                                unescaped: Str::borrowed("\n"),
                            },
                            EscapeSpec {
                                escaped: Str::borrowed("\\r"),
                                unescaped: Str::borrowed("\r"),
                            },
                            EscapeSpec {
                                escaped: Str::borrowed("\\t"),
                                unescaped: Str::borrowed("\t"),
                            },
                            EscapeSpec {
                                escaped: Str::borrowed("\\\\"),
                                unescaped: Str::borrowed("\\"),
                            },
                            EscapeSpec {
                                escaped: Str::borrowed("\\\""),
                                unescaped: Str::borrowed("\""),
                            },
                        ],
                    },
                    QuoteSpec {
                        opening: Str::borrowed("'"),
                        closing: Str::borrowed("'"),
                        escapes: &[
                            EscapeSpec {
                                escaped: Str::borrowed("\\n"),
                                unescaped: Str::borrowed("\n"),
                            },
                            EscapeSpec {
                                escaped: Str::borrowed("\\r"),
                                unescaped: Str::borrowed("\r"),
                            },
                            EscapeSpec {
                                escaped: Str::borrowed("\\t"),
                                unescaped: Str::borrowed("\t"),
                            },
                            EscapeSpec {
                                escaped: Str::borrowed("\\\\"),
                                unescaped: Str::borrowed("\\"),
                            },
                            EscapeSpec {
                                escaped: Str::borrowed("\\'"),
                                unescaped: Str::borrowed("'"),
                            },
                        ],
                    },
                ],
                puncts: &[
                    Str::borrowed(","),
                    Str::borrowed(";"),
                    Str::borrowed(":"),
                    Str::borrowed("="),
                ],
            }
        }
    }
}
