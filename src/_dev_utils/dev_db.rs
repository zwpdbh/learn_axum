use std::{fs, path::PathBuf, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

type Db = Pool<Postgres>;

const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db";

const SQL_RECREATE_DB: &str = "sql/dev_inital/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_inital";

pub async fn init_dev_db(max_connection: u32) -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");
    {
        let root_db = new_db_pool_for_test(PG_DEV_POSTGRES_URL, max_connection).await?;
        let _ = pexec(&root_db, SQL_RECREATE_DB).await?;
    }

    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    let _ = paths.sort();

    let app_db = new_db_pool_for_test(PG_DEV_APP_URL, max_connection).await?;
    for path in paths {
        if let Some(path) = path.to_str() {
            let path = path.replace('\\', "/"); // for windows

            if path.ends_with(".sql") && path != SQL_RECREATE_DB {
                let _ = pexec(&app_db, &path).await?;
            }
        }
    }

    Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    info!("{:<12} - pexec: {file}", "FOR-DEV-ONLY");
    let content = fs::read_to_string(file)?;

    // FIXME: Make  the split more sql proof
    let sqls: Vec<&str> = content.split(';').collect();
    for sql in sqls {
        let _ = sqlx::query(sql).execute(db).await?;
    }

    Ok(())
}

async fn new_db_pool_for_test(db_con_url: &str, max_connection: u32) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connection)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_con_url)
        .await
}
