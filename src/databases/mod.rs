use sqlx::{postgres::PgPoolOptions, PgPool, Postgres};

pub async fn start_connection() -> Pool<Postgres>{
    let postgres_env = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_env)
        .await
        .expect("Failed to connect to Postgres");
}