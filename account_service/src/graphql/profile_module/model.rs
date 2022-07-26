use chrono::{NaiveDateTime, Utc};
use sqlx::{PgPool};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use super::schema::{ProfileType, NewProfileInput};
use crate::QueryResult;
use super::resolvers::{ProfileResolver, ProfileDatabase};
use crate::graphql::to_uuid;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Profiles { 
    pub profile_id: Uuid,
    pub id: Uuid,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct NewProfile { 
    pub id: Uuid,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

impl From<&Profiles> for ProfileType { 
    fn from(f: &Profiles) -> Self {
        Self  { 
            profile_id: f.profile_id.into(),
            id: f.id.into(),
            username: f.username.clone(),
            created_at: f.created_at,
            updated_at: f.updated_at
        }
    }
}


impl From<&NewProfileInput> for NewProfile { 
    fn from(f: &NewProfileInput) -> Self {
        Self { 
            id: to_uuid(f.user_id.clone()).expect("Unable to get the user id"),
            username: f.username.clone(), 
            created_at: f.created_at, 
            updated_at: f.updated_at
        }
    }
}


impl Profiles { 
    pub async fn get_profiles_by_owner<ProfileDatabase: ProfileResolver>(user_id: Uuid, conn: &PgPool) -> QueryResult<Vec<Profiles>> {
        ProfileDatabase::get_profiles_by_owner(user_id, conn).await
    }
    pub async fn get_profile_by_id<ProfileDatabase: ProfileResolver>(profile_id: Uuid, conn: &PgPool) -> QueryResult<Profiles> {
        ProfileDatabase::get_profile_by_id(profile_id, conn).await
    }
    pub async fn get_profile_by_name<ProfileDatabase: ProfileResolver>(username: String, conn: &PgPool) -> QueryResult<Profiles> {
        ProfileDatabase::get_profile_by_name(username, conn).await
    }
    pub async fn create_new_profile<ProfileDatabase: ProfileResolver>(new_profile: NewProfile, conn: &PgPool) -> QueryResult<Profiles> {
        ProfileDatabase::create_new_profile(new_profile, conn).await
    }
    pub async fn update_profile_user<ProfileDatabase: ProfileResolver>(user_id: Uuid, profile_id: Uuid, new_profile: NewProfile, conn: &PgPool) -> QueryResult<Option<Profiles>> {
        ProfileDatabase::update_profile_user(user_id, profile_id, new_profile, conn).await
    }
    pub async fn delete_profile_by_user<ProfileDatabase: ProfileResolver>(user_id: Uuid, profile_id: Uuid, conn: &PgPool) -> QueryResult<bool> {
        ProfileDatabase::delete_profile_by_user(user_id, profile_id, conn).await
    }
}