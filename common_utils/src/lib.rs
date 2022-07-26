#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate thiserror;

pub mod error;

use std::{env::var, str::FromStr};
use actix_web::{HttpResponse, HttpRequest};
use chrono::{Duration, Local, NaiveDate};
use error::ServiceError;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, DecodingKey, 
    TokenData, Validation,
    errors::{Error as JsonError, ErrorKind}, 
    EncodingKey, Header};
use strum_macros::{Display, EnumString};

lazy_static! {
    static ref JWT_SECRET_KEY: String = var("JWT_SECRET_KEY").expect("JWT Secret Key Error");
}
use snowflake::SnowflakeIdGenerator;
use parking_lot::Mutex;
use serial_int::SerialGenerator;


/// Generate Unique identifier for Movies and Companies 
pub fn unique_id(machine_id: i32, node_id: i32) -> i64 { 
    let snow = Mutex::new(SnowflakeIdGenerator::new(machine_id, node_id));
    let id = snow.lock().real_time_generate();
    id
}
///  Generate a new int value for simple ids
lazy_static! {
    static ref GENRE_ID_GEN: Mutex<SerialGenerator> = Mutex::new(SerialGenerator::new());
    static ref PERSON_ID_GEN: Mutex<SerialGenerator> = Mutex::new(SerialGenerator::new());
    static ref COUNTRY_ID_GEN: Mutex<SerialGenerator> = Mutex::new(SerialGenerator::new());
    pub static ref KAFKA_CONSUMER_COUNTER: Mutex<SerialGenerator> = Mutex::new(SerialGenerator::new());
}
/// Autoincrements id for new genre
pub fn inc_genre() -> i32 { 
    GENRE_ID_GEN.lock().generate() as i32
}
/// autogenerates the person id 
pub fn inc_person_id() -> i32 { 
    PERSON_ID_GEN.lock().generate() as i32
}

pub fn inc_country() -> i32 { 
    COUNTRY_ID_GEN.lock().generate() as i32
}
/// For Kafka, each message in a partition is assigned a sequential id called an offset
/// This uses Parking-Lot's Mutex in order to prevent issues faced when using STD mutex locks 
pub fn kafka_counter() -> i32 { 
    KAFKA_CONSUMER_COUNTER.lock().generate() as i32 
}


// default date when date is empty
pub fn default_date() -> NaiveDate { 
    let date = Mutex::new(Local::now());
    let s = date.lock().date().naive_local();
    s
}

pub type QueryResult<T> = std::result::Result<T, ServiceError>; 

#[derive(Serialize, Deserialize, Debug)]
pub struct Claim { 
    issuer: String, 
    subject: String,
    // Timestamp of when these tokens were issued
    issued_at: i64,
    expiry: i64, 
    login_session: String
}

#[derive(Eq, PartialEq, Display, EnumString, Copy, Clone)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Role { 
    Admin, 
    User,
    Customer, 
    Operator
}

pub fn generate_token(username: String, role: Role) -> String { 
    let issuer = var("DOMAIN")
        .unwrap_or_else(|_| "LocalHost".to_string());
    let expiry = (Local::now() + Duration::minutes(60)).timestamp();
    let now = Local::now().timestamp();

    let payload = Claim {
        issuer,
        subject: username,
        issued_at: now,
        expiry,
        login_session: role.to_string(),
    };
    encode(
        &Header::default(),
        &payload, 
        &EncodingKey::from_secret(&JWT_SECRET_KEY.as_ref())
    ).expect("Could not generate JWT Claim")
}

pub fn decode_token(token: &str) -> Result<TokenData<Claim>, JsonError> { 
    Ok(decode::<Claim>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
        &Validation::default(),
    )?)
}

pub fn get_role(req: HttpRequest) -> Option<Role> { 
    req
    .headers()
    .get("Authorization")
    .and_then(|header|  { 
        header.to_str().ok().map(|ch| { 
            let jwt_start_index = "Bearer".len();
            let jwt = ch[jwt_start_index..ch.len()].to_string();
            let token_data = decode_token(&jwt).expect("JsonError");
            Role::from_str(&token_data.claims.login_session).expect("ParseError")
        })
    })
}