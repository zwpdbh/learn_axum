use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::web::AUTH_TOKEN;
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
// region:    --- Ctx Extractor Result/Error
type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    TokenNotInCookie,
    CtxNotInRequestExt,
    CtxCreateFail(String),
}
// endreg

/// used by middleware async function
#[allow(unused)]
pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    debug!(" {:<12} - mw_require_auth", "MIDDLEWARE");
    let _ = ctx?;

    Ok(next.run(req).await)
}

/// For Ctx extractor optimiation
pub async fn mw_ctx_resolver(
    _mm: State<ModelManager>, // for db connection
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!(" {:<12} - mw_ctx_resolver", "MIDDLEWARE");
    let _auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // FIXME - Compute real CtxAuthResult<Ctx>.
    let result_ctx = Ctx::new(100).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()));

    // Remove the cookie if something went wrong other than NoAuthTokenCookie
    if result_ctx.is_err() && !matches!(result_ctx, Err(CtxExtError::TokenNotInCookie)) {
        // if there is no cookie then we don't need to worry about it.
        // but if there is one and it failed then, we need to clean it for the client.
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

// Ctx Extractor
// There are two types of extractor: one for the body(1), another is for any other information but the body(2).
// This is for second case (2): which will take informations from headers or the URL parameters and so on.
// Here, we want to take it from the cookies so from the headers.
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

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
