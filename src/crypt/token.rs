use std::fmt;
use std::str::FromStr;

use crate::crypt::{encrypt_into_b64url, EncryptContent, Error, Result};
use crate::utils::{b64u_decode, b64u_encode, now_utc, now_utc_plus_sec_str, parse_utc};
use crate::{config, utils};

// region:      --- Token Type
pub struct Token {
    pub ident: String,     // identifier
    pub exp: String,       // Expiration data in RFC3339
    pub sign_b64u: String, // Signature, base64url encoded
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = format!(
            "{}.{}.{}",
            utils::b64u_encode(&self.ident),
            utils::b64u_encode(&self.exp),
            self.sign_b64u
        );
        write!(f, "{}", output)
    }
}

impl FromStr for Token {
    type Err = Error;
    fn from_str(token_str: &str) -> std::result::Result<Self, Self::Err> {
        let splits: Vec<&str> = token_str.split('.').collect();
        if splits.len() != 3 {
            return Err(Error::TokenINvalidFormat);
        }

        let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);

        Ok(Self {
            ident: b64u_decode(ident_b64u).map_err(|_| Error::TokenCannotDecodeIdent)?,
            exp: b64u_decode(exp_b64u).map_err(|_| Error::TokenCannotDecodeExp)?,
            sign_b64u: sign_b64u.to_string(),
        })
    }
}

// endregion:   --- Token Type

// region:      --- Web Token Gen and Validation
pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
    let config = &config();
    _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: &str) -> Result<()> {
    let config = &config();
    let _ = _validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)?;
    Ok(())
}

// endregion:   --- Web Token Gen and Validation

// region:      --- (private) Token Gen and Validation
fn _generate_token(ident: &str, duration_sec: f64, salt: &str, key: &[u8]) -> Result<Token> {
    let ident = ident.to_string();
    let exp = now_utc_plus_sec_str(duration_sec);

    // sign the two first components
    let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt, key)?;

    Ok(Token {
        ident,
        exp,
        sign_b64u,
    })
}

fn _validate_token_sign_and_exp(origin_token: &Token, salt: &str, key: &[u8]) -> Result<()> {
    // -- validate signature
    let new_sign_b64u = _token_sign_into_b64u(&origin_token.ident, &origin_token.exp, salt, key)?;
    if new_sign_b64u != origin_token.sign_b64u {
        return Err(Error::TokenSignatureNotMatching);
    }

    // -- validate expiration
    let origin_exp = parse_utc(&origin_token.exp).map_err(|_| Error::TokenExpIsNotIso)?;
    let now = now_utc();

    if origin_exp < now {
        return Err(Error::TokenExpired);
    }

    Ok(())
}

// create token signature from token parts and salt
fn _token_sign_into_b64u(ident: &str, exp: &str, salt: &str, key: &[u8]) -> Result<String> {
    let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));

    // can also use sha512pass without the hash mark
    let signature = encrypt_into_b64url(
        key,
        &EncryptContent {
            content,
            salt: salt.to_string(),
        },
    )?;

    Ok(signature)
}

// endregion:   --- (private) Token Gen and Validation
