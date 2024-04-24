use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

pub async fn init_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:db.sqlite3?mode=rwc")
        .await
        .expect("Failed to connect to database");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            completed BOOLEAN NOT NULL DEFAULT 0,
            created_at DATETIME NOT NULL DEFAULT (datetime('now')),
            updated_at DATETIME NOT NULL DEFAULT (datetime('now'))
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create todos table");

    pool
}
