use serde::{Deserialize, Serialize};

use crate::models::_entities::users;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub user: LoginUser,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &users::Model, token: &String) -> Self {
        Self {
            user: LoginUser {
                username: user.name.clone(),
                email: user.email.clone(),
                token: token.clone(),
                image: "/assets/profile-default.png".to_string(),
                bio: "No bio yet".to_string(),
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUser {
    pub username: String,
    pub email: String,
    pub token: String,
    pub image: String,
    pub bio: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserProfile {
    username: String,
    bio: String,
    image: String,
}

impl UserProfile {
    pub fn new(user: users::Model) -> Self {
        Self {
            username: user.name,
            bio: user.bio.unwrap_or("No bio yet".to_string()),
            image: user.image.unwrap_or("/assets/profile-default.png".to_string()),
        }
    }
}