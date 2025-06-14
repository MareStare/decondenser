use crate::Str;

pub struct EscapeConfig<'a> {
    pub escaped: Str<'a>,
    pub unescaped: Str<'a>,
}

pub struct GroupConfig<'a> {
    pub opening: Str<'a>,
    pub closing: Str<'a>,
}

pub struct QuoteConfig<'a> {
    pub opening: Str<'a>,
    pub closing: Str<'a>,
    pub escapes: &'a [EscapeConfig<'a>],
}

pub struct LanguageConfig<'a> {
    pub groups: &'a [GroupConfig<'a>],
    pub quotes: &'a [QuoteConfig<'a>],
    pub puncts: &'a [Str<'a>],
}

impl LanguageConfig<'_> {
    pub fn generic() -> LanguageConfig<'static> {
        const {
            LanguageConfig {
                groups: &[
                    GroupConfig {
                        opening: Str::borrowed("("),
                        closing: Str::borrowed(")"),
                    },
                    GroupConfig {
                        opening: Str::borrowed("["),
                        closing: Str::borrowed("]"),
                    },
                    GroupConfig {
                        opening: Str::borrowed("{"),
                        closing: Str::borrowed("}"),
                    },
                    GroupConfig {
                        opening: Str::borrowed("<"),
                        closing: Str::borrowed(">"),
                    },
                ],
                quotes: &[
                    QuoteConfig {
                        opening: Str::borrowed("\""),
                        closing: Str::borrowed("\""),
                        escapes: &[
                            EscapeConfig {
                                escaped: Str::borrowed("\\n"),
                                unescaped: Str::borrowed("\n"),
                            },
                            EscapeConfig {
                                escaped: Str::borrowed("\\r"),
                                unescaped: Str::borrowed("\r"),
                            },
                            EscapeConfig {
                                escaped: Str::borrowed("\\t"),
                                unescaped: Str::borrowed("\t"),
                            },
                            EscapeConfig {
                                escaped: Str::borrowed("\\\\"),
                                unescaped: Str::borrowed("\\"),
                            },
                            EscapeConfig {
                                escaped: Str::borrowed("\\\""),
                                unescaped: Str::borrowed("\""),
                            },
                        ],
                    },
                    QuoteConfig {
                        opening: Str::borrowed("'"),
                        closing: Str::borrowed("'"),
                        escapes: &[
                            EscapeConfig {
                                escaped: Str::borrowed("\\n"),
                                unescaped: Str::borrowed("\n"),
                            },
                            EscapeConfig {
                                escaped: Str::borrowed("\\r"),
                                unescaped: Str::borrowed("\r"),
                            },
                            EscapeConfig {
                                escaped: Str::borrowed("\\t"),
                                unescaped: Str::borrowed("\t"),
                            },
                            EscapeConfig {
                                escaped: Str::borrowed("\\\\"),
                                unescaped: Str::borrowed("\\"),
                            },
                            EscapeConfig {
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
