use async_graphql::{Context, Object, Result, SimpleObject};
use mongodb::{bson::doc, Database};
use crate::models::models::User;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(SimpleObject)]
struct MutationResponse {
    success: bool,
    message: String,
}

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Mutation {
    async fn update_user(
        &self,
        ctx: &Context<'_>,
        email: String,
        full_name: Option<String>,
        phone_number: Option<String>,
    ) -> Result<MutationResponse> {
        let db = ctx.data::<Database>()?;
        let collection = db.collection::<User>("users");
    
        // Find existing user
        let user = collection.find_one(doc! { "email": &email }, None).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?
            .ok_or_else(|| async_graphql::Error::new("User not found"))?;
    
        // Check if provided values are the same as existing ones
        if user.full_name == full_name && user.phone_number == phone_number {
            return Ok(MutationResponse {
                success: false,
                message: "No changes detected. Please update with new information.".to_string(),
            });
        }
    
        let filter = doc! { "email": &email };
        let update = doc! {
            "$set": {
                "full_name": full_name.clone(),
                "phone_number": phone_number.clone()
            }
        };
    
        let result = collection.update_one(filter, update, None).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
    
        if result.modified_count == 0 {
            return Ok(MutationResponse {
                success: false,
                message: "Failed to update user. Please try again.".to_string(),
            });
        }
    
        Ok(MutationResponse {
            success: true,
            message: "User updated successfully".to_string(),
        })
    }

    async fn reset_password(
        &self,
        ctx: &Context<'_>,
        email: String,
        old_password: String,
        new_password: String,
    ) -> Result<MutationResponse> {
        let db = ctx.data::<Database>()?;
        let collection = db.collection::<User>("users");

        let user = collection.find_one(doc! { "email": &email }, None).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?
            .ok_or_else(|| async_graphql::Error::new("User not found!"))?;

        if !verify(&old_password, &user.password).map_err(|e| async_graphql::Error::new(e.to_string()))? {
            return Ok(MutationResponse {
                success: false,
                message: "Incorrect old password!".to_string(),
            });
        }

        let hashed_password = hash(new_password, DEFAULT_COST)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        collection.update_one(
            doc! { "email": &email },
            doc! { "$set": { "password": hashed_password } },
            None
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(MutationResponse {
            success: true,
            message: "Password reset successfully!".to_string(),
        })
    }

    async fn delete_account(&self, ctx: &Context<'_>, email: String) -> Result<MutationResponse> {
        let db = ctx.data::<Database>()?;
        let collection = db.collection::<User>("users");

        let delete_result = collection.delete_one(doc! { "email": &email }, None).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        if delete_result.deleted_count == 0 {
            return Ok(MutationResponse {
                success: false,
                message: "User not found!".to_string(),
            });
        }

        Ok(MutationResponse {
            success: true,
            message: "Account deleted successfully!".to_string(),
        })
    }
}
