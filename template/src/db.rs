use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub enum DatabaseKind {
    Example,
    ExampleTest,
}

fn init_pool(database_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection(db_kind: DatabaseKind) -> DbPool {
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

    init_pool(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}