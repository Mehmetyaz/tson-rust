# TSON syntax:

## Values

- Native type values have to have type prefixes: `<type_prefix><value>`
  - `#` for integers: `#42`
  - `?` for booleans: `?true`
  - `=` for floating-point/double numbers: `=99.99`
  - `~` for null values: `~` (value is not needed/allowed)
- Other type values have to be wrapped with:
  - `""` for strings: `"value"`
  - `{}` for objects: `{prop1 prop2}`
  - `[]` for arrays: `[item1 item2]`

## Naming

Naming a value: `<name><value>`

- Object properties must have names.
- Root values and array items can be named optionally, if they named, they will be parsed as an object with single property: `{name: value}`.
