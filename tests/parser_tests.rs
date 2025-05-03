// use serde_json::json;
// use tson_rust::{TsonValue, Tson};

// #[test]
// fn test_parse_object() {
//     let input = r#"{
//         name "Test Object"
//         value #42
//         pi =3.14159
//         enabled ?true
//     }"#;

//     let expected = json!({
//         "name": "Test Object",
//         "value": 42,
//         "pi": 3.14159,
//         "enabled": true
//     });

//     let result = TSON::parse(input);
//     assert_eq!(result, expected);
// }

// #[test]
// fn test_parse_array() {
//     let input = r#"[
//         #1
//         #2
//         #3
//         "test"
//         ?true
//     ]"#;

//     let expected = json!([1, 2, 3, "test", true]);

//     let result = TSON::parse(input);
//     assert_eq!(result, expected);
// }

// #[test]
// fn test_stringify() {
//     let value = json!({
//         "name": "Test",
//         "values": [1, 2, 3],
//         "nested": {
//             "a": true,
//             "b": 3.14
//         }
//     });

//     let result = TSON::stringify(&value, false);
    
//     // We need to parse the result back to compare JSON values
//     // since string representation might differ in formatting
//     let parsed_result = TSON::parse(&result);
//     assert_eq!(parsed_result, value);
// }

// #[test]
// fn test_round_trip() {
//     let original = json!({
//         "string": "Hello, world!",
//         "integer": 42,
//         "float": 3.14159,
//         "boolean": true,
//         "array": [1, 2, 3, "test"],
//         "object": {
//             "nested": "value"
//         }
//     });

//     let tson_str = TSON::stringify(&original, false);
//     let parsed_back = TSON::parse(&tson_str);

//     assert_eq!(parsed_back, original);
// }

// #[test]
// fn test_using_tson_struct() {
//     let input = r#"{
//         name "Via TSON struct"
//         value #99
//     }"#;

//     let expected = json!({
//         "name": "Via TSON struct",
//         "value": 99
//     });

//     let result = TSON::parse(input);
//     assert_eq!(result, expected);

//     let tson_string = TSON::stringify(&expected, false);
//     let parsed_back = TSON::parse(&tson_string);
//     assert_eq!(parsed_back, expected);
// } 