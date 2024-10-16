use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use sqlx::postgres::PgRow;
use sqlx::FromRow;

pub trait DbBmc {
    const TABLE: &'static str;
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let db = mm.db();
    let sql = format!("select * from {} where id = $1", MC::TABLE);

    let entity: E = sqlx::query_as(&sql)
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })?;

    Ok(entity)
}

// pub async fn update<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64, data: E) -> Result<E>
// where
//     MC: DbBmc,
//     E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
// {
//     let db = mm.db();

//     let sql = format!("select * from {} where id = $1", MC::TABLE);
//     let entity: E = sqlx::query_as(&sql)
//         .bind(id)
//         .fetch_optional(db)
//         .await?
//         .ok_or(Error::EntityNotFound {
//             entity: MC::TABLE,
//             id,
//         })?;

//     Ok(entity)
// }
