# Rust Notes 

## How to handle option, result and reference

- `map`
- `ok_or`
- `and_then`

## Learn how to organize Result type

- See how the Error from model/store is automatically converted to the Error of model. -- (47:57)
  - First, express model/store Error as an enum variant case for model Error.
  - Then, `impl From<store::Error> for Error` for model Error.

## Understand the decouple of web layer and model layer.

- This is done via context(Ctx)