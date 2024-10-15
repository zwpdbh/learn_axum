use crate::model::ModelManager;
use crate::model::Result;
use seder::{Deserialize, Serialize};
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

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

impl TaskBmc {
    pub async fn create() -> Result<i64> {
        todo!()
    }
}

// endregion:   --- TaskBmc (BackendModelController)
