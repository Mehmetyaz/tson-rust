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
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(chars: &'a [u8]) -> Self {
        Self { chars, position: 0 }
    }

    pub fn from_str(input: &'a str) -> Self {
        Self::new(input.as_bytes())
    }
}

pub trait Parse {
    fn peek(&self) -> u8;
    fn peek_maybe(&self) -> Option<u8>;
    fn advance(&mut self);
    fn advance_multiple(&mut self, count: usize);
    //fn advance_multiple_and_return(&mut self, count: usize) -> Vec<u8>;
    fn advance_and_return(&mut self) -> u8;
    fn advance_and_return_maybe(&mut self) -> Option<u8>;
    fn skip_whitespace(&mut self);
    fn get_context(&self, position: usize) -> (String, String);
    fn is_end(&self) -> bool;
    fn deserialize(&mut self) -> Res<TsonValue>;
    fn de_bool(&mut self) -> Res<bool>;
    fn de_int(&mut self) -> Res<i64>;
    fn de_float(&mut self) -> Res<f64>;
    fn de_string(&mut self) -> Res<String>;
    fn de_array(&mut self) -> Res<Vec<TsonValue>>;
    fn de_object(&mut self) -> Res<Map<String, TsonValue>>;
    fn de_type_specifier(&mut self) -> Res<String>;
    fn de_name(&mut self) -> Res<String>;
    fn de_value_name_required(&mut self) -> Res<(String, TsonValue)>;
    fn de_value_name_optional(&mut self) -> Res<(Option<String>, TsonValue)>;
    fn de_value(&mut self) -> Res<TsonValue>;
}

impl<'a> Parse for Parser<'a> {
    fn peek(&self) -> u8 {
        self.chars[self.position]
    }

    fn peek_maybe(&self) -> Option<u8> {
        if self.position < self.chars.len() {
            Some(self.chars[self.position])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn advance_multiple(&mut self, count: usize) {
        self.position += count;
    }

    // fn advance_multiple_and_return(&mut self, count: usize) -> Vec<u8> {
    //     let chars = self.chars[self.position..self.position + count].to_vec();
    //     self.position += count;
    //     chars
    // }

    fn advance_and_return(&mut self) -> u8 {
        let char = self.chars[self.position];
        self.position += 1;
        char
    }

    fn advance_and_return_maybe(&mut self) -> Option<u8> {
        if self.position < self.chars.len() {
            let char = self.chars[self.position];
            self.position += 1;
            Some(char)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_end() {
            let char = self.peek();
            if !matches!(char, b' ' | b'\t' | b'\r' | b'\n' | b',') {
                break;
            }
            self.position += 1;
        }
    }

    fn get_context(&self, position: usize) -> (String, String) {
        let start = if position >= 10 { position - 10 } else { 0 };
        let end = if position + 10 < self.chars.len() {
            position + 10
        } else {
            self.chars.len()
        };

        let before = String::from_utf8_lossy(&self.chars[start..position]).to_string();
        let after = String::from_utf8_lossy(&self.chars[position..end]).to_string();
        (before, after)
    }

    fn is_end(&self) -> bool {
        self.position >= self.chars.len()
    }

    fn deserialize(&mut self) -> Res<TsonValue> {
        self.skip_whitespace();
        let result = self.de_value_name_optional().unwrap_or_else(|e| {
            panic!("Parsing error: {}", e);
        });

        match result {
            (Some(name), value) => {
                let mut map = Map::new();
                map.insert(name, value);
                Ok(TsonValue::Object(map))
            }
            (None, value) => Ok(value),
        }
    }

    fn de_value_name_optional(&mut self) -> Res<(Option<String>, TsonValue)> {
        if is_name_start(self.peek()) {
            let name = self.de_name()?;
            let value = self.de_value()?;
            Ok((Some(name), value))
        } else {
            let value = self.de_value()?;
            Ok((None, value))
        }
    }

    fn de_value_name_required(&mut self) -> Res<(String, TsonValue)> {
        let name = self.de_name()?;

        if !is_type_specifier(self.peek()) {
            return Ok((name, TsonValue::Null));
        }

        let value = self.de_value()?;

        Ok((name, value))
    }

    fn de_value(&mut self) -> Res<TsonValue> {
        match self.advance_and_return() {
            BRACE_OPEN => Ok(TsonValue::Object(self.de_object()?)),
            BRACKET_OPEN => Ok(TsonValue::Array(self.de_array()?)),
            LESS_THAN => Ok(TsonValue::String(self.de_type_specifier()?)),
            QUOTE => Ok(TsonValue::String(self.de_string()?)),
            HASH => Ok(TsonValue::Int(self.de_int()?.into())),
            EQUAL => {
                if let Ok(float) = self.de_float() {
                    Ok(TsonValue::Float(float))
                } else {
                    Err("Expected a number".to_string())
                }
            }
            QUESTION => Ok(TsonValue::Boolean(self.de_bool()?)),
            AVERAGE => Ok(TsonValue::Null),
            _ => {
                let (before, after) = self.get_context(self.position);
                Err(format!(
                    "Unexpected character: {} at position {}, before '{}' after '{}'",
                    String::from_utf8_lossy(&[self.peek()]),
                    self.position,
                    before,
                    after
                ))
            }
        }
    }

    fn de_array(&mut self) -> Res<Vec<TsonValue>> {
        self.skip_whitespace();
        let mut array = Vec::new();
        while peek_not_bracket_close(self.peek_maybe()) {
            let result = self.de_value_name_optional()?;
            if let Some(name) = result.0 {
                let mut item_map = Map::new();
                item_map.insert(name, result.1);
                array.push(TsonValue::Object(item_map));
            } else {
                array.push(result.1);
            }
            self.skip_whitespace();
        }

        if self.advance_and_return() != BRACKET_CLOSE {
            return Err("Expected ']'".to_string());
        }

        Ok(array)
    }

    fn de_object(&mut self) -> Res<Map<String, TsonValue>> {
        let mut map = Map::new();
        while peek_not_brace_close(self.peek_maybe()) {
            let (name, value) = self.de_value_name_required()?;
            map.insert(name, value);
            self.skip_whitespace();
        }

        if self.peek() != BRACE_CLOSE {
            return Err("Expected '}'".to_string());
        }

        self.advance();

        Ok(map)
    }

    fn de_type_specifier(&mut self) -> Res<String> {
        let mut type_specifier = String::new();
        while is_type_specifier(self.peek()) {
            type_specifier.push(self.advance_and_return() as char);
        }
        Ok(type_specifier)
    }

    fn de_name(&mut self) -> Res<String> {
        let mut name = Vec::new();
        while is_name_part_maybe(self.peek_maybe()) {
            name.push(self.advance_and_return());
        }
        Ok(String::from_utf8(name).unwrap())
    }

    fn de_bool(&mut self) -> Res<bool> {
        match self.advance_and_return() {
            b't' => {
                self.advance_multiple(3);
                Ok(true)
            }
            b'f' => {
                self.advance_multiple(4);
                Ok(false)
            }
            _ => Err("Expected 'true' or 'false'".to_string()),
        }
    }

    fn de_int(&mut self) -> Res<i64> {
        let mut sign = 1;
        if self.peek() == MINUS {
            sign = -1;
            self.advance();
        } else if self.peek() == PLUS {
            self.advance();
        }

        let mut result: i64 = 0;
        let mut processed_digits = false;
        while let Some(digit) = is_int_digit(self.peek_maybe()) {
            result = (result << 3) + (result << 1) + digit as i64;
            processed_digits = true;
            self.advance();
        }

        if !processed_digits {
            return Err("Expected digits after '#'".to_string());
        }

        Ok(result * sign)
    }

    fn de_float(&mut self) -> Res<f64> {
        let mut number_string = String::with_capacity(16);

        while let Some(c) = is_not_field_terminator(self.peek_maybe()) {
            number_string.push(c as char);
            self.advance();
        }

        Ok(number_string.parse::<f64>().unwrap())
    }

    fn de_string(&mut self) -> Res<String> {
        let mut string = String::with_capacity(64);
        while let Some(c) = is_not_close_quote(self.advance_and_return_maybe()) {
            if c == BACKSLASH {
                if let Some(next_char) = self.advance_and_return_maybe() {
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
                            string.push(c as char);
                            string.push(next_char as char);
                        }
                    }
                } else {
                    return Err("Expected a character after '\\'".to_string());
                }
            } else {
                string.push(c as char);
            }
        }
        Ok(string)
    }
}

fn is_type_specifier(c: u8) -> bool {
    matches!(
        c,
        BRACKET_OPEN |  // BRACKET_OPEN
        BRACE_OPEN |  // BRACE_OPEN
        HASH |  // HASH
        EQUAL |  // EQUAL
        QUESTION |  // QUESTION
        QUOTE | // QUOTE
        LESS_THAN | // LESS_THAN
        AVERAGE // AVERAGE
    )
}

fn is_name_start(c: u8) -> bool {
    matches!(c,
        b'A'..=b'Z' |
        b'a'..=b'z' |
        b'_' | b'$'
    )
}

fn is_name_part(c: u8) -> bool {
    matches!(c,
        b'A'..=b'Z' |
        b'a'..=b'z' |
        b'0'..=b'9' |
        b'.' | b'-' | b'$'
    )
}

fn is_name_part_maybe(c: Option<u8>) -> bool {
    if let Some(c) = c {
        is_name_part(c)
    } else {
        false
    }
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

fn is_int_digit(c: Option<u8>) -> Option<u8> {
    if let Some(c) = c {
        if c >= b'0' && c <= b'9' {
            Some(c - b'0')
        } else {
            None
        }
    } else {
        None
    }
}

fn is_not_field_terminator(c: Option<u8>) -> Option<u8> {
    if let Some(c) = c {
        if !is_field_terminator(c) {
            Some(c)
        } else {
            None
        }
    } else {
        None
    }
}

fn is_not_close_quote(c: Option<u8>) -> Option<u8> {
    if let Some(c) = c {
        if c != QUOTE { Some(c) } else { None }
    } else {
        None
    }
}

fn peek_not_brace_close(c: Option<u8>) -> bool {
    if let Some(c) = c {
        if c != BRACE_CLOSE { true } else { false }
    } else {
        false
    }
}

fn peek_not_bracket_close(c: Option<u8>) -> bool {
    if let Some(c) = c {
        if c != BRACKET_CLOSE { true } else { false }
    } else {
        false
    }
}
