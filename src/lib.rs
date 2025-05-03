#![feature(portable_simd)]

mod converter;
mod parser;
mod parser_new;
mod parser_third;
mod types;

pub use converter::TSONStringifier;
pub use parser::Parser;
pub use parser_new::Parse;
pub use parser_third::Parse as ParseThird;
pub use types::{Map, Res, TsonValue};

pub fn parse<'a>(input: &'a str) -> Res<TsonValue> {
    Parser::parse(input)
}

pub fn parse_new<'a>(input: &'a str) -> Res<TsonValue> {
    parser_new::Parser::from_str(input).deserialize()
}

pub fn parse_third<'a>(input: &'a str) -> Res<TsonValue> {
    parser_third::Parser::from_str(input).deserialize()
}

pub fn stringify<'a>(value: &'a TsonValue, pretty: bool) -> String {
    TSONStringifier::stringify(value, pretty)
}
