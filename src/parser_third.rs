use crate::types::{Map, Res, TsonValue};

const SPACE: u8 = 0x20; // ' '
const COMMA: u8 = 0x2C; // ','
const TAB: u8 = 0x09; // '\t'
const CR: u8 = 0x0D; // '\r'
const LF: u8 = 0x0A; // '\n'
const QUOTE: u8 = 0x22; // '"'
const BACKSLASH: u8 = 0x5C; // '\\'

const BRACKET_OPEN: u8 = 0x5B; // '['
const BRACKET_CLOSE: u8 = 0x5D; // ']'

const BRACE_OPEN: u8 = 0x7B; // '{'
const BRACE_CLOSE: u8 = 0x7D; // '}'

const LESS_THAN: u8 = 0x3C; // '<'

const HASH: u8 = 0x23; // '#'
const EQUAL: u8 = 0x3D; // '='
const QUESTION: u8 = 0x3F; // '?'
const AVERAGE: u8 = 0x7E; // '~'
const MINUS: u8 = 0x2D; // '-'
const PLUS: u8 = 0x2B; // '+'

pub struct Parser<'a> {
    chars: &'a [u8],
    index: usize,
    length: usize,
}

impl<'a> Parser<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Self {
            chars: input.as_bytes(),
            index: 0,
            length: input.len(),
        }
    }
}

macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}

impl<'a> Parser<'a> {
    fn in_bounds(&self) -> bool {
        self.index < self.length
    }

    fn peek(&self) -> Option<u8> {
        if self.in_bounds() {
            Some(self.chars[self.index])
        } else {
            None
        }
    }

    fn next(&mut self) -> Option<u8> {
        if self.in_bounds() {
            let item = self.chars[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

    fn skip_n(&mut self, n: usize) {
        self.index += n;
    }

    fn take_while<const N: usize>(&mut self, predicate: impl Fn(u8) -> bool) -> String {
        let mut result = String::with_capacity(N);
        while let Some(char) = self.next() {
            if predicate(char) {
                result.push(char as char);
            } else {
                self.index -= 1;
                break;
            }
        }
        result
    }

    fn skip_while(&mut self, predicate: impl Fn(u8) -> bool) {
        while let Some(char) = self.next() {
            if !predicate(char) {
                self.index -= 1;
                break;
            }
        }
    }
}

pub enum ValueNameOptional {
    Named(String, TsonValue),
    Unnamed(TsonValue),
}

pub type ValueNameRequired = (String, TsonValue);

pub type Result<T> = core::result::Result<T, String>;

macro_rules! some_or_error {
    ($e:expr) => {
        match $e {
            Some(value) => Ok(value),
            None => Err("Unexpected end of input".to_string()),
        }
    };
}

pub trait Parse {
    fn skip_whitespace(&mut self);
    fn deserialize(&mut self) -> Result<TsonValue>;
    fn de_bool(&mut self) -> Result<TsonValue>;
    fn de_int(&mut self) -> Result<TsonValue>;
    fn de_float(&mut self) -> Result<TsonValue>;
    fn de_string(&mut self) -> Result<TsonValue>;
    fn de_array(&mut self) -> Result<TsonValue>;
    fn de_object(&mut self) -> Result<TsonValue>;
    fn de_type_specifier(&mut self) -> Result<TsonValue>;
    fn de_name(&mut self) -> Result<Option<String>>;
    fn de_name_exact(&mut self) -> Result<String>;
    fn de_value_name_required(&mut self) -> Result<ValueNameRequired>;
    fn de_value_name_optional(&mut self) -> Result<ValueNameOptional>;
    fn de_value(&mut self) -> Result<TsonValue>;
}

impl<'a> Parse for Parser<'a> {
    fn skip_whitespace(&mut self) {
        self.skip_while(|c| matches!(c, SPACE | TAB | CR | LF | COMMA));
    }

    fn deserialize(&mut self) -> Result<TsonValue> {
        self.skip_whitespace();
        let result = tri!(self.de_value_name_optional());

        match result {
            ValueNameOptional::Named(name, value) => {
                let mut map = Map::new();
                map.insert(name, value);
                Ok(TsonValue::Object(map))
            }
            ValueNameOptional::Unnamed(value) => Ok(value),
        }
    }

    fn de_value_name_optional(&mut self) -> Result<ValueNameOptional> {
        let name = tri!(self.de_name());
        let value = tri!(self.de_value());
        match name {
            Some(name) => Ok(ValueNameOptional::Named(name, value)),
            None => Ok(ValueNameOptional::Unnamed(value)),
        }
    }

    fn de_value_name_required(&mut self) -> Result<(String, TsonValue)> {
        let name = tri!(self.de_name_exact());

        let value = tri!(self.de_value());

        Ok((name, value))
    }

    fn de_value(&mut self) -> Result<TsonValue> {
        match tri!(some_or_error!(self.next())) {
            BRACE_OPEN => Ok(tri!(self.de_object())),
            BRACKET_OPEN => Ok(tri!(self.de_array())),
            LESS_THAN => Ok(tri!(self.de_type_specifier())),
            QUOTE => Ok(tri!(self.de_string())),
            HASH => Ok(tri!(self.de_int())),
            EQUAL => Ok(tri!(self.de_float())),
            QUESTION => Ok(tri!(self.de_bool())),
            AVERAGE => Ok(TsonValue::Null),
            c => Err(format!(
                "Unexpected character: {} at position {}",
                c as char, self.index
            )),
        }
    }

    fn de_array(&mut self) -> Res<TsonValue> {
        self.skip_whitespace();
        let mut array = Vec::new();
        while peek_not_bracket_close(self.peek()) {
            let result = self.de_value_name_optional()?;

            match result {
                ValueNameOptional::Named(name, value) => {
                    let mut item_map = Map::new();
                    item_map.insert(name, value);
                    array.push(TsonValue::Object(item_map));
                }
                ValueNameOptional::Unnamed(value) => {
                    array.push(value);
                }
            }

            self.skip_whitespace();
        }

        if tri!(some_or_error!(self.next())) != BRACKET_CLOSE {
            return Err("Expected ']'".to_string());
        }

        Ok(TsonValue::Array(array))
    }

    fn de_object(&mut self) -> Res<TsonValue> {
        let mut map = Map::new();
        self.skip_whitespace();
        while peek_not_brace_close(self.peek()) {
            let (name, value) = self.de_value_name_required()?;
            map.insert(name, value);
            self.skip_whitespace();
        }

        if tri!(some_or_error!(self.next())) != BRACE_CLOSE {
            return Err("Expected '}'".to_string());
        }

        Ok(TsonValue::Object(map))
    }

    fn de_type_specifier(&mut self) -> Res<TsonValue> {
        panic!("Not implemented");
    }

    fn de_name(&mut self) -> Res<Option<String>> {
        let name = self.take_while::<16>(is_name_part);
        if name.is_empty() {
            Ok(None)
        } else {
            Ok(Some(name))
        }
    }

    fn de_name_exact(&mut self) -> Res<String> {
        let name = self.take_while::<16>(is_name_part);
        if name.is_empty() {
            Err(format!(
                "Name unexpected end of input at position {}",
                self.index
            ))
        } else {
            Ok(name)
        }
    }

    fn de_bool(&mut self) -> Result<TsonValue> {
        match self.next() {
            Some(b't') => {
                self.skip_n(3);
                Ok(TsonValue::Boolean(true))
            }
            Some(b'f') => {
                self.skip_n(4);
                Ok(TsonValue::Boolean(false))
            }
            _ => Err("Expected 'true' or 'false'".to_string()),
        }
    }

    fn de_int(&mut self) -> Result<TsonValue> {
        let mut sign = 1;
        let first = self.peek();

        let mut result: i64 = 0;
        match first {
            Some(MINUS) => {
                sign = -1;
                self.next();
            }
            Some(PLUS) => {
                self.next();
            }
            None => return Ok(TsonValue::Int(0)),
            _ => (),
        }

        let mut processed_digits = false;

        while let Some(char) = self.next() {
            if let Some(digit) = is_int_digit(char) {
                result = (result << 3) + (result << 1) + digit as i64;
                processed_digits = true;
            } else {
                self.index -= 1;
                break;
            }
        }

        if !processed_digits {
            return Err("Expected digits after '#'".to_string());
        }

        Ok(TsonValue::Int(result * sign))
    }

    fn de_float(&mut self) -> Res<TsonValue> {
        let number_string = self.take_while::<16>(is_not_field_terminator);
        Ok(TsonValue::Float(number_string.parse::<f64>().unwrap()))
    }

    fn de_string(&mut self) -> Res<TsonValue> {
        let mut string = String::with_capacity(64);
        while let Some(c) = self.next() {
            match c {
                QUOTE => {
                    return Ok(TsonValue::String(string));
                }
                BACKSLASH => {
                    if let Some(next_char) = self.next() {
                        match next_char {
                            QUOTE => {
                                string.push(QUOTE as char);
                            }
                            BACKSLASH => {
                                string.push(BACKSLASH as char);
                            }
                            LF => {
                                string.push(LF as char);
                            }
                            CR => {
                                string.push(CR as char);
                            }
                            TAB => {
                                string.push(TAB as char);
                            }
                            _ => {
                                string.push(next_char as char);
                            }
                        }
                    } else {
                        return Err("Expected a character after '\\'".to_string());
                    }
                }
                _ => {
                    string.push(c as char);
                }
            }
        }
        Ok(TsonValue::String(string))
    }
}

const LETTER_A: u8 = b'A';
const LETTER_Z: u8 = b'Z';
const LETTER_LOWER_A: u8 = b'a';
const LETTER_LOWER_Z: u8 = b'z';
const UNDERSCORE: u8 = b'_';
const DOLLAR: u8 = b'$';
const DOT: u8 = b'.';
const DIGIT_0: u8 = b'0';
const DIGIT_9: u8 = b'9';

fn is_name_part(c: u8) -> bool {
    matches!(c, LETTER_A..=LETTER_Z | LETTER_LOWER_A..=LETTER_LOWER_Z | DIGIT_0..=DIGIT_9 | DOT | MINUS | DOLLAR | UNDERSCORE)
}

fn is_field_terminator(c: u8) -> bool {
    matches!(
        c,
        TAB | // TAB
        LF | // LF
        CR | // CR
        SPACE |  // SPACE
        QUOTE |  // QUOTE
        COMMA |  // COMMA
        EQUAL |  // EQUAL
        QUESTION |  // QUESTION
        BRACKET_OPEN |  // BRACKET_OPEN
        BRACKET_CLOSE |  // BRACKET_CLOSE
        BRACE_OPEN |  // BRACE_OPEN
        BRACE_CLOSE // BRACE_CLOSE
    )
}

fn is_int_digit(c: u8) -> Option<u8> {
    if c >= DIGIT_0 && c <= DIGIT_9 {
        Some(c - DIGIT_0)
    } else {
        None
    }
}

fn is_not_field_terminator(c: u8) -> bool {
    !is_field_terminator(c)
}

fn peek_not_bracket_close(c: Option<u8>) -> bool {
    if let Some(c) = c {
        if c != BRACKET_CLOSE { true } else { false }
    } else {
        false
    }
}

fn peek_not_brace_close(c: Option<u8>) -> bool {
    if let Some(c) = c {
        if c != BRACE_CLOSE { true } else { false }
    } else {
        false
    }
}
