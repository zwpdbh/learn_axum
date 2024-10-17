use crate::model::ModelManager;
use crate::model::{Error, Result};
use crate::Ctx;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::base::{self, DbBmc};

// region:      --- Task Types
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}
// endregion:   --- Task Types

// region:      --- TaskBmc (BackendModelController)
pub struct TaskBmc;

impl DbBmc for TaskBmc {
    const TABLE: &'static str = "task";
}

impl TaskBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<i64> {
        let db = mm.db();
        let (id,) =
            sqlx::query_as::<_, (i64,)>("INSERT into task (title) values ($1) returning id")
                .bind(task_c.title)
                .fetch_one(db)
                .await?;

        Ok(id)
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn update(
        _ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        task_u: TaskForUpdate,
    ) -> Result<()> {
        let db = mm.db();

        match task_u.title {
            None => Ok(()),
            Some(title) => {
                let sql = format!("UPDATE task SET title = '{}' WHERE id = $1 ", title);
                let count = sqlx::query(&sql)
                    .bind(id)
                    .execute(db)
                    .await?
                    .rows_affected();

                if count == 0 {
                    Err(Error::EntityNotFound {
                        entity: Self::TABLE,
                        id,
                    })
                } else {
                    Ok(())
                }
            }
        }
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

// endregion:   --- TaskBmc (BackendModelController)

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use crate::_dev_utils;

    use super::*;
    use anyhow::{Ok, Result};
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test(1).await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        let task_c = TaskForCreate {
            title: fx_title.to_string(),
        };

        let id = TaskBmc::create(&ctx, &mm, task_c).await?;

        // Check
        let task = TaskBmc::get(&ctx, &mm, id).await?;
        assert_eq!(task.title, fx_title);

        // Cleanup
        TaskBmc::delete(&ctx, &mm, id).await?;
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test(1).await;
        let ctx: Ctx = Ctx::root_ctx();
        let fix_id = 100;

        let res = TaskBmc::get(&ctx, &mm, fix_id).await;
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        let mm = _dev_utils::init_test(1).await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_update_ok - task 01";
        let fx_title_new = "test_update_ok -- task 01 new";
        let fx_task = _dev_utils::seed_tasks(&ctx, &mm, &[fx_title])
            .await?
            .remove(0);

        // -- Exec
        let _ = TaskBmc::update(
            &ctx,
            &mm,
            fx_task.id,
            TaskForUpdate {
                title: Some(fx_title_new.to_string()),
            },
        )
        .await?;

        // -- Check
        let task = TaskBmc::get(&ctx, &mm, fx_task.id).await?;
        assert_eq!(task.title, "test_update_ok -- task 01 new");

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        let mm = _dev_utils::init_test(1).await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &["test_list_ok-task 01", "test_list_ok-task 02"];
        let _ = _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

        // Exec
        let tasks = TaskBmc::list(&ctx, &mm).await?;

        // Check
        let tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|t| t.title.starts_with("test_list_ok-task"))
            .collect();
        assert_eq!(2, tasks.len(), "number of seed tasks");

        // Clean
        for task in tasks.iter() {
            let _ = TaskBmc::delete(&ctx, &mm, task.id).await?;
        }

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test(1).await;
        let ctx: Ctx = Ctx::root_ctx();
        let fix_id = 100;

        let res = TaskBmc::get(&ctx, &mm, fix_id).await;
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
