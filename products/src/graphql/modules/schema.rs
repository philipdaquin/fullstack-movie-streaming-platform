use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::{redis::{get_post_cache_key, create_connection}, graphql::config::{get_redis_conn_from_ctx, get_redis_conn_manager}};

use super::model::NewProduct;
use {
    super::resolver::{get_all_products, get_product_by_id, self},
    async_graphql::*,
    async_graphql_actix_web::*,
    super::model::Product,
    crate::{graphql::config::get_conn_from_ctx},
};
use redis::{aio::ConnectionManager, Value,  AsyncCommands, RedisError};


#[derive(Default)]
pub struct QueryProducts;

///  The Price Type in our System 
#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct ProductType { 
    pub id: ID,
    pub name: String, 
    /// Price attribute can be Zero 
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

// pub struct UserType { 
//     pub id: ID
// }
// #[Object(extends)]
// impl UserType { 
//     #[graphql(external)]
//     pub async fn id(&self) -> &ID { &self.id}

//     #[graphql(name = "getUserProducts")]
//     pub async fn products(&self, ctx: &Context<'_>) -> Vec<ProductType> { 
//         resolver::get_all_products_by_id(self.id.parse::<i32>().unwrap(), &get_conn_from_ctx(ctx))
//             .expect("")
//             .iter()
//             .map(|f| ProductType::from(f))
//             .collect()
//     }
// }


#[Object(extends)]
impl QueryProducts { 

    /// Reference Resolver for Products
    #[graphql(entity)]
    pub async fn find_product_by_id(&self, ctx: &Context<'_>, id: ID) -> Option<ProductType> { 
        find_product_by_id_internal(ctx, id)
    }
    /// Get all products found inside the Database
    #[graphql(name = "getAllProducts")]
    pub async fn get_all(&self, ctx: &Context<'_>) -> Vec<ProductType> { 
        resolver::get_all_products(&get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| ProductType::from(f))
            .collect()
    }
    #[graphql(name = "getProductById")]
    pub async fn get_product_by_id(&self, ctx: &Context<'_>, id: ID) -> Option<ProductType> { 
        let cache_key = get_post_cache_key(id.to_string().as_str());
        let redis_client = get_redis_conn_from_ctx(ctx).await;
        let mut redis_connection = create_connection(redis_client)
            .await
            .expect("Unable to get redis connection");

        let cached_object: Value = redis_connection
            .get(cache_key.clone())
            .await
            .expect("Redis Error on client");
        match cached_object { 
            Value::Nil => { 
                log::info!("Unable to find cache under this product id, accessing Database.. 😂");

                let product = find_product_by_id_internal(ctx, id);
                let _: () = redis::pipe()
                    .atomic()
                    .set(&cache_key, product.clone())
                    .expire(&cache_key, 60)
                    .query_async(&mut redis_connection)
                    .await
                    .expect("Internal Error Occurred while attempting to cache the object");
                return product
            },
            Value::Data(cache) => { 
                log::info!("Cache Found Under this product Id! 👌");
                serde_json::from_slice(&cache).expect("Unable to Deserialize Struct")
            },
            _ => { None }
        }
    }

    #[graphql(name = "getShippingEstimate")]
    pub async fn shipping_estimate(&self, ctx: &Context<'_>, id: ID) -> Option<i32> { 
        let ProductType { 
            price, 
            weight, .. 
        }  = find_product_by_id_internal(ctx, id).unwrap(); 
        Some(price.unwrap_or_default() * weight.unwrap_or_default())
    } 
    #[graphql(name = "getProductsByCategory")]
    pub async fn get_by_category(&self, ctx: &Context<'_>, category: String) -> FieldResult<Vec<ProductType>> { 
        Ok(resolver::filter_by_category(category, &get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| ProductType::from(f))
            .collect()
        )
    }
    #[graphql(name = "getProductsByTags")]
    pub async fn get_by_tags(&self, ctx: &Context<'_>, tag: String) -> FieldResult<Vec<ProductType>> { 
        Ok(resolver::filter_by_tags(tag, &get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| ProductType::from(f))
            .collect()
        )
    }
    // #[graphql(entity)]
    // pub async fn find_user_by_id(&self, #[graphql(key)] id: ID) -> UserType { 
    //     UserType { id }
    // }

}
fn find_product_by_id_internal(ctx: &Context<'_>, id: ID) -> Option<ProductType> { 
    let id = id.parse::<i32>().expect("");
    resolver::get_product_by_id(id, &get_conn_from_ctx(ctx))
        .ok()
        .map(|f| ProductType::from(&f))
}

#[derive(Default)]
pub struct MutateProduct;

#[derive(InputObject, Clone, Debug)]
pub struct NewProductInput { 
    pub name: String,
    pub price: Option<i32>, 
    pub weight: Option<i32>,
    pub category: Option<String>, 
    pub created_by: Option<i32>,
    pub tags: Option<String>,
    /// users should not be able tto change the time manually 
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub image_url: Option<String>
}


#[Object]
impl MutateProduct { 
    #[graphql(name = "createNewProduct")]
    async fn create_product(&self, ctx: &Context<'_>, new_product: NewProductInput) -> Option<ProductType> { 
        let product = resolver::create_product(
            NewProduct::from(&new_product), 
            &get_conn_from_ctx(ctx))
            .expect("Unable to Execute 'Create_Product' in the Database");
        ProductType::from(&product).into()
            
    }    
    #[graphql(name = "updateProduct")]
    async fn update_product(
        &self, 
        ctx: &Context<'_>, 
        product_id: ID, 
        user_id: ID, 
        new_input: NewProductInput) -> FieldResult<ProductType> { 

        let new_product_input = NewProductInput { 
            updated_at: chrono::Utc::now().naive_utc().into(), 
            ..new_input.clone()
        };

        let product = resolver::update_product(
            product_id.clone().parse::<i32>().expect("ParseIntError"), 
            user_id.parse::<i32>().expect("ParseIntError"), 
            NewProduct::from(&new_product_input), 
            &get_conn_from_ctx(ctx)).expect("");

        // Cache invalidation once updated, this ensures data consistency 
        let cache_key = get_post_cache_key(product_id.to_string().as_str());
        get_redis_conn_manager(ctx)
            .await
            .del(cache_key)
            .await?;
        Ok(ProductType::from(&product).into())
    }
    #[graphql(name = "deleteProduct")]
    async fn delete_product(&self, ctx: &Context<'_>, product_id: ID) -> FieldResult<bool> { 
        log::info!("Invalidated Cache Key in Deleting Product ID Cache Key");
        let cache_id = get_post_cache_key(product_id.to_string().as_str());
        get_redis_conn_manager(ctx)
            .await
            .del(cache_id)
            .await?;
        Ok(resolver::delete_product(product_id.parse()?, &get_conn_from_ctx(ctx)).expect(""))
       
    }
}







