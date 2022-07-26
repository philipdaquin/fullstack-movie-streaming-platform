use async_graphql::*;
use async_graphql_actix_web::*;
use super::{resolver::{ProdCompanyResolver, CompanyDatabase}, 
    model::{ProductionCompany, NewProductionComp, OriginCountry}};
use crate::{graphql::{config::get_conn_from_ctx}, to_bigint};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ProductionCompanyQuery;
#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct ProductionCompanyType { 
    pub company_id: ID,
    pub movie_id: ID,
    pub description: String,
    pub headquarter: String,
    pub homepage: String,
    pub logo_path: String,
    pub name: String,
    pub origin_country: String,
    pub parent_company: String
}

#[Object(extends, cache_control(max_age = 60))]
impl ProductionCompanyQuery { 
    /// Resolver Reference for CompanyType
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(entity, name = "getCompanyEntity")]
    async fn get_company_by_id(&self, ctx: &Context<'_>, company_id: ID) -> FieldResult<ProductionCompanyType> { 
        find_company_internally(ctx, company_id).await
    }
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "getAllCompanies")]
    async fn get_all_companies(&self, ctx: &Context<'_>) -> FieldResult<Vec<ProductionCompanyType>>  {
        let res = ProductionCompany::get_all_companies::<CompanyDatabase>(get_conn_from_ctx(ctx))
            .await
            .expect("Unable to get all the companies")
            .iter()
            .map(ProductionCompanyType::from)
            .collect();

        Ok(res)
    }
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "getCompaniesByMovie")]
    async fn get_companies_by_movie(&self, ctx: &Context<'_>, movie_id: ID) -> Vec<ProductionCompanyType> { 
        find_companies_by_movies(ctx, movie_id).await
    }
}

#[derive(Default)]
pub struct ProductionCompanyMutation;

#[derive(InputObject, Debug, Deserialize, Clone)]
pub struct InputProductionCompany { 
    /// Movie id, this may not be necessary anywmore
    pub movie_id: Option<ID>,
    /// Company Name
    #[graphql(validator(max_length = 20))]
    pub name: Option<String>,
    /// Company Description
    #[graphql(validator(max_length = 30))]
    pub description: Option<String>,
    /// Company Location 
    #[graphql(validator(max_length = 10))]
    pub headquarter: Option<String>,
    /// Company Website
    #[graphql(validator(max_length = 50))]
    pub homepage: Option<String>,
    /// S3 link to image 
    #[graphql(validator(max_length = 50))]
    pub logo_path: Option<String>,
    /// Country of origin
    #[graphql(validator(max_length = 10))]
    pub origin_country: Option<String>,
    /// Parent Company
    #[graphql(validator(max_length = 25))]
    pub parent_company: Option<String>,
}

#[Object(extends)]
impl ProductionCompanyMutation { 
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "createCompany")]
    async fn create_new_company(&self, ctx: &Context<'_>, new_product: InputProductionCompany) -> FieldResult<ProductionCompanyType> { 
        let res = ProductionCompany::create_movie_company::<CompanyDatabase>(
            NewProductionComp::from(&new_product),
            get_conn_from_ctx(ctx))
            .await
            .expect("");
        Ok(ProductionCompanyType::from(&res))
    }
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "updateCompany")]
    async fn update_prod_company(&self, ctx: &Context<'_>, id: ID, new_company: InputProductionCompany) -> FieldResult<ProductionCompanyType> { 
        let res = ProductionCompany::update_movie_company::<CompanyDatabase>(
            NewProductionComp::from(&new_company),
            to_bigint(id),
            get_conn_from_ctx(ctx)
        ).await.expect("");
        Ok(ProductionCompanyType::from(&res))
    }
    #[tracing::instrument(skip(self, ctx))]
    #[graphql(name = "deleteCompany")]
    async fn delete_prod_company(&self, ctx: &Context<'_>, id: ID, company_name: String) -> FieldResult<bool> { 
        let res = ProductionCompany::delete_movie_company::<CompanyDatabase>(
            to_bigint(id),
            company_name,
            get_conn_from_ctx(ctx)
        )
        .await
        .expect("");
        Ok(res)
    }
}

/// Helper functions
#[tracing::instrument(skip(ctx))]
async fn find_company_internally(ctx: &Context<'_>, id: ID) -> FieldResult<ProductionCompanyType> { 
    let company = ProductionCompany::get_company_id::<CompanyDatabase>(to_bigint(id), &get_conn_from_ctx(ctx))
        .await
        .ok()
        .map(|f| ProductionCompanyType::from(&f))
        .expect("Unable to get the Company by id");
    Ok(company)
}

#[tracing::instrument(skip(ctx))]
pub async fn find_companies_by_movies(ctx: &Context<'_>, movie_id: ID) -> Vec<ProductionCompanyType> { 
    let company = ProductionCompany::get_companies_movie::<CompanyDatabase>(
        get_conn_from_ctx(ctx), 
        to_bigint(movie_id))
        .await
        .expect("")
        .iter()
        .map(ProductionCompanyType::from)
        .collect();
    company
}

pub async fn create_new_company(ctx: &Context<'_>, new_product: InputProductionCompany) -> ProductionCompanyType { 
    let res = ProductionCompany::create_movie_company::<CompanyDatabase>(
        NewProductionComp::from(&new_product),
        get_conn_from_ctx(ctx))
        .await.expect("");
    ProductionCompanyType::from(&res)
}