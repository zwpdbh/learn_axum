# Rust Notes 

## How to handle option, result and reference

- `map`
- `ok_or`
- `and_then`

## Learn how to organize Result type

- See how the Error from model/store is automatically converted to the Error of model. -- (47:57)
  - First, express model/store Error as an enum variant case for model Error.
  - Then, `impl From<store::Error> for Error` for model Error.

## Example: Solve `?` couldn't convert the `error` to `model::error::Error`

- Complete error message 

```txt
`?` couldn't convert the error to `model::error::Error`
the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
the following other types implement trait `From<T>`:
  <model::error::Error as From<sqlx::Error>>
  <model::error::Error as From<store::error::Error>>
required for `std::result::Result<(), model::error::Error>` to implement 
```

- Currently, the `model::error::Error` is defined as:

```rust
pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    EntityNotFound { entity: &'static str, id: i64 },
    // -- Modules
    Store(store::Error),

    // -- Externals
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}
```

While, the `error` is type of `crypt::error::Error`:

```rust
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    KeyFailHmac,
    PwdNotMatching,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
```

Need to modify `model::error::Error`:

```rust 
pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    EntityNotFound { entity: &'static str, id: i64 },
    Store(store::Error),
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
    // 1) Add this variant
    Crypt(crypt::Error), 
}

// 2) Implement trait `From<T>` for `model::error::Error`
impl From<crypt::Error> for Error {
    fn from(value: crypt::Error) -> Self {
        Self::Crypt(value)
    }
}
```