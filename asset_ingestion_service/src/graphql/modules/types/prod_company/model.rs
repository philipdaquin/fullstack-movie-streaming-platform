use async_graphql::Enum;
use common_utils::QueryResult;
use common_utils::error::ServiceError;
use scylla::{FromRow, ValueList};
use scylla::macros::{FromUserType, IntoUserType};
use serde::{Serialize, Deserialize};
use strum_macros::{EnumString, Display};
use crate::db::CachedSession;
use super::resolver::ProdCompanyResolver;
use super::schema::{ProductionCompanyType, InputProductionCompany};
// use scylla::frame::value::{MaybeUnset, Unset};
use crate::{generate_unique_id, to_bigint};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, ValueList)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct ProductionCompany { 
    pub company_id: i64,
    pub name: String,
    pub description: String,
    pub headquarter: String,
    pub homepage: String,
    pub logo_path: String,
    pub movie_id: i64,
    pub origin_country: String,
    pub parent_company: String
}

#[derive(Debug, Clone, IntoUserType, ValueList, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct NewProductionComp { 
    pub company_id: i64,
    pub name: String,
    pub description: String,
    pub headquarter: String,
    pub homepage: String,
    pub logo_path: String,
    pub movie_id: i64,
    pub origin_country: String,
    pub parent_company: String
}

#[derive(Copy, Clone, Eq, SmartDefault, PartialEq, Serialize, Deserialize, Enum, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OriginCountry { 
    Us, 
    Aus, 
    Uk,
    #[default]
    NoCountrySelected
}
//  Database Type into GraphQl Type 
impl From<&ProductionCompany> for ProductionCompanyType { 
    fn from(f: &ProductionCompany) -> Self {
        log::info!("Converting type into GraphqlType {:#?}", f);
        Self { 
            company_id: f.company_id.into(),
            movie_id: f.movie_id.into(),
            name: f.name.clone(),
            description: f.description.clone(),
            logo_path: f.logo_path.clone(),
            headquarter: f.headquarter.clone(),
            origin_country: f.origin_country.clone(),
            parent_company: f.parent_company.clone().into(),
            homepage: f.homepage.clone(),
        }
    }
}
impl From<&InputProductionCompany> for NewProductionComp { 
    fn from(f: &InputProductionCompany) -> Self {
        log::info!("Inserting new values into database {:#?}", f);

        let company_id = generate_unique_id();
        Self {
            company_id,
            movie_id: to_bigint(f.movie_id.clone().unwrap_or_default()),
            name: f.name.clone().unwrap_or_default(),
            description: f.description.clone().unwrap_or_default(),
            logo_path: f.logo_path.clone().unwrap_or_default(),
            headquarter: f.headquarter.clone().unwrap_or_default(),
            homepage: f.homepage.clone().unwrap_or_default(),
            origin_country: f.origin_country.clone().unwrap_or_default(),
            parent_company: f.parent_company.clone().unwrap_or_default().to_string(),
        }
    }
}


impl ProductionCompany { 
    #[tracing::instrument(skip(session))]
    pub async fn get_company_id<CompanyDatabase: ProdCompanyResolver>(id: i64, session: &'static CachedSession) -> QueryResult<ProductionCompany> {
        CompanyDatabase::get_company_id(id, session).await
    }
    #[tracing::instrument(skip(session))]
    pub async fn get_company_details<CompanyDatabase: ProdCompanyResolver>(name: String, session: &'static CachedSession) -> QueryResult<ProductionCompany> {
        CompanyDatabase::get_company_details(name, session).await
    }

    #[tracing::instrument(skip(session))]
    pub async fn get_all_companies<CompanyDatabase: ProdCompanyResolver>(session: &'static CachedSession) -> QueryResult<Vec<ProductionCompany>> {
        CompanyDatabase::get_all_companies(session).await
    }
    #[tracing::instrument(skip(session))]
    pub async fn get_companies_movie<CompanyDatabase: ProdCompanyResolver>(session: &'static CachedSession, movie_id: i64) -> QueryResult<Vec<ProductionCompany>>  {
        CompanyDatabase::get_companies_movie(session, movie_id).await
    }
    #[tracing::instrument(skip(session))]
    pub async fn create_movie_company<CompanyDatabase: ProdCompanyResolver>(new_company: NewProductionComp, session: &'static CachedSession) -> QueryResult<ProductionCompany> {
        CompanyDatabase::create_movie_company(new_company, session).await
    }
    #[tracing::instrument(skip(session))]
    pub async fn update_movie_company<CompanyDatabase: ProdCompanyResolver>(new_company: NewProductionComp, id: i64, session: &'static CachedSession) -> QueryResult<ProductionCompany> {
        CompanyDatabase::update_movie_company(new_company, id, session).await
    }
    #[tracing::instrument(skip(session))]
    pub async fn delete_movie_company<CompanyDatabase: ProdCompanyResolver>(id: i64, company_name: String, session: &'static CachedSession) -> QueryResult<bool> {
        CompanyDatabase::delete_movie_company(id, company_name, session).await
    }
}
