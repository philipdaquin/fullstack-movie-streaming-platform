use influx_db_client::{Point, Precision, Value, Error as InfluxDbError, Client as InfluxClient, Series};
use lazy_static::lazy_static;
use url::Url;
use reqwest::Client;
use crate::server::recreate_database;
use once_cell::sync::OnceCell;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub static INFLUX: OnceCell<InfluxDBClient> = OnceCell::new();

#[inline]
pub(crate) fn influx_client() -> &'static InfluxDBClient { 
    INFLUX.get().expect("Missing Session for Influx")
}
#[derive(Clone, Debug)]
pub struct InfluxDBClient(pub influx_db_client::Client);
impl From<InfluxClient> for InfluxDBClient { 
    fn from(f: InfluxClient) -> Self {
        Self(f)
    }
}

lazy_static! { 
    /// Hostname to connect to InfluxDB, defaults to 'localhost:8086'
    static ref INFLUXDB_URL: String = std::env::var("INFLUXDB_URL").unwrap_or_else(|_| "http://localhost:8086".into());
    /// Name of an organisation for a group of users 
    /// All dashboardsm tasks, buckets, members belong to an organisation 
    static ref INFLUXDB_ORG: String = std::env::var("INFLUXDB_ORG").unwrap_or_else(|_| "influx".into());
    /// Tokens to authenticate requests to influx DB, including requests to write, query and manage data and resources
    static ref INFLUXDB_TOKEN: String = std::env::var("INFLUXDB_TOKEN").expect("Unable to get Influx DB Token");
    /// A named locations where time series data is stored 
    static ref INFLUXDB_BUCKET: String = std::env::var("INFLUXDB_BUCKET").expect("Unable to get Influx DB Bucket");
    /// User to connect, defaults to 'root'
    static ref INFLUXDB_USER: String = std::env::var("INFLUX_USER").unwrap_or_else(|_| "influx".into());
    /// Passowrd of the user, defaults to 'root'
    static ref INFLUXDB_PASSWORD: String = std::env::var("INFLUXDB_PASSWORD").unwrap_or_else(|_| "password".into());
    /// Database name to connect to, defaults to None
    static ref INLFUXDB_DBNAME: String  = std::env::var("INLFUXDB_DBNAME").unwrap_or_else(|_| "None".into());
}
/// Establish InfluxDbClient connection to Influx DB
pub async fn create_client() -> Result<&'static InfluxDBClient> { 
    let url = Url::parse(INFLUXDB_URL.as_ref()).expect("Unable to parse INFLUXDB_URL");
    let client = InfluxClient::new(url, INLFUXDB_DBNAME.as_str())
        .set_authentication(INFLUXDB_USER.to_string(),INFLUXDB_PASSWORD.to_string())
        .set_jwt_token(INFLUXDB_TOKEN.as_str());
    // client.grant_admin_privileges(INFLUXDB_USER.as_str()).await?;
    // Set global client 
    println!("{:#?}", client);
    let _ = INFLUX.set(InfluxDBClient::from(client.clone()));
    //  Drops and recreates the database under the same Bucket Name
    if recreate_database() { 
        new_database(client.clone()).await.expect("Unable to recreate a specfied database")
    }
    Ok(INFLUX.get().unwrap())
} 

/// Drop a database from InfluxDB
pub async fn new_database(client: InfluxClient) -> Result<()> { 
    log::info!("🔥 Dropping database: {}", INLFUXDB_DBNAME.as_str());
    client.drop_database(INLFUXDB_DBNAME.as_str()).await?;
    log::info!("🧑‍🔬 Recreating database: {}", INLFUXDB_DBNAME.as_str());
    client.create_database(INLFUXDB_DBNAME.as_str()).await?;
    Ok(())
}

impl InfluxDBClient { 
    /// Write data to InfluxDB
    pub async fn write(&self, point: Point, precision: Option<Precision>, retention_policy: Option<&str>) -> Result<()> { 
        log::info!("Writing Point: {:#?} to Influx DB", point);
        // let url = Url::parse(INFLUXDB_URL.as_ref()).expect("Unable to parse INFLUXDB_URL");
        // let client = InfluxClient::new(url, INFLUXDB_BUCKET.as_str().to_string())
        // .set_authentication(INFLUXDB_USER.to_string(),INFLUXDB_PASSWORD.to_string());
        let _ = influx_client()
            .0
            .write_point(point, precision, retention_policy)
            .await
            .expect("Unable to write to Influx DB");
        Ok(())
    }
    /// Send a query to Influx DB
    pub async fn query(&self, query: &str, precision: Option<Precision>) -> Result<Option<Vec<Series>>> {
        log::info!("📦 Query results for {query} ");
        // let url = Url::parse(INFLUXDB_URL.as_ref()).expect("Unable to parse INFLUXDB_URL");
        // let client = InfluxClient::new(url, INFLUXDB_BUCKET.as_str().to_string());
        let res = influx_client()
            .0
            .query(query, precision)
            .await
            .expect("Unable to get response from Influx DB")
            .unwrap_or_default()[0]
            .series
            .clone();
        log::info!("🛬 Received Response for Database! -- {:#?}", res);
        Ok(res)
    }
}