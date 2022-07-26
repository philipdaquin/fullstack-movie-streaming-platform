use dotenv::dotenv;
use futures::StreamExt;
use strum_macros::{EnumString, Display};
use std::{env, time::Duration, ffi::FromBytesWithNulError, sync::Arc};
use scylla::{self,IntoTypedRows, query::Query, Session, SessionBuilder, batch::Consistency, load_balancing::{DcAwareRoundRobinPolicy, TokenAwarePolicy}, CachingSession, frame::value::ValueList, QueryResult, SessionConfig, transport::{iterator::RowIterator, session::KnownNode}, ValueList};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use std::fmt::Debug;



use crate::{server::{is_new_database, enable_tracing}, modules::model::RecommendedMovies};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type DbPool = CachedSession;

// Cassandra/ Scylla DB types
lazy_static! { 
    static ref KEYSPACE_NAME: String = std::env::var("KEYSPACE_NAME").expect("Expected a Keyspace name");
    static ref SCYLLA_URL: String = std::env::var("CASSANDRA_URL").unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    static ref NODE2: String = std::env::var("NODE2").unwrap_or_else(|_| "127.0.0.1:9043".to_string());
    static ref NODE3: String = std::env::var("NODE3").unwrap_or_else(|_| "127.0.0.1:9043".to_string());
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
        .known_node(NODE2.as_str())
        .known_node(NODE3.as_str())
        .connection_timeout(Duration::from_secs(10))
        .load_balancing(policy)
        .default_consistency(consistency_lvl)
        .user(USER.to_string(), PASSWORD.to_string())
        .build()
        .await?;
    
    // Cache current Session for faster queries 
    let cached = CachedSession::from(session);
    //  Store Current Session as a global variable
    let _ = DBCONN.set(cached);

    /* There may be a better way of ensuring the server doesn't panic here... */
    if is_new_database() { 
        log::info!("Initialising a new Scylla Cluster...");
        initialise_pool().await?
    } else { 
        log::info!("Database already exist, connecting to the current cluster");
        //  This could mean that the database already exists
        connect_to_existing_db().await?;
    }
    
    Ok(DBCONN.get().unwrap())
}
/// Keyspace is a collection of tables with attributed that define how data is replicated 
/// across nodes. It defines a number of options that apply rto all the tables it contains 
/// To get the session, you must first establish a connection to ScyllaDb using 
/// 'establish_connection'
pub async fn create_keyspace() -> Result<()> { 
    dotenv::dotenv().ok();

    session()
        .query(create_keyspace_query().as_str(), ())
        .await
        .map(|_| ())
        .expect("Unable to create a new keyspace ");
    Ok(())
}
/// This will let you create a new table anywhere, ensure you write your queries 
/// in the right format,ie
/// static CREATE_TEMPERATURE_TABLE_QUERY: &str = r#"
///   CREATE TABLE IF NOT EXISTS tutorial.temperature (
///     device UUID,
///     time timestamp,
///     temperature smallint,
///     PRIMARY KEY(device, time)
///   );
/// "#;
pub async fn create_tables() -> Result<()> { 
    log::info!("Creating tables for Scylla!");
    for tables in include_str!("./.././queries.cql").split(';') { 
        let tables = tables.trim();
        log::info!("{}", tables.to_string());
        if tables.is_empty() { 
            continue;
        }
        session().query(tables, ()).await?;
        log::info!("Final CQL {}", tables.to_string());
    }
    Ok(())
}

/// Drops the table in the keyspace 
pub async fn drop_table(keyspace: &str, table_name: &str, session: &Session) -> Result<()> { 
    let query = format!(r#"DROP TABLE IF EXISTS {}.{};"#, keyspace, table_name);
    session.query(query, ()).await?;
    
    Ok(())
}

/// If a database already exists then we simple connect to an existing db
async fn connect_to_existing_db() -> Result<Session> { 
    let hostnode = KnownNode::Hostname(SCYLLA_URL.to_string());
    let (auth_username, auth_password) = ( 
        Some(USER.to_string()), Some(PASSWORD.to_string()));
    let cfg = SessionConfig::new();
    let cfg =  SessionConfig { 
        known_nodes: vec![hostnode],
        auth_username,
        auth_password
        ,..cfg
    };  
    log::info!("🛫🛫 Connecting to: {}", DATACENTER.to_string());

    let existing_session = Session::connect(cfg).await?;
    existing_session.use_keyspace(KEYSPACE_NAME.as_str(), false).await?;

    Ok(existing_session)
}
/// This function initialises a new keyspace based off the user's specification
pub async fn initialise_pool() -> Result<()> { 
    //  create keyspace 
    create_keyspace().await.expect("❌❌ Keyspace already exist!");
    //  Create all tables and types 
    create_tables().await.expect("❌❌ Database Already Exist");
    Ok(())
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

        let mut simple_query = Query::new(query);
        simple_query.set_tracing(true);

        let result = self.0.execute(simple_query, &values)
            .await
            .expect("Error executing queries on Scylla DB");
        //  Query tracing info from system_traces.sessions and system_traces.events
        if enable_tracing() { 
            if let Some(id) = result.tracing_id { 
                let tracing_info = session()
                    .0
                    .session
                    .get_tracing_info(&id)
                    .await?;
                log::warn!("🧑‍🍳 Tracing Info: {:#?}", tracing_info)
            }
        }
        Ok(result)
    }
    ///  Executes a paged query 
    pub async fn query_iter(&self, query: &str, values: impl ValueList + Debug) -> Result<RowIterator> { 
        log::info!("Preparing and paging new statemet");
        let result = self
            .0
            .execute_iter(Query::from(query), &values)
            .await
            .expect("Failed to prepare a query with paging");
        Ok(result)
    }
    /// Executes a prepared query 
    /// Always use prepared statements for any of your request 
    /// When using a prepared statements the client does a prepare phase, where the request is parsed and upon 
    /// execution only binds the values to the statemetn identifier
    pub async fn query_prepared(&self, query: &str, values: impl ValueList + Debug) -> Result<QueryResult> { 
        log::info!("Preparing and executing statement: {:#?}", query);

        let mut prepared = Query::from(query);
        prepared.set_tracing(enable_tracing());

        let result = self
            .0
            .execute(prepared, &values)
            .await
            .map_err(|f| log::warn!("{:#?}", f))
            .expect("Error executing a prepared query");
        //  Query tracing info from system_traces.sessions and system_traces.events
        if enable_tracing() { 
            if let Some(id) = result.tracing_id { 
                let tracing_info = session()
                    .0
                    .session
                    .get_tracing_info(&id)
                    .await?;
                log::warn!("Tracing Info: {:#?}", tracing_info)
            }
        }
        Ok(result)
    }
}
// Using Non-Batch, Batch Inserts can be too big and fail 
static BATCH_INSERT_RECOMMENDATIONS: &str = "
    INSERT INTO recommended_movies.user_recommendations (
        user_id, movie_id, created_at, title
    ) VALUES (?, ?, ?, ?);
";
// Insert all items to database
#[tracing::instrument(skip(session), fields(repository = "recommended_movies.user_recommendations"), err)]
pub async fn stream_insert(movie: Vec<RecommendedMovies>, session: &'static CachedSession) -> Result<bool> { 
    let mut stream = futures::stream::iter(movie);

    while let Some(movie) = stream.next().await { 
        log::info!("🛬 Streaming {:#?} into the Database ", movie);
        let _ = session
            .query_prepared(BATCH_INSERT_RECOMMENDATIONS, (movie))
            .await
            .expect("Unable to stream movies");
    }
    Ok(true)
}
