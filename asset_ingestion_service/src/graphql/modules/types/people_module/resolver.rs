use async_trait::async_trait;
use common_utils::QueryResult;
use scylla::IntoTypedRows;
use super::model::{NewPerson, Person};
use crate::db::CachedSession;


#[async_trait]
pub trait PersonResolver { 
    async fn get_person_by_id(session: &'static CachedSession, person_id: i32) -> QueryResult<Person>;
    async fn get_all_person(session: &'static CachedSession) -> QueryResult<Vec<Person>>;
    async fn get_person_by_name(session: &'static CachedSession, name: String) -> QueryResult<Person>;
    async fn create_movie_person(session: &'static CachedSession, new_person: NewPerson) -> QueryResult<Person>;
    async fn update_movie_person(session: &'static CachedSession, person_id: i32, new_person: NewPerson) -> QueryResult<Person>;
    async fn delete_movie_person(session: &'static CachedSession, person_id: i32, person_name: String) -> QueryResult<bool>;
}

static GET_ALL_PERSON: &str = "SELECT * FROM movie_keyspace.person_object;";
static GET_PERSON_BY_NAME: &str = "SELECT * FROM movie_keyspace.person_object WHERE name = ?;";
static GET_PERSON_BY_ID: &str = "SELECT * FROM movie_keyspace.person_object WHERE person_id = ?";
static CREATE_MOVIE_PERSON: &str = "
    INSERT INTO movie_keyspace.person_object (
        person_id,
        name,
        awards,
        biography,
        birthday,
        death_date,
        gender,
        homepage,
        known_for,
        place_of_birth,
        profile_path
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
";
static UPDATE_MOVIE_PERSON: &str = "
    UPDATE movie_keyspace.person_object SET
        name = ?,
        awards = ?,
        biographgy = ?,
        death_date = ?,
        gender = ?,
        known_for = ?,
        place_of_birth = ?,
        profile_path = ?
        WHERE person_id = ? ;
";
static DELETE_MOVIE_PERSON: &str = "DELETE FROM movie_keyspace.person_object WHERE person_id = ? AND name = ?;";

pub struct PersonDatabase;

#[async_trait]
impl PersonResolver for PersonDatabase { 
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.person_object"))]
    async fn get_person_by_id(session: &'static CachedSession, person_id: i32) -> QueryResult<Person> {
        let response = session
            .query_prepared(GET_PERSON_BY_ID, (person_id,))
            .await
            .expect("Unable to get Person details")
            .rows
            .unwrap_or_default()
            .into_typed::<Person>()
            .next()
            .unwrap()
            .unwrap();
        Ok(response)

    }

    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.person_object"))]
    async fn get_all_person(session: &'static CachedSession) -> QueryResult<Vec<Person>> {
        log::info!("Getting all available persons in database");
        let response = session
            .query_prepared(GET_ALL_PERSON, ())
            .await
            .expect("Database Error")
            .rows
            .unwrap_or_default()
            .into_typed::<Person>()
            .map(|f| f.expect("Unable to query all person"))
            .collect();
        log::info!("Database Response: {:#?}", response);
        Ok(response)
            
    }
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.person_object"))]
    async fn get_person_by_name(session: &'static CachedSession, name: String) -> QueryResult<Person> {
        let response = session
            .query_prepared(GET_PERSON_BY_NAME, (name,))
            .await
            .expect("Unable to retrieve user by name")
            .rows
            .unwrap_or_default()
            .into_typed::<Person>()
            .next()
            .unwrap()
            .unwrap();
        log::info!("Database response {:#?}", response);
        Ok(response)
    }
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.person_object"))]
    async fn create_movie_person(session: &'static CachedSession, new_person: NewPerson) -> QueryResult<Person> {
        let response = session 
            .query_prepared(CREATE_MOVIE_PERSON, (new_person.clone()))
            .await
            .expect("Unable to create Person");
        log::info!("Creating a new Person {:#?}", response);
        
        PersonDatabase::get_person_by_id(session, new_person.person_id).await
    }
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.person_object"))]
    async fn update_movie_person(session: &'static CachedSession, person_id: i32, new_person: NewPerson) -> QueryResult<Person> {
        let response = session 
            .query_prepared(
                UPDATE_MOVIE_PERSON,
                (new_person.name,
                new_person.awards,
                new_person.biography,
                new_person.death_date, 
                new_person.gender,
                new_person.known_for,
                new_person.place_of_birth,
                new_person.profile_path,
                person_id
            ))
            .await
            .expect("Unable to process Update Query");
            log::info!("Updating user: {:#?}", response);
        PersonDatabase::get_person_by_id(session, new_person.person_id).await
    }
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.person_object"))]
    async fn delete_movie_person(session: &'static CachedSession, person_id: i32, person_name: String) -> QueryResult<bool> {
        let response = session
            .query_prepared(DELETE_MOVIE_PERSON, (person_id, person_name))
            .await
            .is_ok();
        Ok(response)
    }
}



