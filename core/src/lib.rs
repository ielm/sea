use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{self, NotSet, Set, Unchanged},
    EntityTrait, InsertResult,
};

use crate::domains::role_grants::model::RoleGrant;

pub mod database;
pub mod domains;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let db = database::establish_connection().await?;

    // Create set of role grants for user
    let role_grants = vec![
        domains::role_grants::model::ActiveModel {
            role_key: ActiveValue::Set(1),
            user_id: ActiveValue::Set(2),
            resource_table: ActiveValue::Set("test1".to_string()),
            resource_id: ActiveValue::Set("test1".to_string()),
            ..Default::default()
        },
        domains::role_grants::model::ActiveModel {
            role_key: ActiveValue::Set(2),
            user_id: ActiveValue::Set(2),
            resource_table: ActiveValue::Set("test1".to_string()),
            resource_id: ActiveValue::Set("test1".to_string()),
            ..Default::default()
        },
    ];

    let mut grants: Vec<RoleGrant> = Vec::new();
    // Insert role grants
    // let role_grants = RoleGrant::insert_many(role_grants).exec(&db).await?;
    for role_grant in role_grants {
        let role_grant = role_grant.insert(&db).await?;
        grants.push(role_grant.to_owned());
        // println!("role_grant: {:?}", role_grant);
    }

    // Create a user with a role
    let user = domains::users::model::ActiveModel {
        external_id: ActiveValue::Set(uuid::Uuid::new_v4()),
        username: ActiveValue::Set("test".to_string()),
        email: ActiveValue::Set("email@email.com".to_string()),
        is_active: ActiveValue::Set(true),
        ..Default::default()
    };

    let user = user.insert(&db).await?;

    // println!("roles: {:?}", roles);
    println!("role_grants: {:?}", grants);
    // println!("user: {:?}", user);
    println!("\n\n");

    // Get all users
    let users = domains::users::model::Entity::find().all(&db).await?;

    println!("users: {:?}", users);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
