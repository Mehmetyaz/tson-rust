use crate::types::{Map, Res, TsonValue};
use std::{iter::Peekable, str::Chars}; // Veya u8x32 vb.
const SPACE: char = ' '; // ' '
const COMMA: char = ','; // ','
const TAB: char = '\t'; // '\t'
const CR: char = '\r'; // '\r'
const LF: char = '\n'; // '\n'
const LETTER_T: char = 't'; // 't'
const LETTER_F: char = 'f'; // 'f'
const QUOTE: char = '"'; // '"'
const BACKSLASH: char = '\\'; // '\\'

const BRACKET_OPEN: char = '['; // '['
const BRACKET_CLOSE: char = ']'; // ']'

const BRACE_OPEN: char = '{'; // '{'
const BRACE_CLOSE: char = '}'; // '}'

const LESS_THAN: char = '<'; // '<'

const HASH: char = '#'; // '#'
const EQUAL: char = '='; // '='
const QUESTION: char = '?'; // '?'

const AVERAGE: char = '~'; // '~'

const UNEXPECTED_END: &str = "Unexpected end of input";
const NAME_REQUIRED: &str = "Name is required";

/// Parser state for tracking position in input string
struct Parsing<'a> {
    //input: &'a str,
    chars: Peekable<Chars<'a>>,
}

/// TSON Parser
pub struct Parser;

impl Parser {
    /// Parse TSON string to Rust value
    pub fn parse<'a>(input: &'a str) -> Res<TsonValue> {
        let mut parsing = Parsing::<'a> {
            chars: input.chars().peekable(),
        };

        Self::skip_whitespace(&mut parsing);

        let result = Self::parse_value_name_optional(&mut parsing);

        match result {
            Ok((Some(name), value)) => {
                let mut map = Map::new();
                map.insert(name, value);
                Ok(TsonValue::Object(map))
            }
            Ok((None, value)) => Ok(value),
            Err(e) => Err(e),
        }
    }

    fn parse_value_name_required(parsing: &mut Parsing) -> Res<(String, TsonValue)> {
        if parsing.chars.peek().is_none() {
            return Err(UNEXPECTED_END.to_string());
        }

        let name = Self::parse_name(parsing)?;

        if let Some(name) = name {
            let value = Self::parse_value(parsing)?;
            Self::skip_whitespace(parsing);
            Ok((name, value))
        } else {
            Err(NAME_REQUIRED.to_string())
        }
    }

    fn parse_value_name_optional(parsing: &mut Parsing) -> Res<(Option<String>, TsonValue)> {
        let name = Self::parse_name(parsing)?;

        if let Some(name) = name {
            let value = Self::parse_value(parsing)?;
            Ok((Some(name), value))
        } else {
            let value = Self::parse_value(parsing)?;
            Ok((None, value))
        }
    }

    fn parse_value(parsing: &mut Parsing) -> Res<TsonValue> {
        if parsing.chars.peek().is_none() {
            return Err(UNEXPECTED_END.to_string());
        }

        let char = parsing.chars.next().unwrap();
        match char {
            BRACE_OPEN => Self::parse_object(parsing),
            BRACKET_OPEN => Self::parse_array(parsing),
            LESS_THAN => Self::parse_array_type_specifier(parsing),
            QUOTE => Self::parse_string(parsing),
            HASH => Self::parse_int(parsing),
            EQUAL => Self::parse_float(parsing),
            QUESTION => Self::parse_boolean(parsing),
            AVERAGE => Ok(TsonValue::Null),
            _ => Err(format!("Unexpected character: {}", String::from(char))),
        }
    }

    fn parse_object(parsing: &mut Parsing) -> Res<TsonValue> {
        let mut map = Map::new();

        Self::skip_whitespace(parsing);

        while let Some(c) = parsing.chars.peek() {
            match c {
                &BRACE_CLOSE => {
                    parsing.chars.next();
                    break;
                }
                _ => {
                    let (name, value) = Self::parse_value_name_required(parsing)?;
                    map.insert(name, value);
                    Self::skip_whitespace(parsing);
                }
            }
        }

        // parsing.chars.next(); // skip the closing brace

        Ok(TsonValue::Object(map))
    }

    fn parse_array(parsing: &mut Parsing) -> Res<TsonValue> {
        let mut array = Vec::new();

        Self::skip_whitespace(parsing);

        while let Some(c) = parsing.chars.peek() {
            match c {
                &BRACKET_CLOSE => {
                    parsing.chars.next();
                    break;
                }
                _ => {
                    let (name, value) = Self::parse_value_name_optional(parsing)?;
                    if let Some(name) = name {
                        let mut item_map = Map::new();
                        item_map.insert(name, value);
                        array.push(TsonValue::Object(item_map));
                    } else {
                        array.push(value);
                    }
                    Self::skip_whitespace(parsing);
                }
            }
        }

        //parsing.chars.next(); // skip the closing bracket

        Ok(TsonValue::Array(array))
    }

    fn parse_array_type_specifier(parsing: &mut Parsing) -> Res<TsonValue> {
        println!(
            "parse_array_type_specifier {:?}",
            parsing.chars.peek().unwrap()
        );
        panic!("Not implemented");
    }

    // fn _handle_backslash(
    //     parsing: &mut Parsing,
    //     chunk: Simd<u8, 16>,
    //     backslash_mask: &mut Mask<i8, 16>,
    //     result: &mut String,
    //     count: usize,
    // ) -> bool {
    //     let mut res = unsafe { String::from_utf8_unchecked(chunk.to_array()[0..count].to_vec()) };

    //     let mut removed = 0;

    //     while let Some(backslash_offset) = backslash_mask.first_set() {
    //         let n_offset = backslash_offset + 1;
    //         match chunk[n_offset] {
    //             QUOTE => {
    //                 // remove the backslash
    //                 res.remove(backslash_offset - removed);
    //                 removed += 1;
    //                 backslash_mask.set(backslash_offset, false);
    //                 continue;
    //             }
    //             LF => {
    //                 // remove the backslash and add a newline
    //                 res.remove(backslash_offset - removed);
    //                 res.push('\n');
    //                 removed += 1;
    //                 backslash_mask.set(backslash_offset, false);
    //                 continue;
    //             }
    //             CR => {
    //                 // remove the backslash and add a carriage return
    //                 res.remove(backslash_offset - removed);
    //                 res.push('\r');
    //                 removed += 1;
    //                 backslash_mask.set(backslash_offset, false);
    //                 continue;
    //             }
    //             TAB => {
    //                 // remove the backslash and add a tab
    //                 res.remove(backslash_offset - removed);
    //                 res.push('\t');
    //                 removed += 1;
    //                 backslash_mask.set(backslash_offset, false);
    //                 continue;
    //             }
    //             BACKSLASH => {
    //                 // remove the backslash
    //                 res.remove(backslash_offset - removed);
    //                 removed += 1;
    //                 backslash_mask.set(backslash_offset, false);
    //                 backslash_mask.set(n_offset, false);
    //                 continue;
    //             }
    //             _ => {
    //                 // skip the backslash
    //                 backslash_mask.set(backslash_offset, false);
    //                 continue;
    //             }
    //         }
    //     }

    //     result.push_str(&res);
    //     parsing.position += count;
    //     count == LONG_CHUNK_SIZE
    // }

    // fn _handle_string_chunk(
    //     parsing: &mut Parsing,
    //     chunk: Simd<u8, 16>,
    //     result: &mut String,
    // ) -> bool {
    //     let mut quote_mask = chunk.simd_eq(*QUOTE_MASK_16);
    //     let mut backslash_mask = chunk.simd_eq(*BACKSLASH_MASK_16);

    //     loop {
    //         if let Some(quote_offset) = quote_mask.first_set() {
    //             if backslash_mask.test(quote_offset - 1) {
    //                 // skip this quote
    //                 quote_mask.set(quote_offset, false);
    //                 continue;
    //             } else {
    //                 // found the end of the string
    //                 Self::_set_mask_false_after_offset(&mut backslash_mask, quote_offset);
    //                 return Self::_handle_backslash(
    //                     parsing,
    //                     chunk,
    //                     &mut backslash_mask,
    //                     result,
    //                     quote_offset,
    //                 );
    //             }
    //         } else {
    //             // not found the end of the string
    //             return Self::_handle_backslash(
    //                 parsing,
    //                 chunk,
    //                 &mut backslash_mask,
    //                 result,
    //                 chunk
    //                     .simd_eq(*ZERO_MASK_16)
    //                     .first_set()
    //                     .unwrap_or(LONG_CHUNK_SIZE),
    //             );
    //         }
    //     }
    // }

    fn parse_string(parsing: &mut Parsing) -> Res<TsonValue> {
        let mut result = String::new();

        while let Some(c) = parsing.chars.next() {
            // current char consumed
            match c {
                BACKSLASH => {
                    parsing.chars.next();
                    result.push(c);
                }
                QUOTE => break,
                _ => result.push(c),
            }
        }

        Ok(TsonValue::String(result))
    }

    // Parses a name from the input string
    // Uses short chunks to parse the name
    fn parse_name(parsing: &mut Parsing) -> Res<Option<String>> {
        let mut result = String::new();

        while let Some(c) = parsing.chars.peek() {
            if Self::is_name_char(*c) {
                result.push(*c);
                parsing.chars.next();
            } else {
                break;
            }
        }

        if result.is_empty() {
            return Ok(None);
        }

        Ok(Some(result))
    }

    fn is_name_char(c: char) -> bool {
        c.is_ascii_alphabetic()
            || c.is_ascii_digit()
            || c == '_'
            || c == '-'
            || c == '$'
            || c == '.'
    }

    fn is_field_terminator(c: char) -> bool {
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

    // fn is_type_specifier(c: u8) -> bool {
    //     matches!(c,
    //         BRACKET_OPEN |  // BRACKET_OPEN
    //         BRACE_OPEN |  // BRACE_OPEN
    //         HASH |  // HASH
    //         EQUAL |  // EQUAL
    //         QUESTION| // QUESTION
    //         QUOTE | // QUOTE
    //         LESS_THAN // LESS_THAN
    //     )
    // }

    fn parse_float(parsing: &mut Parsing) -> Res<TsonValue> {
        let mut str = String::new();

        while let Some(c) = parsing.chars.peek() {
            if Self::is_field_terminator(*c) {
                break;
            }
            str.push(*c);
            parsing.chars.next();
        }

        // Parse the &str directly to f64
        str.parse::<f64>()
            .map(TsonValue::Float) // Wrap the result in TsonValue::Float
            .map_err(|e| format!("Failed to parse float literal '{}': {}", str, e))
    }

    fn parse_int(parsing: &mut Parsing) -> Res<TsonValue> {
        let mut str = String::new();

        while let Some(c) = parsing.chars.peek() {
            if Self::is_field_terminator(*c) {
                break;
            }
            str.push(*c);
            parsing.chars.next();
        }

        str.parse::<i64>()
            .map(TsonValue::Int) // Wrap the result in TsonValue::Int
            .map_err(|e| format!("Failed to parse int literal '{}': {}", str, e))
    }

    fn parse_boolean(parsing: &mut Parsing) -> Res<TsonValue> {
        let value = parsing.chars.next().unwrap();

        match value {
            LETTER_T => {
                parsing.chars.next();
                parsing.chars.next();
                parsing.chars.next();
                Ok(TsonValue::Boolean(true))
            }
            LETTER_F => {
                parsing.chars.next();
                parsing.chars.next();
                parsing.chars.next();
                parsing.chars.next();
                Ok(TsonValue::Boolean(false))
            }
            _ => return Err(format!("Invalid boolean value: {}", value)),
        }
    }

    // Uses short chunks to skip whitespace
    fn skip_whitespace(parsing: &mut Parsing) {
        while let Some(c) = parsing.chars.peek() {
            match c {
                &SPACE | &TAB | &CR | &LF => {
                    parsing.chars.next();
                }
                _ => break,
            }
        }

        // loop {
        //     if Self::is_end_of_input(parsing) {
        //         break;
        //     }

        //     let chunk = Self::next_chunk_short(parsing);

        //     // Check equal to whitespace mask and find first true
        //     let mask = !chunk.simd_eq(*SPACE_SIMD_4)
        //         & !chunk.simd_eq(*COMMA_SIMD_4)
        //         & !chunk.simd_eq(*TAB_SIMD_4)
        //         & !chunk.simd_eq(*CR_SIMD_4)
        //         & !chunk.simd_eq(*LF_SIMD_4);

        //     if let Some(offset) = mask.first_set() {
        //         parsing.position += offset;
        //         break;
        //     } else {
        //         parsing.position += SHORT_CHUNK_SIZE;
        //     }
        // }
    }

    // fn is_at_end(parsing: &Parsing) -> bool {
    //     parsing.position >= parsing.chars.len()
    // }

    // fn next_chunk_short(parsing: &mut Parsing) -> Simd<u8, SHORT_CHUNK_SIZE> {
    //     debug_assert!(parsing.position < parsing.chars.len());
    //     if parsing.position + SHORT_CHUNK_SIZE > parsing.chars.len() {
    //         let mut array = [0; SHORT_CHUNK_SIZE];
    //         array[0..parsing.chars.len() - parsing.position]
    //             .copy_from_slice(&parsing.chars[parsing.position..]);

    //         u8x4::from_array(array)
    //     } else {
    //         u8x4::from_slice(&parsing.chars[parsing.position..parsing.position + SHORT_CHUNK_SIZE])
    //     }
    // }

    // fn next_chunk_long(parsing: &mut Parsing) -> Simd<u8, LONG_CHUNK_SIZE> {
    //     debug_assert!(parsing.position < parsing.chars.len());
    //     if parsing.position + LONG_CHUNK_SIZE > parsing.chars.len() {
    //         let mut array = [0; LONG_CHUNK_SIZE];
    //         array[0..parsing.chars.len() - parsing.position]
    //             .copy_from_slice(&parsing.chars[parsing.position..]);

    //         u8x16::from_array(array)
    //     } else {
    //         u8x16::from_slice(&parsing.chars[parsing.position..parsing.position + LONG_CHUNK_SIZE])
    //     }
    // }
}
