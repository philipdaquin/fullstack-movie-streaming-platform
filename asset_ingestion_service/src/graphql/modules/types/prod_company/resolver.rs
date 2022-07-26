use std::collections::HashMap;

use async_graphql::dataloader::Loader;
use async_trait::async_trait;
use scylla::IntoTypedRows;
use crate::db::CachedSession;
use super::model::{NewProductionComp, ProductionCompany};
use common_utils::{QueryResult, error::ServiceError};
use super::schema::ProductionCompanyType;


// GraphQl DataLoader   
// Improves the performance of your Graphql Query. Dataloader supports
// batching and caching functional capabilities
pub struct CompanyDetailsLoader {
    pub pool:  &'static CachedSession
}
#[async_trait]
impl Loader<String> for CompanyDetailsLoader {
    type Value = ProductionCompanyType;
    type Error = ServiceError;
    
    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        log::info!("🧭 Accessing Dataloader");
        let res = self
            .pool
            .query_prepared(GET_COMPANY_DETAILS, (keys, ))
            .await
            .expect("")
            .rows_or_empty()
            .into_typed::<ProductionCompany>()
            .next()
            .expect("")
            .into_iter()
            .map(|company_list| (company_list.name.clone(), ProductionCompanyType::from(&company_list)))
            .collect::<HashMap<_, _>>();
        Ok(res)
    }
}

#[async_trait]
pub trait ProdCompanyResolver { 
    async fn get_company_id(id: i64, session: &'static CachedSession) -> QueryResult<ProductionCompany>; 
    async fn get_all_companies(session: &'static CachedSession) -> QueryResult<Vec<ProductionCompany>>;
    async fn get_company_details(name: String, session: &'static CachedSession) -> QueryResult<ProductionCompany>;
    async fn get_companies_movie(session: &'static CachedSession, movie_id: i64) -> QueryResult<Vec<ProductionCompany>>;
    async fn create_movie_company(new_company: NewProductionComp, session: &'static CachedSession) -> QueryResult<ProductionCompany>;
    async fn update_movie_company(new_company: NewProductionComp, id: i64, session: &'static CachedSession) -> QueryResult<ProductionCompany>;
    async fn delete_movie_company(id: i64, company_name: String, session: &'static CachedSession) -> QueryResult<bool>;
}
pub struct CompanyDatabase;
static GET_COMPANY_BY_ID: &str = "SELECT * FROM movie_keyspace.movie_company WHERE company_id = ?;";
static GET_ALL_COMPANY: &str = "SELECT * FROM movie_keyspace.movie_company;";
static CREATE_COMPANY: &str = "
    INSERT INTO movie_keyspace.movie_company ( 
        company_id,
        name,
        description,
        headquarter,
        homepage,
        logo_path,
        movie_id,
        origin_country,
        parent_company
    ) VALUES ( ?, ?, ?, ?, ?, ?, ?, ?, ? );";
static GET_COMPANY_BY_MOVIE: &str = "SELECT * FROM movie_keyspace.movie_company WHERE movie_id = ?";
static UPDATE_COMPANY_DETAILS: &str = "
    UPDATE movie_keyspace.movie_company SET
        name = ?,
        description = ?,
        headquarter = ?,
        logo_path = ?,
        origin_country = ?,
        parent_company = ?  
        WHERE company_id = ? 
";
static DELETE_COMPANY: &str = "DELETE FROM movies_keyspace.movie_company WHERE company_id = ? AND name = ?";
static GET_COMPANY_DETAILS: &str = "SELECT * FROM from movie_keyspace.movie_company WHERE name = ?;";

#[async_trait]
impl ProdCompanyResolver for CompanyDatabase { 
    
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.movie_company"))]
    async fn get_company_id(id: i64, session: &'static CachedSession) -> QueryResult<ProductionCompany> { 
        let res = session.query_prepared(GET_COMPANY_BY_ID, (id,))
            .await
            .expect("")
            .rows_or_empty()
            .into_typed::<ProductionCompany>()
            .next()
            .expect("")
            .expect("");
        Ok(res)
    }
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.movie_company"))]
    async fn get_companies_movie(session: &'static CachedSession, movie_id: i64) -> QueryResult<Vec<ProductionCompany>> { 
        let res = session.query_prepared(GET_COMPANY_BY_MOVIE, (movie_id, ))
            .await
            .expect("")
            .rows
            .unwrap_or_default()
            .into_typed::<ProductionCompany>()
            .map(|f| f.expect(""))
            .collect();
        Ok(res)
    }
    
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.movie_company"))]
    async fn get_all_companies(session: &'static CachedSession) -> QueryResult<Vec<ProductionCompany>> {
        let res = session.query_prepared(GET_ALL_COMPANY, ())
            .await
            .expect("")
            .rows
            .unwrap_or_default()
            .into_typed::<ProductionCompany>()
            .map(|f| f.expect(""))
            .collect();
        Ok(res)
    }
    /// Used by the dataloaders
    /// NOTE: Dataloaders have a builtin Caching layer that prevents frequent look ups to the database
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.movie_company"))]
    async fn get_company_details(name: String, session: &'static CachedSession) -> QueryResult<ProductionCompany> {
        let res = session
            .query_prepared(GET_COMPANY_DETAILS, (name, ))
            .await
            .expect("")
            .rows_or_empty()
            .into_typed::<ProductionCompany>()
            .next()
            .expect("")
            .expect("");
        Ok(res)
    }


    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.movie_company"))]
    async fn create_movie_company(new_company: NewProductionComp, session: &'static CachedSession) -> QueryResult<ProductionCompany> {
        let response = session
            .query_prepared(CREATE_COMPANY, new_company.clone())
            .await
            .expect("Unable to insert new company");
            
        log::info!("The company details 🍺🍺 {:#?}", response);
        CompanyDatabase::get_company_id(new_company.company_id, session).await        
    }
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.movie_company"))]
    async fn update_movie_company(new_company: NewProductionComp, id: i64, session: &'static CachedSession) -> QueryResult<ProductionCompany> {
        let response = session
            .query_prepared(
                UPDATE_COMPANY_DETAILS, 
                (new_company.name, 
                new_company.description, 
                new_company.headquarter,
                new_company.logo_path, 
                new_company.origin_country, 
                new_company.parent_company, 
                id 
            ))
            .await
            .expect("Unable to update the company");
        log::info!("The company details 🍺🍺 {:#?}", response);
        CompanyDatabase::get_company_id(id, session).await
    }
    #[tracing::instrument(skip(session), fields(repository = "movie_keyspace.movie_company"))]
    async fn delete_movie_company(id: i64, company_name: String, session: &'static CachedSession) -> QueryResult<bool> {
        let response = session
            .query_prepared(DELETE_COMPANY, (id, company_name))
            .await.expect("Unable to Delete the values under the key ");
        log::info!("The company details 🍺🍺 {:#?}", response);
        Ok(true)
    }
}
