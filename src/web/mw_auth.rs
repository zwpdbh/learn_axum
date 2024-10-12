use async_trait::async_trait;
use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{Ctx, Error, Result};

/// used by middleware async function
pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    let _ = ctx?;

    Ok(next.run(req).await)
}

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}

// Ctx Extractor
// There are two types of extractor: one for the body(1), another is for any other information but the body(2).
// This is for second case (2): which will take informations from headers or the URL parameters and so on.
// Here, we want to take it from the cookies so from the headers.
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");
        // use the cookie extractor
        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // TODO: Real auth-token parsing & validation.
        let (user_id, exp, sign) = auth_token
            // From Option type to Result type
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            // Used to chain a Result type
            .and_then(parse_token)?;

        Ok(Ctx::new(user_id))
    }
}
