use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use tracing::info;
use uuid::Uuid;

// region:      --- User Types
#[derive(Clone, FromRow, Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

/// For app api (register user )
#[derive(Clone, Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
}

/// FOr usr module impl
#[derive(Clone)]
pub struct UserForInsert {
    pub username: String,
}

/// Readonly -- information to validate login
#[derive(Clone, FromRow, Debug)]
pub struct UserForLogin {
    pub id: i64,
    pub username: String,

    // pwd and token info
    pub pwd: Option<String>, // encrypted
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

/// For Authentication logic
#[derive(Clone, FromRow, Debug)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,

    // token info
    pub token_salt: Uuid,
}

///Marker trait, make User, UserForLogin and UserForAuth grouped together to share common trait
pub trait UserBy: for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

// endregion:   --- User Types

pub struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "user";
}

impl UserBmc {
    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: UserBy,
    {
        base::get::<Self, E>(ctx, mm, id).await
    }

    pub async fn first_by_username<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let db = mm.db();
        let sql = format!(r#"select * from "{}" where username = $1"#, Self::TABLE,);
        let user: Option<E> = sqlx::query_as(&sql)
            .bind(username)
            .fetch_optional(db)
            .await?;
        Ok(user)
    }

    pub async fn update_pwd(ctx: &Ctx, mm: &ModelManager, id: i64, pwd_clear: &str) -> Result<()> {
        let db = mm.db();
        let user: UserForLogin = Self::get(ctx, mm, id).await?;
        info!("{:<12} - update_pwd - for user: {user:?}", "PREFIX");

        let pwd = pwd::encrypt_pwd(&EncryptContent {
            content: pwd_clear.to_string(),
            salt: user.pwd_salt.to_string(),
        })?;

        let sql = format!(r#"update "{}" set pwd = $1 where id = $2"#, Self::TABLE);
        let _ = sqlx::query(&sql).bind(pwd).bind(id).execute(db).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use crate::_dev_utils;
    use anyhow::{Context, Ok, Result};
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_frist_ok_demo1() -> Result<()> {
        let mm = _dev_utils::init_test(1).await;
        let ctx = Ctx::root_ctx();
        let fx_username = "demo1";

        let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username)
            .await?
            .context("Should have user 'demo1'")?;

        assert_eq!(user.username, fx_username);

        Ok(())
    }
}
