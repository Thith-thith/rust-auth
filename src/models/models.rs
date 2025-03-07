use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};



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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub thumbnail: String,
    #[serde(rename = "author_id")]
    pub author: ObjectId,
    pub desc: String,
    #[serde(with = "chrono::serde::ts_seconds")] 
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}

