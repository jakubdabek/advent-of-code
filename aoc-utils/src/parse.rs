const LEXICAL_HEX_FORMAT: u128 = lexical::format::NumberFormatBuilder::hexadecimal();

pub fn lexical_parse_hex<T: lexical::FromLexicalWithOptions>(s: &str) -> lexical::Result<T> {
    lexical::parse_with_options::<_, _, LEXICAL_HEX_FORMAT>(s, &Default::default())
}
