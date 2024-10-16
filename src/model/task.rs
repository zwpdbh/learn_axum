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

    pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        let db = mm.db();
        let tasks: Vec<Task> = sqlx::query_as("select * from task order by id")
            .fetch_all(db)
            .await?;

        Ok(tasks)
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        let db = mm.db();
        let count = sqlx::query("DELETE from  task where id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound { entity: "task", id });
        }

        Ok(())
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
        let mm = _dev_utils::init_test().await;
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
        let mm = _dev_utils::init_test().await;
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
    async fn test_list_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
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
        let mm = _dev_utils::init_test().await;
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
