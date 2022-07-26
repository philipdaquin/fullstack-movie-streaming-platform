use async_graphql::Enum;
use chrono::{NaiveDateTime, Utc};
use sqlx::{PgPool};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use super::{
    schema::{UserType, NewUserInput},
    resolver::{UserDatabase, UserResolver},
};
use std::str::FromStr;
use strum_macros::{EnumString, Display};

use crate::QueryResult;
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Users { 
    pub id: Uuid, 
    pub email: String, 
    pub hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>, 
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub image_url: Option<String>,
    pub last_login_at: Option<NaiveDateTime>,
    pub role: String
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct NewUser { 
    pub email: String, 
    pub hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub username: String, 
    pub first_name: String,
    pub last_name: String,
    pub image_url: Option<String>,
    pub last_login_at: NaiveDateTime,
    pub role: String
}

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Enum, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Role { 
    Admin, 
    Customer,
    Operator,
    User
}

impl From<&Users> for UserType {
    fn from(f: &Users) -> Self {
        Self { 
            id: f.id.into(),
            email: f.email.clone(),
            hash: f.hash.clone(),
            created_at: Some(f.created_at),
            updated_at: f.updated_at.clone(),
            username: f.username.clone(),
            first_name: f.first_name.clone(),
            last_name: f.last_name.clone(),
            image_url: f.image_url.clone(),
            last_login_at: f.last_login_at,
            role: Role::from_str(f.role.as_str()).unwrap_or(Role::User).to_string()
        }
    }
}

impl From<&NewUserInput> for NewUser { 
    fn from(f: &NewUserInput) -> Self {
        let now = Utc::now().naive_utc();

        Self { 
            email: f.email.clone(),
            hash: f.hash.clone(),
            created_at: f.created_at.unwrap_or(now),
            updated_at: f.updated_at.clone(),
            username: f.username.clone(),
            first_name: f.first_name.clone(),
            last_name: f.last_name.clone(),
            image_url: f.image_url.clone(), 
            last_login_at: f.last_login_at.unwrap_or(now),
            role: f.role.unwrap_or(Role::User).to_string()
        }
    }
}

impl Users { 
    #[tracing::instrument(skip(conn), err)]
    pub async fn get_all_users<UserDatabase: UserResolver>(conn: &PgPool) -> QueryResult<Vec<Self>> { 
        UserDatabase::get_all_users(conn).await
    }
    #[tracing::instrument(skip(conn), err)]
    pub async fn get_user_by_id<UserDatabase: UserResolver>(id: Uuid, conn: &PgPool) -> QueryResult<Option<Self>> { 
        UserDatabase::get_user_by_id(id, conn).await
    }
    #[tracing::instrument(skip(conn), err)]
    pub async fn create_user<UserDatabase: UserResolver>(new_user: NewUser, conn: &PgPool) -> QueryResult<Users> { 
        UserDatabase::create_user(new_user, conn).await
    }
    #[tracing::instrument(skip(conn), err)]
    pub async fn get_user_by_name<UserDatabase: UserResolver>(username: String, conn: &PgPool) -> QueryResult<Option<Self>> { 
        UserDatabase::get_user_by_username(username, conn).await
    }
    #[tracing::instrument(skip(conn), err)]
    pub async fn delete_user<UserDatabase: UserResolver>(user_id: Uuid, conn: &PgPool) -> QueryResult<bool> { 
        UserDatabase::delete_user(user_id, conn).await
    }
    #[tracing::instrument(skip(conn), err)]
    pub async fn update_user<UserDatabase: UserResolver>(user_id: Uuid, new_user: NewUser, conn: &PgPool) -> QueryResult<Option<Self>> { 
        UserDatabase::update_user(user_id, new_user, conn).await
    }
    #[tracing::instrument(skip(conn), err)]
    pub async fn update_password<UserDatabase: UserResolver>(user_id: Uuid, password: String, conn: &PgPool) -> QueryResult<Option<Self>> { 
        UserDatabase::update_password(user_id, password, conn).await
    }
    #[tracing::instrument(skip(conn), err)]
    pub async fn get_user_by_email<UserDatabase: UserResolver>(email: String, conn: &PgPool) -> QueryResult<Option<Users>> { 
        UserDatabase::get_user_by_email(email, conn).await
    }
    #[tracing::instrument(skip(conn), err)]
    pub async fn update_last_login<UserDatabase: UserResolver>(user_id: Uuid, conn: &PgPool) -> QueryResult<bool> { 
        UserDatabase::update_last_login(user_id, conn).await
    }
}