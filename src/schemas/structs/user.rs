use juniper::{GraphQLInputObject, GraphQLObject};

use crate::utils::date::date_time_now;

#[derive(GraphQLObject)]
#[graphql(description = "user info")]
pub struct User {
    pub avatar_url: String,
    pub created_at: String,
    pub description: Option<String>,
    pub email: String,
    pub is_active: bool,
    pub password_hash: String,
    pub phone_no: Option<String>,
    pub profile_url: String,
    pub status: String,
    pub user_id: String,
    pub username: String,
}

impl User {
    /// Creates a new [`User`].
    pub fn new(
        username: String,
        email: String,
        password_hash: String,
    ) -> Self {
        User {
            avatar_url: "".to_string(),
            created_at: date_time_now(),
            description: Some("".to_string()),
            email,
            is_active: false,
            password_hash,
            phone_no: Some("".to_string()),
            profile_url: "".to_string(),
            status: "".to_string(),
            user_id: "".to_string(),
            username,
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User input")]
pub struct UserQueryInput {
    pub username: String,
    pub email: String,
    pub password: String,
    pub description: Option<String>,
    pub phone_no: String,
}
