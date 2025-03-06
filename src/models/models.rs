use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
}

// For login request
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// For registration request
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
}

// For JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (email)
    pub exp: usize,  // Expiration timestamp
}