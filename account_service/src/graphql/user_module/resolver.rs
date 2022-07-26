use async_trait::async_trait;
use sqlx::{query, Error, PgPool, postgres::PgQueryResult};
use uuid::Uuid;
use crate::QueryResult;
use crate::graphql::user_module::model::{Users, NewUser};
use crate::graphql::utils::hash_password;
use chrono::Utc;

#[async_trait]
pub trait UserResolver { 
    async fn get_all_users(conn: &PgPool) -> QueryResult<Vec<Users>>;
    async fn get_user_by_id(id: Uuid, conn: &PgPool) -> QueryResult<Option<Users>>;
    async fn get_user_by_email(email: String, conn: &PgPool) -> QueryResult<Option<Users>>;
    async fn get_user_by_username(username: String, conn: &PgPool) -> QueryResult<Option<Users>>;
    async fn create_user(new_user: NewUser, conn: &PgPool) -> QueryResult<Users>;
    async fn delete_user(user_id: Uuid, conn: &PgPool) -> QueryResult<bool>;
    async fn update_user(user_id: Uuid, new_user: NewUser, conn: &PgPool) -> QueryResult<Option<Users>>;
    async fn update_password(user_id: Uuid, password: String, conn: &PgPool) -> QueryResult<Option<Users>>;
    async fn update_last_login(user_id: Uuid, conn: &PgPool) -> QueryResult<bool>;
}

pub struct UserDatabase;

#[async_trait]
impl UserResolver for UserDatabase { 
    #[tracing::instrument(skip(conn), fields(repository = "user"))]
    async fn get_all_users(conn: &PgPool) -> QueryResult<Vec<Users>> { 
        let user = sqlx::query_as!(Users, r#"SELECT * FROM users"#)
            .fetch_all(conn)
            .await?;
        Ok(user)
    }
    #[tracing::instrument(skip(conn), fields(repository = "user"))]
    async fn get_user_by_id(id: Uuid, conn: &PgPool) -> QueryResult<Option<Users>> { 
        let user = sqlx::query_as!(Users, r#"SELECT * FROM users WHERE id = $1"#, id)
            .fetch_optional(conn)
            .await?;
        Ok(user)
    }
    #[tracing::instrument(skip(conn), fields(repository = "user"))]
    async fn get_user_by_email(email: String, conn: &PgPool) -> QueryResult<Option<Users>> {
        let user = sqlx::query_as!(Users, r#"SELECT * FROM users WHERE email = $1"#, email)
            .fetch_optional(conn)
            .await?;
        Ok(user)
    }
    #[tracing::instrument(skip(conn), fields(repository = "user"))]
    async fn get_user_by_username(username: String, conn: &PgPool) -> QueryResult<Option<Users>> { 
        let user = sqlx::query_as!(Users, r#"SELECT * FROM users WHERE username = $1"#, username)
            .fetch_optional(conn)
            .await?;
        Ok(user)
    }
    #[tracing::instrument(skip(conn), fields(repository = "user"))]
    async fn create_user(new_user: NewUser, conn: &PgPool) -> QueryResult<Users> {
        let mut transaction = conn.begin().await?;
        let password = hash_password(&new_user.hash).expect("Unable to hash User's Password");
        let user_id = Uuid::new_v4();
        let updated_now = Utc::now().naive_utc();


        sqlx::query_as!(
            Users,
            r#"
            INSERT INTO users (
                id,
                email,
                hash,
                created_at,
                updated_at,
                username,
                first_name,
                last_name,
                image_url,
                last_login_at,
                role
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#, 
            user_id,
            new_user.email,
            password,
            new_user.created_at,
            new_user.updated_at,
            new_user.username,
            new_user.first_name,
            new_user.last_name,
            new_user.image_url,
            new_user.last_login_at,
            new_user.role
        ).execute(&mut transaction).await?;
        
        let user = sqlx::query_as!(
                Users, 
                r#"SELECT * FROM users where id = $1"#, 
                user_id
            )
            .fetch_one(&mut transaction)
            .await?;

        log::warn!("🎉🎉 New User Created! {:?}", user);

        transaction.commit().await?;
        Ok(user)
    }
    async fn delete_user(user_id: Uuid, conn: &PgPool) -> QueryResult<bool> {
        let mut transaction = conn.begin().await?;
        let res = sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, user_id)
            .execute(&mut transaction)
            .await?
            .rows_affected();        
        transaction.commit().await?;

        if res == 0 { 
            return Ok(false)
        }
        return Ok(true)
    }
    async fn update_user(user_id: Uuid, new_user: NewUser, conn: &PgPool) -> QueryResult<Option<Users>> {
        let mut transaction = conn.begin().await?;
        let updated_now = Utc::now().naive_utc();
        let password = hash_password(&new_user.hash).expect("Unable to hash User's Password");
        let is_updated = sqlx::query_as!(
            Users, 
            r#" UPDATE users SET 
                    email = $1,
                    hash = $2,
                    updated_at = $3,
                    username =$4,
                    first_name = $5 ,
                    last_name = $6,
                    image_url = $7
                WHERE id = $8
            "#, 
            new_user.email,
            password,
            updated_now,
            new_user.username,
            new_user.first_name,
            new_user.last_name,
            new_user.image_url,
            user_id
        )
        .execute(&mut transaction)
        .await?
        .rows_affected();
        
        if is_updated == 0 { return Ok(None); }
        let user = sqlx::query_as!(
            Users, 
            r#"SELECT * FROM users where id = $1"#, 
            user_id
        )
        .fetch_optional(&mut transaction)
        .await?;
        transaction.commit().await?;
        Ok(user)
    }

    async fn update_password(user_id: Uuid, password: String, conn: &PgPool) -> QueryResult<Option<Users>> { 
        let hash = hash_password(&password).expect("Unable to hash User's Password");
        let mut transaction = conn.begin().await?;
        let is_updated = sqlx::query_as!(
            Users, 
            r#" UPDATE users SET hash = $1 WHERE id = $2"#, 
            hash,
            user_id
        )
        .execute(&mut transaction)
        .await?
        .rows_affected();
        
        if is_updated == 0 { return Ok(None); }
        let user = sqlx::query_as!(
            Users, 
            r#"SELECT * FROM users where id = $1"#, 
            user_id
        )
        .fetch_optional(&mut transaction)
        .await?;
        transaction.commit().await?;
        Ok(user)
    }

    async fn update_last_login(user_id: Uuid, conn: &PgPool) -> QueryResult<bool> {
        let mut transaction = conn.begin().await?;
        let updated_now = Utc::now().naive_utc();

        let updated = sqlx::query_as!(Users, r#"UPDATE users SET last_login_at = $1 WHERE id = $2"#, updated_now, user_id)
            .execute(&mut transaction)
            .await?
            .rows_affected();
        log::info!("Last Login Updated? {}", updated);
        if updated != 0 { 
            return Ok(true)
        } else { 
            return Ok(false)            
        }
    }
    
}

