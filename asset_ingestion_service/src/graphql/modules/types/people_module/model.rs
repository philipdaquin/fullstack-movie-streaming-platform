use common_utils::{QueryResult, inc_person_id};
use scylla::{ValueList, FromRow, Session, IntoUserType, FromUserType};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use strum_macros::{EnumString, Display};
use crate::db::CachedSession;
use async_graphql::Enum;

use super::{resolver::PersonResolver, schema::{PersonType, PersonInput}};

#[derive(Debug, FromRow, Clone, Deserialize, Serialize, ValueList)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Person { 
    pub person_id: i32,
    pub name: String,
    pub awards: Vec<String>,
    pub biography: String,
    pub birthday: NaiveDate,
    pub death_date: NaiveDate,
    pub gender: String,
    pub homepage: String,
    pub known_for: Vec<String>,
    pub place_of_birth: String,
    pub profile_path: Vec<String>,
}
#[derive(Clone, Debug, ValueList, Serialize, Deserialize, FromRow, IntoUserType)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct NewPerson { 
    pub person_id: i32,
    pub name: String,
    pub awards: Vec<String>,
    pub biography: String,
    pub birthday: NaiveDate,
    pub death_date: NaiveDate,
    pub gender: String,
    pub homepage: String,
    pub known_for: Vec<String>,
    pub place_of_birth: String,
    pub profile_path: Vec<String>,
}

#[derive(Copy, Clone, Eq, Debug, PartialEq, Serialize, SmartDefault, Deserialize, Enum, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Gender { 
    #[default]
    NotConfigured,
    Male,
    Female,
    NonBinary,
    PreferToNotSay,
}

impl From<&Person> for PersonType { 
    fn from(f: &Person) -> Self {
        log::info!("Retrieving user from Database: {:#?}", f);
        Self { 
            person_id: f.person_id.into() ,
            name: f.name.clone(),
            awards: f.awards.clone() ,
            biography: f.biography.clone() ,
            birthday: f.birthday.clone() ,
            death_date: f.death_date.clone() ,
            gender: f.gender.clone() ,
            homepage: f.homepage.clone() ,
            known_for: f.known_for.clone() ,
            place_of_birth: f.place_of_birth.clone() ,
            profile_path: f.profile_path.clone() ,
        }
    }
}

impl From<&PersonInput> for NewPerson { 
    fn from(f: &PersonInput) -> Self {
        let id = inc_person_id();
        let default_value = NaiveDate::from_ymd(2015, 9, 8);
        Self { 
            person_id: id.clone() ,
            name: f.name.clone(),
            awards: f.awards.clone().unwrap_or(vec![String::new()]) ,
            biography: f.biography.clone().unwrap_or_default() ,
            birthday: f.birthday.clone().unwrap_or(default_value),
            death_date: f.death_date.clone().unwrap_or(default_value) ,
            gender: f.gender.clone().unwrap_or_default().to_string() ,
            homepage: f.homepage.clone().unwrap_or_default() ,
            known_for: f.known_for.clone().unwrap_or(vec![String::new()]) ,
            place_of_birth: f.place_of_birth.clone().unwrap_or_default() ,
            profile_path: f.profile_path.clone().unwrap_or(vec![String::new()]) ,
        }
    }
}


impl Person { 
    pub async fn get_person_by_id<PersonDatabase: PersonResolver>(session: &'static CachedSession, person_id: i32) -> QueryResult<Person> {
        PersonDatabase::get_person_by_id(session, person_id).await
    }
    pub async fn get_all_person<PersonDatabase: PersonResolver>(session: &'static CachedSession) -> QueryResult<Vec<Person>> {
        PersonDatabase::get_all_person(session).await
    }
    pub async fn get_person_by_name<PersonDatabase: PersonResolver>(session: &'static CachedSession, name: String) -> QueryResult<Person> {
        PersonDatabase::get_person_by_name(session, name).await
    }
    pub async fn create_movie_person<PersonDatabase: PersonResolver>(session: &'static CachedSession, new_person: NewPerson) -> QueryResult<Person> {
        PersonDatabase::create_movie_person(session, new_person).await
    }
    pub async fn update_movie_person<PersonDatabase: PersonResolver>(session: &'static CachedSession, person_id: i32, new_person: NewPerson) -> QueryResult<Person> {
        PersonDatabase::update_movie_person(session, person_id, new_person).await
    }
    pub async fn delete_movie_person<PersonDatabase: PersonResolver>(session: &'static CachedSession, person_id: i32, person_name: String) -> QueryResult<bool> {
        PersonDatabase::delete_movie_person(session, person_id, person_name).await
    }
}