use actix_web::{
    web, HttpResponse, Responder,
};
use mongodb::{bson::doc, Database};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::env;
use chrono::{Utc, Duration};

use crate::models::models::{AuthUser, Claims};


/// Register a new user
pub async fn register_user(db: web::Data<Database>, user: web::Json<AuthUser>) -> impl Responder {
    let collection = db.collection::<AuthUser>("users");

    // Check if the user already exists
    if collection.find_one(doc! {"email": &user.email}, None).await.unwrap().is_some() {
        return HttpResponse::Conflict().body("User already exists!");
    }

    // Hash the password
    let hashed_password = match hash(&user.password, 10) {
        Ok(pwd) => pwd,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };

    // Create the new user
    let new_user = AuthUser {
        email: user.email.clone(),
        password: hashed_password,
        full_name: user.full_name.clone(),
        phone_number: user.phone_number.clone(),
    };

    // Insert the user into the database
    match collection.insert_one(new_user, None).await {
        Ok(_) => HttpResponse::Ok().body("User registered successfully!"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to register user"),
    }
}

/// Log in a user and issue a JWT token
pub async fn login_user(db: web::Data<Database>, user: web::Json<AuthUser>) -> impl Responder {
    let collection = db.collection::<AuthUser>("users");

    // Find the user by email
    let existing_user = match collection.find_one(doc! {"email": &user.email}, None).await {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    // Verify the password
    if let Err(_) = verify(&user.password, &existing_user.password) {
        return HttpResponse::Unauthorized().body("Invalid credentials");
    }

    // Generate a JWT token
    let expiration = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: user.email.clone(),
        exp: expiration,
    };

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = match encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    ) {
        Ok(token) => token,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to generate token"),
    };

    HttpResponse::Ok().json(serde_json::json!({ "token": token }))
}