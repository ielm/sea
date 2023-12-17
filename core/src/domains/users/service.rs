use anyhow::{Ok, Result};
use async_trait::async_trait;
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait, QueryFilter};

use std::sync::Arc;

use super::{
    model::{self as user_model, User, UserOption},
    mutations::UpdateUserInput,
};

use crate::domains::role_grants::model::{self as role_grant_model};

#[async_trait]
pub trait UserServiceTrait: Sync + Send {
    /// Get an individual user by id
    async fn get(&self, id: &i32) -> Result<Option<User>>;

    /// Get a list of user results matching the given ids
    async fn get_by_ids(&self, ids: &[i32]) -> Result<Vec<User>>;

    /// Get an individual user by username
    async fn get_by_username(&self, username: &str, with_roles: &bool) -> Result<Option<User>>;

    /// Create a user with the given username
    async fn create(&self, username: &str) -> Result<User>;

    /// Get the user with a given username, creating one if none are found
    async fn get_or_create(&self, username: &str) -> Result<User>;

    /// Update an existing User
    async fn update(&self, id: &i32, input: &UpdateUserInput, with_roles: &bool) -> Result<User>;

    /// Delete an existing User
    async fn delete(&self, id: &i32) -> Result<()>;
}

pub struct UserService {
    /// The SeaORM database connection
    db: Arc<DatabaseConnection>,
}

impl UserService {
    /// Create a new UserService instance
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn get(&self, id: &i32) -> Result<Option<User>> {
        let user = user_model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?;
        Ok(user)
    }

    async fn get_by_ids(&self, ids: &[i32]) -> Result<Vec<User>> {
        let mut condition = Condition::any();

        for id in ids {
            condition = condition.add(user_model::Column::Id.eq(*id));
        }

        let users = user_model::Entity::find()
            .filter(condition)
            .all(&*self.db)
            .await?;

        Ok(users)
    }

    async fn get_by_username(&self, username: &str, with_roles: &bool) -> Result<Option<User>> {
        let query =
            user_model::Entity::find().filter(user_model::Column::Username.eq(username.to_owned()));

        let user: UserOption = if *with_roles {
            query
                .find_with_related(role_grant_model::Entity)
                .all(&*self.db)
                .await?
                .first()
                .map(|t| t.to_owned())
                .into()
        } else {
            query.one(&*self.db).await?.into()
        };

        Ok(user.into())
    }

    async fn create(&self, username: &str) -> Result<User> {
        let user = user_model::ActiveModel {
            username: Set(username.to_owned()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        Ok(user)
    }

    async fn get_or_create(&self, username: &str) -> Result<User> {
        match self.get_by_username(username, &false).await? {
            Some(user) => Ok(user),
            None => self.create(username).await,
        }
    }

    async fn update(&self, id: &i32, input: &UpdateUserInput, with_roles: &bool) -> Result<User> {
        let query = user_model::Entity::find_by_id(id.to_owned());

        let (user, roles) = if *with_roles {
            query
                .find_with_related(role_grant_model::Entity)
                .all(&*self.db)
                .await?
                .first()
                .map(|t| t.to_owned())
        } else {
            query.one(&*self.db).await?.map(|u| (u, vec![]))
        }
        .ok_or_else(|| anyhow::anyhow!("Unable to find user with id {}", id))?;

        let mut user: user_model::ActiveModel = user.into();

        if let Some(username) = &input.username {
            user.username = Set(username.to_owned());
        }

        if let Some(is_active) = &input.is_active {
            user.is_active = Set(is_active.to_owned());
        }

        let mut updated = user.update(&*self.db).await?;

        updated.roles = roles;

        Ok(updated)
    }

    async fn delete(&self, id: &i32) -> Result<()> {
        let user = user_model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Unable to find user with id {}", id))?;

        let _result = user.delete(&*self.db).await?;

        Ok(())
    }
}
