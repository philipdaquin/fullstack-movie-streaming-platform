use dotenv::dotenv;
use strum_macros::{EnumString, Display};
use std::{env, time::Duration, ffi::FromBytesWithNulError, sync::Arc};
use scylla::{self,IntoTypedRows, query::Query, Session, SessionBuilder, batch::Consistency, load_balancing::{DcAwareRoundRobinPolicy, TokenAwarePolicy}, CachingSession, frame::value::ValueList, QueryResult, SessionConfig, transport::iterator::RowIterator};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use std::fmt::Debug;


pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type DbPool = CachedSession;

// Cassandra/ Scylla DB types
lazy_static! { 
    static ref KEYSPACE_NAME: String = std::env::var("KEYSPACE_NAME").expect("Expected a Keyspace name");
    static ref SCYLLA_URL: String = std::env::var("CASSANDRA_URL").unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    static ref REPLICATION_STRATEGY: String = std::env::var("REPLICATION_STRATEGY").expect("Expected a valid Replication Strategy");
    static ref REPLICATION_FACTOR: String = std::env::var("REPLICATION_FACTOR").expect("Invalid Replication Factor");
    static ref CONSISTENCY_LEVEL: String = std::env::var("CONSISTENCY_LEVEL").expect("Invalid Consistency Level");
    static ref SNITCH: String = std::env::var("SNITCH").expect("Invalid Snitch Strategy");
    static ref DATACENTER: String = std::env::var("DATACENTER").unwrap_or_else(|_| "replication_factor".to_owned());
    static ref WRITES: String = std::env::var("DURABLE_WRITES").expect("Unable to get Durable Writes");
    static ref USER: String = std::env::var("CASSANDRA_USER").expect("Unable to get username");
    static ref PASSWORD: String = std::env::var("CASSANDRA_PASSWORD").expect("Unable to get database password");
}

pub static DBCONN: OnceCell<CachedSession> = OnceCell::new();

#[inline]
pub(crate) fn session() -> &'static CachedSession { 
    DBCONN.get().expect("Missing Session")
}

#[derive(Eq, PartialEq, Display, EnumString)]
enum ReplicationStrategy  {
    SimpleStrategy,
    NetworkTopologyStrategy,
}

// Table Queries:
//  This is temporary for now, the plan is to be able to initialise Scylla on Env only 
// static CREATE_KEYSPACE_QUERY: &str = r#"
//     CREATE KEYSPACE {KEYSPACE_NAME} WITH REPLICATION = {'class': '{REPLICATION_STRATEGY}', '{DATACENTER}' : 3} 
//     AND durable_writes = true;"#;
fn create_keyspace_query() -> String { 
    let replication_strategy = match REPLICATION_STRATEGY.as_str() { 
        "NetworkTopologyStrategy" => ReplicationStrategy::NetworkTopologyStrategy,
        _ =>  ReplicationStrategy::SimpleStrategy,
    };
    let is_durable = match WRITES.as_str() { 
        "true" => true,
        _ => false
    };

    let keyspace_name = std::env::var("KEYSPACE_NAME").expect("Expected a Keyspace name");
    let datacenter = std::env::var("DATACENTER").unwrap_or_else(|_| "replication_factor".to_string());
    let replication_factor = std::env::var("REPLICATION_FACTOR").ok().and_then(|port| port.parse::<i32>().ok()).unwrap_or(3);

    format!("
        CREATE KEYSPACE {} WITH REPLICATION = {{'class': '{}', '{}' : {}}} AND durable_writes = {};",
        keyspace_name,
        replication_strategy.to_string(),
        datacenter, 
        replication_factor, 
        is_durable
    )
}
/// This establishes the connection your nodes, you can add more nodes as you wish 
/// Ensure that you have clearly scartated your Scylla DB attributes
pub async fn establish_connection() -> Result<&'static CachedSession> {
    dotenv::dotenv().ok();
    let consistency_lvl = match CONSISTENCY_LEVEL.as_str() { 
        "Any" => Consistency::Any,
        "One" => Consistency::One,
        "Two" => Consistency::Two,
        "Three" => Consistency::Three,
        "Quorum" => Consistency::Quorum,
        "All" => Consistency::All,
        "LocalQuorum" => Consistency::LocalQuorum,
        "EachQuorum" => Consistency::EachQuorum,
        "LocalOne" => Consistency::LocalOne,
        _ => Consistency::Any
    };
    log::info!("👉 Welcome to Scylla Db 🍺. Your configured keyspace: {}", create_keyspace_query());
    log::info!("🎉 Consistency Level is set to: {}", consistency_lvl);
    log::info!("👉 User: {} password: {}", USER.to_string(), PASSWORD.to_string());
    
    // This policy will try to calculate a token to find replica nodes in which queried data is stored. After 
    //  finding the replicas it chooses the ones from the local datacenter and performs a round robin on them 
    let dc_robin = Box::new(DcAwareRoundRobinPolicy::new(DATACENTER.to_string()));
    let policy = Arc::new(TokenAwarePolicy::new(dc_robin));
    let session = SessionBuilder::new()
        .known_node(SCYLLA_URL.as_str())
        .connection_timeout(Duration::from_secs(10))
        .load_balancing(policy)
        .default_consistency(consistency_lvl)
        .user(USER.to_string(), PASSWORD.to_string())
        .build()
        .await?;
    //  Connect to an existing keyspace created on the 'asset_ingestion_service'
    session.use_keyspace(KEYSPACE_NAME.as_str(), false).await?;
    // Cache current Session for faster queries 
    let cached = CachedSession::from(session);
    //  Store Current Session as a global variable
    let _ = DBCONN.set(cached);

    log::info!("Database already exist, connecting to the current cluster");
    connect_to_existing_db().await?;
    
    Ok(DBCONN.get().unwrap())
}
/// Drops the table in the keyspace 
pub async fn drop_table(keyspace: &str, table_name: &str, session: &Session) -> Result<()> { 
    let query = format!(r#"DROP TABLE IF EXISTS {}.{};"#, keyspace, table_name);
    session.query(query, ()).await?;
    
    Ok(())
}

/// If a database already exists then we simple connect to an existing db
async fn connect_to_existing_db() -> Result<Session> { 
    let mut cfg = SessionConfig::new();
    cfg.add_known_node(SCYLLA_URL.as_str());
    let existing_session = Session::connect(cfg).await?;
    existing_session.use_keyspace(KEYSPACE_NAME.as_str(), false).await?;

    Ok(existing_session)
}
// Provides auto caching whiile executing queries
pub struct CachedSession(pub CachingSession);
impl From<Session> for CachedSession { 
    fn from(f: Session) -> Self {
        Self(CachingSession::from(f, 100))
    }
}
impl CachedSession { 
    /// Simple query
    pub async fn query(&self, query: &str, values: impl ValueList + Debug) -> Result<QueryResult> { 
        log::info!("Executing query from Scylla Database!");
        let result = self.0.execute(query, &values)
            .await
            .expect("Error executing queries on Scylla DB");
        Ok(result)
    }
    ///  NOTE: Executes a paged query 
    /// source: scylla.com
    /// ``Without paging, the coordinator node prepares a single result entitity that holds all the data 
    /// and returns it. IN the case of a large resilt, this may have a significant performance impace as it might 
    /// use up alot of memory, both for the client and on the database side.
    /// 
    /// After the query_iter, the driver starts a background task that fetches subseuqent rows. The caller 
    /// and the background task run concurrently, so one of them can fetch new rows while the other consumes them 
    /// By adding paging to the app, you reduce memory usage and increase the applicaiton performance ``
    pub async fn query_iter(&self, query: &str, values: impl ValueList + Debug, page_size: Option<i32>) -> Result<RowIterator> { 
        log::info!("Preparing and paging new statemet");
        //  Set the page size: 'Display n Number of query results'
        let mut query = Query::from(query);
        query.set_page_size(page_size.unwrap_or(10));

        let result = self
            .0
            .execute_iter(query, &values)
            .await
            .expect("Failed to prepare a query with paging");
        Ok(result)
    }
    /// Executes a prepared query 
    pub async fn query_prepared(&self, query: &str, values: impl ValueList + Debug) -> Result<QueryResult> { 
        log::info!("Preparing and executing statement: {}", query);
        let result = self
            .0
            .execute(Query::from(query), &values)
            .await
            .expect("Error executing a prepared query");
        Ok(result)
    }

    /// Passing the paging state manually
    /// This extracts the paging state from the resul and manually pass it on to the next query
    /// In doing so, the next query will stat fetching the results form where the previous one left off
    pub async fn query_paged(&self, query: &str, values: impl ValueList + Debug, page_size: Option<i32>) -> Result<QueryResult> { 
        log::info!("📖 Manually fetching a single page");

        let mut query = Query::from(query);
        query.set_page_size(page_size.unwrap_or(6));

        // Extract where the previous query left off
        let previous_query = self
            .0
            .execute(query.clone(), &values)
            .await
            .expect("Failed to get the previous state");
        // Query the next batch
        let result = self
            .0
            .execute_paged(query, &values, previous_query.paging_state)
            .await
            .expect("Error executing paged query");

        Ok(result)
    }
}


