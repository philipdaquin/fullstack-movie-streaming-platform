use async_graphql::*;
use serde::{Deserialize, Serialize};
use crate::graphql::config::get_conn_from_ctx;
use crate::{to_int, to_bigint};
use chrono::NaiveDate;
use super::resolver::PersonDatabase;
use super::model::{Person, Gender, NewPerson};
#[derive(Default)]
pub struct PersonQuery;
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct PersonType { 
    pub person_id: ID,
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
#[Object(extends, cache_control(max_age = 40))]
impl PersonQuery { 
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "getAllPersons")]
    async fn get_all_person(&self, ctx: &Context<'_>) -> FieldResult<Vec<PersonType>> { 
        let response = Person::get_all_person::<PersonDatabase>(get_conn_from_ctx(ctx))
            .await
            .expect("")
            .iter()
            .map(|f| PersonType::from(f))
            .collect();
        Ok(response)
    }
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "getPersonByName")]
    async fn get_person_info_by_name(&self, ctx: &Context<'_>, person_name: String) -> FieldResult<PersonType> { 
        let response = Person::get_person_by_name::<PersonDatabase>(get_conn_from_ctx(ctx), person_name)
            .await
            .ok()
            .map(|f| PersonType::from(&f))
            .unwrap();
        Ok(response)
    }
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "getPersonById")]
    async fn get_person_info_by_id(&self, ctx: &Context<'_>, person_id: ID) -> FieldResult<PersonType> { 
        let response = Person::get_person_by_id::<PersonDatabase>(get_conn_from_ctx(ctx), to_int(person_id))
            .await
            .ok()
            .map(|f| PersonType::from(&f))
            .unwrap();
        Ok(response)
    }

}

#[derive(Default)]
pub struct PersonMutation;


#[derive(InputObject, Debug, Deserialize, Serialize)]
pub struct PersonInput {
    /// Person name 
    pub name: String,
    /// List of awards under the user name 
    pub awards: Option<Vec<String>>,
    /// Biography of actor/actress 
    pub biography: Option<String>,
    /// Birthday YYYY-MM-DD
    pub birthday: Option<NaiveDate>,
    /// Death date YYYY-MM-DD
    pub death_date: Option<NaiveDate>,
    /// Gender, based on Enum
    pub gender: Option<Gender>,
    /// Person person website
    pub homepage: Option<String>,
    /// Movie characters or other names of person
    pub known_for: Option<Vec<String>>,
    /// Person place of birth
    pub place_of_birth: Option<String>,
    /// S3 profile image link
    pub profile_path: Option<Vec<String>>,
}
#[Object]
impl PersonMutation {   
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "createPerson")]
    async fn create_person(&self, ctx: &Context<'_>, new_person: PersonInput) -> FieldResult<PersonType> { 
        let new_person = Person::create_movie_person::<PersonDatabase>(
            get_conn_from_ctx(ctx), 
        NewPerson::from(&new_person))
        .await
        .expect("Unable to get any response from the server");
        Ok(PersonType::from(&new_person))
    }

    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "updatePerson")]
    pub async fn update_person(&self, ctx: &Context<'_>, person_id: ID, new_person: PersonInput)  -> FieldResult<PersonType> { 
        let res = Person::update_movie_person::<PersonDatabase>(
            get_conn_from_ctx(ctx),
            to_int(person_id),
            NewPerson::from(&new_person))
            .await
            .expect("Unable to get any response from the database");
        Ok(PersonType::from(&res))
    }
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "deleteGenre")]
    pub async fn delete_genre(&self, ctx: &Context<'_>, person_id: ID, person_name: String) -> FieldResult<bool> { 
        let res = Person::delete_movie_person::<PersonDatabase>(
            get_conn_from_ctx(ctx),
            to_int(person_id),
            person_name)
            .await
            .expect("Unable to get any response from the database");
        Ok(res)
    }
}

