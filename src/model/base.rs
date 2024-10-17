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

pub async fn list<MC, E>(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<E>>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let db = mm.db();
    let sql = format!("select * from {} order by id", MC::TABLE);
    let entities: Vec<E> = sqlx::query_as(&sql).fetch_all(db).await?;

    Ok(entities)
}

pub async fn delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
    MC: DbBmc,
{
    let db = mm.db();
    let sql = format!("DELETE from {} where id = $1", MC::TABLE);
    let count = sqlx::query(&sql)
        .bind(id)
        .execute(db)
        .await?
        .rows_affected();

    if count == 0 {
        Err(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}
