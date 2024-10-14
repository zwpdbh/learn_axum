mod error;
pub use self::error::{Error, Result};

// used by both web layer and model layer
#[derive(Clone)]
pub struct Ctx {
    user_id: u64,
}

// Constructor
impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: 0 }
    }
    pub fn new(user_id: u64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
