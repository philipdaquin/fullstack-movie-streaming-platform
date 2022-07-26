use async_graphql::*;
use chrono::NaiveDateTime;
use crate::graphql::{to_uuid, config::get_conn_from_ctx};
use super::{model::{Profiles, NewProfile}, resolvers::ProfileDatabase};
use serde::{Deserialize, Serialize};
#[derive(Default)]
pub struct ProfileQuery;

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct ProfileType { 
    pub profile_id: ID,
    pub id: ID,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

#[Object]
impl ProfileQuery { 
    #[graphql(name = "getProfilesFromUser")]
    async fn get_profiles(&self, ctx: &Context<'_>, user_id: ID) -> FieldResult<Vec<ProfileType>> { 
        let profile = Profiles::get_profiles_by_owner::<ProfileDatabase>(
            to_uuid(user_id)?,
            &get_conn_from_ctx(ctx)
        )
        .await
        .expect("Unable to get Profiles under this User ID")
        .iter()
        .map(|f| ProfileType::from(f))
        .collect();

        Ok(profile)
    }
    #[graphql(name = "getProfilesById")]
    async fn get_profile_id(&self, ctx: &Context<'_>, profile_id: ID) -> FieldResult<ProfileType> { 
        let profile = Profiles::get_profile_by_id::<ProfileDatabase>(
            to_uuid(profile_id)?,
            &get_conn_from_ctx(ctx)
        )
        .await
        .map(|f| ProfileType::from(&f))
        .expect("Unable to get user profile by id");

        Ok(profile)
    }
    #[graphql(name = "getProfilesByUsername")]
    async fn get_profile_name(&self, ctx: &Context<'_>, username: String) -> FieldResult<ProfileType>  {
        let profile = Profiles::get_profile_by_name::<ProfileDatabase>(
            username,
            &get_conn_from_ctx(ctx)
        )
        .await
        .map(|f| ProfileType::from(&f))
        .expect("Unable to get profile by name");

        Ok(profile)
    }
}

#[derive(Default)]
pub struct ProfileMutation;

#[derive(InputObject, Clone, Debug)]
pub struct NewProfileInput { 
    pub user_id: ID, 
    pub username: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

#[Object]
impl ProfileMutation { 
    #[graphql(name = "createNewProfile")]
    async fn create_new_profile(&self, ctx: &Context<'_>, new_user: NewProfileInput) -> FieldResult<ProfileType> { 
        let profile = Profiles::create_new_profile::<ProfileDatabase>(
            NewProfile::from(&new_user),
            &get_conn_from_ctx(ctx)
        )
        .await
        .map(|f| ProfileType::from(&f))
        .expect("Unable to create profile");
        
        Ok(profile)
    }
    
    #[graphql(name = "deleteProfile")]
    async fn delete_profile(&self, ctx: &Context<'_>, user_id: ID, profile_id: ID) -> FieldResult<bool> { 
        let deleted_profile = Profiles::delete_profile_by_user::<ProfileDatabase>(
            to_uuid(user_id)?,
            to_uuid(profile_id)?,
            &get_conn_from_ctx(ctx)
        )
        .await
        .expect("Unable to delete profile by user");

        Ok(deleted_profile)
    }
    #[graphql(name = "updateUserProfile")]
    async fn update_profile(&self, ctx: &Context<'_>, user_id: ID, profile_id: ID, new_profile: NewProfileInput) -> FieldResult<Option<ProfileType>> { 
        let profile = Profiles::update_profile_user::<ProfileDatabase>(
            to_uuid(user_id)?,
            to_uuid(profile_id)?,
            NewProfile::from(&new_profile),
            &get_conn_from_ctx(ctx)
        )
        .await
        .expect("Unable to update")
        .map(|f| ProfileType::from(&f));

        Ok(profile)
    }

}