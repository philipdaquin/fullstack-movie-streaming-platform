use crate::schema::products;
use chrono::{NaiveDateTime, NaiveDate};
use serde::{Serialize, Deserialize};
use super::schema::{ProductType, NewProductInput};

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[table_name = "products"]
pub struct Product { 
    pub id: i32, 
    pub name: String,
    pub price: Option<i32>, 
    pub weight: Option<i32>,
    pub category: Option<String>, 
    pub created_by: Option<i32>,
    pub tags: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub image_url: Option<String>
}

#[derive(Debug, Clone, AsChangeset, Insertable)]
#[table_name = "products"]
pub struct NewProduct { 
    pub name: String,
    pub price: Option<i32>, 
    pub weight: Option<i32>,
    pub category: Option<String>, 
    pub created_by: Option<i32>,
    pub tags: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub image_url: Option<String>
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShoppingCart { 
    id: i32,
    session_id: i32, 
    product_id: i32, 
    quantity: i32, 
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime 
}

#[derive(Clone, Debug)]
pub struct ShoppingSession { 
    id: i32, 
    user_id: i32, 
    total: i32, 
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}


impl From<&Product> for ProductType { 
    fn from(f: &Product) -> Self {
        ProductType { 
            id: f.id.into(), 
            name: f.name.to_string(), 
            price: f.price.clone(), 
            weight: f.weight.clone(),
            category: f.category.clone(),
            created_by: f.created_by.clone(),
            tags: f.tags.clone(),
            created_at: chrono::Utc::now().naive_utc().into(), 
            updated_at: f.updated_at.clone(), 
            description: f.description.clone(), 
            image_url: f.image_url.clone()
        }
    }
}

impl From<&NewProductInput> for NewProduct { 
    fn from(f: &NewProductInput) -> Self {
        Self { 
            name: f.name.to_owned(), 
            price: f.price,
            weight: f.weight,
            category: f.category.to_owned(),
            created_by: f.created_by,
            tags: f.tags.to_owned(),
            created_at: f.created_at.clone(),
            updated_at: f.updated_at,
            description: f.description.to_owned(),
            image_url: f.image_url.to_owned()
        }
    }
}