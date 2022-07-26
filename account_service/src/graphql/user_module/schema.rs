use std::str::FromStr;

use async_graphql::*;
use async_graphql_actix_web::*;
use common_utils::{error::ServiceError, generate_token};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use uuid::{Uuid, Error};
use crate::graphql::{config::{
    get_conn_from_ctx,
    get_redis_conn_from_ctx,
    get_redis_conn_manager
}, utils::verify_password};
use chrono::NaiveDateTime;
use crate::graphql::to_uuid;
use crate::graphql::user_module::{
    model::{Users, NewUser, Role},
    resolver::{UserDatabase}
};
use redis::{aio::ConnectionManager, Value,  AsyncCommands, RedisError};
use crate::redis::{ get_post_cache_key, create_connection};
use async_graphql::{validators::{email, min_length}};
use common_utils::Role as AuthRole;



#[derive(Default)]
pub struct UserQuery;

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct UserType { 
    /// UUID
    pub id: ID,
    /// Email Stype 
    pub email: String, 
    pub hash: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>, 
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub image_url: Option<String>,
    pub last_login_at: Option<NaiveDateTime>,
    pub role: String
}

#[Object(extends)]
impl UserQuery { 
    async fn test_api(&self) -> String { 
        "This is the User Service".into()
    }

    #[graphql(entity, name = "getUserByID")]
    async fn get_user(&self, ctx: &Context<'_>, #[graphql(key)] id: ID) -> FieldResult<UserType> { 
        find_user_internally(ctx, id).await
    }

    /// Get all the users
    #[graphql(name = "getAllUsers")]
    async fn get_all_users(&self, ctx: &Context<'_>) -> FieldResult<Vec<UserType>> { 
        let user = Users::get_all_users::<UserDatabase>(&get_conn_from_ctx(ctx))
            .await
            .expect("Unable to get Users")
            .iter() 
            .map(|f| UserType::from(f))
            .collect();
        Ok(user)
    }
    /// Get User either from cache or Database
    #[graphql(name = "getUserById")]
    async fn get_user_by_id(&self, ctx: &Context<'_>, user_id: ID) -> Option<UserType> { 
        let cache_key = get_post_cache_key(user_id.to_string().as_str());
        let redis_client = get_redis_conn_from_ctx(ctx).await;
        let mut redis_connection = create_connection(redis_client)
            .await
            .expect("Unable to create Redis DB connection");
        let cached_object = redis_connection.get(cache_key.clone()).await.expect("Redis Error on Client ");
        
        //  Check If Cache Object is available 
        match cached_object { 
            Value::Nil => { 
                log::info!("Unable to find cache under this id, accessing Database.. 😂");

                let user = find_user_internally(ctx, user_id)
                    .await
                    .map_err(|_| ServiceError::DatabaseError)
                    .ok()
                    .map(|e| Some(e))
                    .expect("Unablet to get the User");
                    
                let _: () = redis::pipe()
                    .atomic()
                    .set(&cache_key, user.clone())
                    .expire(&cache_key, 60)
                    .query_async(&mut redis_connection)
                    .await
                    .expect("Internal Error Occurred while attempting to cache the object");
                return user
            },
            Value::Data(cache) => { 
                log::info!("Cache Found Under this Id! 👌");
                serde_json::from_slice(&cache).expect("Unable to Deserialize Struct")
            },
            _ => { None }
        }
    }
    /// Gets the user from the cache or database
    #[graphql(name = "getUserByUsername")]
    async fn get_user_by_name(&self, ctx: &Context<'_>, username: String) -> Option<UserType> { 
        let cache_key = get_post_cache_key(username.as_str());
        let redis_client = get_redis_conn_from_ctx(ctx).await;
        let mut redis_connection = create_connection(redis_client)
            .await
            .expect("Unable to create Redis DB connection");
        let cached_object = redis_connection.get(cache_key.clone()).await.expect("Redis Error on Client ");
        
        //  Check If Cache Object is available 
        match cached_object { 
            Value::Nil => { 
                log::info!("Unable to find cache under this id, accessing Database.. 😂");

                let user = find_user_internally_by_name(ctx, username)
                    .await
                    .map_err(|_| ServiceError::DatabaseError)
                    .ok()
                    .map(|e| Some(e))
                    .expect("Unablet to get the User");
                    
                let _: () = redis::pipe()
                    .atomic()
                    .set(&cache_key, user.clone())
                    .expire(&cache_key, 60)
                    .query_async(&mut redis_connection)
                    .await
                    .expect("Internal Error Occurred while attempting to cache the object");
                return user
            },
            Value::Data(cache) => { 
                log::info!("Cache Found Under this Id! 👌");
                serde_json::from_slice(&cache).expect("Unable to Deserialize Struct")
            },
            _ => { None }
        }
    } 
}



#[derive(Default)]
pub struct UserMutation;

#[derive(InputObject)]
pub struct NewUserInput { 
    /// Input Email
    #[graphql(validator(email))]
    pub email: String, 

    /// Password; Min Password Strength 
    #[graphql(validator(min_password_strength = "1"))]
    pub hash: String,

    /// Optional Input, the values are generated automatically 
    #[graphql(visible = false)]
    pub created_at: Option<NaiveDateTime>,

    /// Optional input, the values are auto generated
    #[graphql(visible = false)]
    pub updated_at: Option<NaiveDateTime>,

    /// Username, 
    /// Min_length: 2 characters; Max_length: 50 characters
    #[graphql(validator(chars_min_length = "2", chars_max_length = "50"))]
    pub username: String, 

    /// First_name, Non-nullable, min-length is 1 
    #[graphql(validator(chars_min_length = "1", chars_max_length = "50"))]
    pub first_name: String,

    /// Last name, non-nullable, 
    /// Min-length: 1; Max-length: 50
    #[graphql(validator(chars_min_length = "1", chars_max_length = "50"))]
    pub last_name: String,

    /// User Profile image 
    /// Ideally, this image url is linked to a Static Database ie S3
    pub image_url: Option<String>,

    /// User's last_login:
    /// This is hidden from the user, however it will be automatically 
    /// updated everytime the user creates a new user and logins
    #[graphql(visible = false)]
    pub last_login_at: Option<NaiveDateTime>,

    /// Optional, default values are set to USERS, can be set 
    /// to USERS, ADMIN, CUSTOMER, OPERATOR
    #[graphql(visible = false)]    
    pub role: Option<Role>
}

#[derive(InputObject)]
pub struct UserLogin { 
    /// Ensure that the value is in email format
    #[graphql(validator(email))]
    pub email: String, 
    /// Non-nullable input for password. 
    /// Min-Char-len = 8 characters
    #[graphql(validator(chars_min_length = "8"))]
    pub password: String
}

#[Object]
impl UserMutation { 
    /// Create new users using the User Input
    #[graphql(name = "createNewUsers")]
    async fn create_user(&self, ctx: &Context<'_>, new_user: NewUserInput) -> UserType  {
        Users::create_user::<UserDatabase>(
            NewUser::from(&new_user),
            &get_conn_from_ctx(ctx)
        ).await
        .map(|f| UserType::from(&f))
        .expect("")
    }
    /// Deletes the User from the system
    #[graphql(name = "deleteUser")]
    async fn delete_user(&self, ctx: &Context<'_>, user_id: ID) -> FieldResult<bool> { 
        let result = Users::delete_user::<UserDatabase>( 
            to_uuid(user_id.to_owned())?, 
            &get_conn_from_ctx(ctx)
        )
        .await
        .expect("Unable to delete user");

        update_key_val_cache(ctx, user_id).await?;
        Ok(result)
    }
    /// Update User Detaisl
    #[graphql(name = "updateUserDetails")]
    async fn update_user_details(&self, ctx: &Context<'_>, user_id: ID, new_user: NewUserInput) -> FieldResult<UserType> { 
        let user = Users::update_user::<UserDatabase>(
            to_uuid(user_id.to_owned())?,
            NewUser::from(&new_user),
            &get_conn_from_ctx(ctx) 
        ).await.expect("Unable to update and retrieve the user").unwrap();

        //  Delete the cache under this key 
        update_key_val_cache(ctx, user_id).await?;

        Ok(UserType::from(&user))
    }
    #[graphql(name = "updateUserPassword")]
    async fn update_user_password(&self, ctx: &Context<'_>, user_id: ID, password: String) -> FieldResult<UserType> { 
        let user = Users::update_password::<UserDatabase>(
            to_uuid(user_id.to_owned())?,
            password,
            &get_conn_from_ctx(ctx)
        ).await.expect("Unable to retrieve the password and User details").unwrap();

        //  Delete the cache under this key 
        update_key_val_cache(ctx, user_id).await?;
        Ok(UserType::from(&user))
    }
    /// Logins the user, Also Updates the LastUserLogin Row for the Same User
    #[graphql(name = "loginUser")]
    async fn login_user(&self, ctx: &Context<'_>, user: UserLogin) -> Result<String, ServiceError> { 
        if let Some(user_info) = find_user_internally_by_email(ctx, user.email).await.ok() { 
            if let Ok(role) = verify_password(&user_info.clone().expect("Missing UserPassword").hash, &user.password) { 
                if role {
                    let user_role = AuthRole::from_str(user_info.clone().expect("Missing User Role").role.as_str()).expect("");
                    let token = generate_token(user_info.clone().expect("missing Username").email, user_role);

                    //  Update the last login
                    Users::update_last_login::<UserDatabase>(
                        to_uuid(user_info.clone().expect("Missing UserId").id)?, 
                        &get_conn_from_ctx(ctx)
                    ).await?;
                    log::info!("User Login {}", user_info.expect("Missing Username").username);
                    return Ok(token)
                }
            }
        }
        Err(ServiceError::NotFound)
    }
}

///  Internal Database Reads
/// Find the user by id, this function does one job and doesnt check access the caching layer
async fn find_user_internally(ctx: &Context<'_>, user_id: ID) -> FieldResult<UserType> { 
    let user = Users::get_user_by_id::<UserDatabase>(to_uuid(user_id)?, &get_conn_from_ctx(ctx))
        .await
        .expect("Unable to get Users")
        .map(|f| UserType::from(&f))
        .expect("Unable to convert UserDatabase Type into GraphQL Type");
    Ok(user)
}
async fn find_user_internally_by_name(ctx: &Context<'_>, username: String) -> FieldResult<UserType> { 
    let user = Users::get_user_by_name::<UserDatabase>(username, &get_conn_from_ctx(ctx))
        .await?
        .map(|e| UserType::from(&e))
        .expect("Unable to get Username");
    Ok(user)
}
async fn find_user_internally_by_email(ctx: &Context<'_>, email: String) -> FieldResult<Option<UserType>> { 
    let user = Users::get_user_by_email::<UserDatabase>(email, &get_conn_from_ctx(ctx))
        .await
        .expect("Unable to get User by email")
        .map(|f| UserType::from(&f));
    Ok(user)
}
/// Cache Invalidation, whenever a value is updated in the database, each cached item with a corresponding key 
/// is automativally deleted from the cache or caches
async fn update_key_val_cache(ctx: &Context<'_>, key: ID) -> Result<(), RedisError> { 
    let cache_key = get_post_cache_key(key.to_string().as_str());
    get_redis_conn_manager(ctx)
        .await
        .del(cache_key)
        .await
}