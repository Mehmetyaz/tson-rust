# TSON (Token-Saving Object Notation)

---

## This is a newly created project. Not ready for production use.

TSON is a compact, human-readable data format designed for token-efficient, easy-to-parse data representation. Main reason to create this format is to reduce the number of tokens in the response of the LLM APIs. It is not recommend to use this format for data storage/exchange, but still it is suitable.

## Core Syntax

TSON uses a simple, concise syntax:

`<name?><type_specifier?><value?><type_specifier?>`

- name: key of the value.
  - It is optional if value is in root level or array.
  - All object properties must have names.
- type_specifier: first type specifier is required
  - as an exception, if the parent is an array and the array has type_specifier.
- value: value of the key.
  - As an exception, for null values value is not needed, not allowed.
- type_specifier: second type specifier is required for String, Object, Array. Not allowed for other types.

### Type Specifiers

- `#` for integers: `name#42`
- `?` for booleans: `name?true`
- `=` for floating-point/double numbers: `name=99.99`
- `~` for null values: `name~`
- `""` for strings: `name"value"`
- `{}` for objects: `name{prop1 prop2}`
- `[]` for arrays: `name[item1 item2]`
- `<#|?|=>[]` for typed arrays: `name<#|?|=>[item1 item2]`

## Naming Rules

- All names must be valid JavaScript identifiers.

  - Cannot starts with a number.
  - First character must be matched with `[a-z]` (lowercase letter), `[A-Z]` (uppercase letter), `_` (underscore) or `$` (dollar sign).
  - Other characters must be matched with first character's rule and also can be `[0-9]` (digit) or `.` (dot) or `-` (dash).
  - Cannot include `#`, `?`, `=`, `~`, `""`, `{}`, `[]`, `<`, `>`.

- All properties of objects must have names.
- If the root property or array item has a name, it will be converted to `{name: property}` format in JSON.
