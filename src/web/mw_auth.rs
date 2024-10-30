use crate::crypt::{validate_web_token, Token};
use crate::ctx::Ctx;
use crate::model::user::UserBmc;
use crate::model::{user, ModelManager, UserForAuth};
use crate::web::{set_token_cookie, AUTH_TOKEN};
use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

/// used by middleware async function
pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    debug!(" {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");
    let _ = ctx?;

    Ok(next.run(req).await)
}

/// For Ctx extractor optimiation
pub async fn mw_ctx_resolve(
    mm: State<ModelManager>, // for db connection
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!(" {:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let ctx_ext_result = _ctx_resolve(mm, &cookies).await;
    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie)) {
        // remove invalid cookie such that we don't need to validate it over and over
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store the ctx_ext_result in the request extension (for Ctx extractor)
    req.extensions_mut().insert(ctx_ext_result);

    Ok(next.run(req).await)
}

async fn _ctx_resolve(mm: State<ModelManager>, cookies: &Cookies) -> CtxExtResult {
    // -- Get Token String
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)?;

    // -- Parse Token
    let token: Token = token.parse().map_err(|_| CtxExtError::TokenWrongFormat)?;

    // -- Get UserForAuth
    let user: UserForAuth = UserBmc::first_by_username(&Ctx::root_ctx(), &mm, &token.ident)
        .await
        .map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))? // First handle Result
        .ok_or(CtxExtError::UserNotFound)?; // Then handle Option to conver to Result type

    // -- Validate Token
    let () = validate_web_token(&token, &user.token_salt.to_string())
        .map_err(|_| CtxExtError::FailValidateToken)?;

    // -- Update Token
    let () = set_token_cookie(cookies, &user.username, &user.token_salt.to_string())
        .map_err(|_| CtxExtError::CannotSetTokenCookie)?;

    // -- Create CtxExtResult
    Ctx::new(user.id as u64).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

// Ctx Extractor
// There are two types of extractor: one for the body(1), another is for any other information but the body(2).
// This is for second case (2): which will take informations from headers or the URL parameters and so on.
// Here, we want to take it from the cookies so from the headers.
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    /// If there is no ctx result, fail earlier.
    /// It also map CtxExtresult Error to Web Error.
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!(" {:<12} - Ctx", "EXTRACTOR");
        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}

// region:      --- Ctx Extractor Result/Error
// That is what the Ctx resolve will put into the request and so that is the same pattern that
// cookies from Tower cookies uses.
// You do the expensive part once and then you put it in the request as an extension.
// Then you have the extractor that just pluck it out.
type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    TokenNotInCookie,
    CtxNotInRequestExt,
    CtxCreateFail(String),
    TokenWrongFormat,
    UserNotFound,
    ModelAccessError(String),
    CannotSetTokenCookie,
    FailValidateToken,
}
// endregion:   --- Ctx Extractor Result/Error
