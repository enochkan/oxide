use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub async fn init_storage() -> Pool<Sqlite> {
    SqlitePoolOptions::new()
        .connect("sqlite://oxide.db")
        .await
        .expect("Failed to initialize database")
}

pub async fn save_task_status(pool: &Pool<Sqlite>, task_id: &str, status: &str) {
    sqlx::query("INSERT INTO task_status (task_id, status) VALUES (?, ?)")
        .bind(task_id)
        .bind(status)
        .execute(pool)
        .await
        .expect("Failed to save task status");
}
