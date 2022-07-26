use async_trait::async_trait;
use crate::graphql::profile_module::model::{NewProfile, Profiles};
use uuid::Uuid;
use sqlx::PgPool;
use crate::QueryResult;
use chrono::Utc;
#[async_trait]
pub trait ProfileResolver { 
    async fn get_profiles_by_owner(user_id: Uuid, conn: &PgPool) -> QueryResult<Vec<Profiles>>;
    async fn get_profile_by_id(profile_id: Uuid, conn: &PgPool) -> QueryResult<Profiles>;
    async fn get_profile_by_name(username: String, conn: &PgPool) -> QueryResult<Profiles>;
    async fn create_new_profile(new_profile: NewProfile, conn: &PgPool) -> QueryResult<Profiles>;
    async fn update_profile_user(user_id: Uuid, profile_id: Uuid, new_profile: NewProfile, conn: &PgPool) -> QueryResult<Option<Profiles>>;
    async fn delete_profile_by_user(user_id: Uuid, profile_id: Uuid, conn: &PgPool) -> QueryResult<bool>;
}

pub struct ProfileDatabase;

#[async_trait]
impl ProfileResolver for ProfileDatabase { 
    async fn get_profiles_by_owner(user_id: Uuid, conn: &PgPool) -> QueryResult<Vec<Profiles>> {
        let profile = sqlx::query_as!(Profiles, r#"SELECT * FROM profiles WHERE id = $1"#, user_id)
            .fetch_all(conn)
            .await?;
        Ok(profile)
    }
    async fn get_profile_by_id(profile_id: Uuid, conn: &PgPool) -> QueryResult<Profiles> {
        let profile = sqlx::query_as!(Profiles, r#"SELECT * FROM profiles WHERE profile_id = $1"#, profile_id)
            .fetch_one(conn)
            .await?;
        Ok(profile)
    }
    async fn get_profile_by_name(username: String, conn: &PgPool) -> QueryResult<Profiles> {
        let profile = sqlx::query_as!(Profiles, r#"SELECT * FROM profiles WHERE username = $1"#, username)
            .fetch_one(conn)
            .await?;
        Ok(profile)
    }
    async fn create_new_profile(new_profile: NewProfile, conn: &PgPool) -> QueryResult<Profiles> {
        let mut transaction = conn.begin().await?;
        let NewProfile {id, username, updated_at, ..} = new_profile;
        let profile_id = Uuid::new_v4();
        let created_at = Utc::now().naive_utc();

        let _ = sqlx::query_as!(Profiles, r#"INSERT INTO profiles (
            profile_id,
            id,
            username,
            created_at,
            updated_at
        ) VALUES ($1, $2, $3, $4, $5)"#,
            profile_id,
            id, 
            username, 
            created_at, 
            updated_at
        ).execute(&mut transaction).await?;

        let profile = sqlx::query_as!(Profiles, r#"SELECT * FROM profiles WHERE profile_id = $1"#, profile_id)
            .fetch_one(&mut transaction)
            .await?;
        log::warn!("New Profile Created: {}", username);
        transaction.commit().await?;
        Ok(profile)
    }
    async fn update_profile_user(user_id: Uuid, profile_id: Uuid, new_profile: NewProfile, conn: &PgPool) -> QueryResult<Option<Profiles>> {
        let mut transaction = conn.begin().await?;
        let NewProfile {
            username, updated_at, ..
        } = new_profile;
        let updated_profile = sqlx::query_as!(Profiles, r#"UPDATE profiles SET
            username = $1, 
            updated_at = $2
            WHERE id = $3 AND profile_id = $4
        "#, username, updated_at, user_id, profile_id)
        .execute(&mut transaction).await?.rows_affected();
        
        if updated_profile == 0 { return Ok(None)}
        log::info!("Updating Profile for the username: {username}");

        let profile = sqlx::query_as!(Profiles, r#"SELECT * FROM profiles WHERE profile_id = $1"#, profile_id)
            .fetch_optional(&mut transaction)
            .await?;
        Ok(profile)
    }
    async fn delete_profile_by_user(user_id: Uuid, profile_id: Uuid, conn: &PgPool) -> QueryResult<bool> {
        let mut transaction = conn.begin().await?;
        let deleted = sqlx::query_as!(Profiles, r#"DELETE FROM profiles WHERE profile_id = $1 AND id = $2"#, profile_id, user_id)
            .execute(&mut transaction)
            .await?
            .rows_affected();
        if deleted == 0 { 
            return Ok(false)
        }
        return Ok(true)
    }
}