use async_graphql::InputObject;
use uuid::Uuid;

#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct UpdateUserInput {
    /// The user's external ID
    pub external_id: Option<Uuid>,

    /// The user's username
    pub username: Option<String>,

    /// The user's email
    pub email: Option<String>,

    /// Whether the user is active or disabled
    pub is_active: Option<bool>,
}
