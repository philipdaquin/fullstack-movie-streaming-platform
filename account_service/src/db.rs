use dotenv::dotenv;
use std::{env, time::Duration};
use sqlx::{Pool, Postgres, PgConnection, postgres::{PgPoolOptions}, PgPool, Error};

pub type DbPool = PgPool;
// pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub enum DatabaseKind {
    Example,
    ExampleTest,
}
/// Pool Builder 
pub async fn init_pool(database_url: &str) -> Result<DbPool, Error> {

    let pool = PgPoolOptions::new()
        .max_connections(1000)
        .idle_timeout(Duration::new(5, 0))
        .connect(&database_url)
        .await
        .unwrap();

    Ok(pool)
}

pub async fn establish_connection(db_kind: DatabaseKind) -> DbPool {
    dotenv().ok();

    let postgres_db_host = env::var("POSTGRES_DB_HOST").expect("POSTGRES_DB_HOST must be set");
    let postgres_db = match db_kind {
        DatabaseKind::Example => env::var("POSTGRES_DB").expect("POSTGRES_DB must be set"),
        _ => env::var("POSTGRES_DB_TEST").expect("POSTGRES_DB_TEST must be set"),
    };

    let postgres_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let postgres_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");

    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        postgres_user, postgres_password, postgres_db_host, postgres_db
    );
    
    init_pool(&database_url)
    .await
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}