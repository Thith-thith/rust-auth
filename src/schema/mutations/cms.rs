use std::time::SystemTime;
use async_graphql::{Context, InputObject, Object, Result, SimpleObject, ID};
use mongodb::{bson::{doc, oid::ObjectId, Bson, DateTime as BsonDateTime}, Database};
use chrono::Utc;
use crate::models::models::{Post, User};

#[derive(SimpleObject)]
pub struct CmsResponse {
    pub success: bool,
    pub message: String,
}

#[derive(InputObject)]
struct PostInput {
    pub title: String,
    pub thumbnail: String,
    pub author_id: String,
    pub desc: String,
}

#[derive(InputObject)]
struct PostUpdateInput {
    pub title: Option<String>,
    pub thumbnail: Option<String>,
    pub author_id: Option<String>,
    pub desc: Option<String>,
}

#[derive(Default)]
pub struct CMSMutation;

#[Object]
impl CMSMutation {
    async fn create_post(&self, ctx: &Context<'_>, input: PostInput) -> Result<CmsResponse> {
        let db = ctx.data::<Database>()?;
        let user_collection = db.collection::<User>("users");
        let post_collection = db.collection::<Post>("posts");

        let author_oid = ObjectId::parse_str(&input.author_id)
            .map_err(|_| async_graphql::Error::new("Invalid author ID"))?;

        let author_exist = user_collection
            .find_one(doc! {"_id": &author_oid}, None)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?
            .is_some();

        if !author_exist {
            return Err(async_graphql::Error::new("Author not found"));
        }

        let created_at = Utc::now(); // Get chrono::DateTime<Utc>
        let system_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(created_at.timestamp() as u64); // Convert to SystemTime
        let bson_datetime = BsonDateTime::from_system_time(system_time); // Convert to BsonDateTime

        let post = Post {
            id: None,
            title: input.title,
            thumbnail: input.thumbnail,
            author: author_oid,
            desc: input.desc,
            created_at: Some(bson_datetime), // Use converted created_at
            updated_at: None,
        };

        post_collection
            .insert_one(post.clone(), None)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(CmsResponse {
            success: true,
            message: "Post created successfully".to_string(),
        })
    }

    async fn update_post(&self, ctx: &Context<'_>, id: ID, input: PostUpdateInput) -> Result<CmsResponse> {
        let db = ctx.data::<Database>()?;
        let post_collection = db.collection::<Post>("posts");

        // Convert the current time to BSONDateTime
        let now = Utc::now();
        let system_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(now.timestamp() as u64);
        let bson_datetime = BsonDateTime::from_system_time(system_time);

        let post_oid = ObjectId::parse_str(&id.as_str())
            .map_err(|_| async_graphql::Error::new("Invalid post ID"))?;

        let mut update_doc = doc! {};
        if let Some(title) = &input.title {
            update_doc.insert("title", title);
        }
        if let Some(desc) = &input.desc {
            update_doc.insert("desc", desc);
        }
        if let Some(thumbnail) = &input.thumbnail {
            update_doc.insert("thumbnail", thumbnail);
        }
        if let Some(author_id) = &input.author_id {
            let author_oid = ObjectId::parse_str(&author_id)
                .map_err(|_| async_graphql::Error::new("Invalid author ID"))?;
            update_doc.insert("author", author_oid);
        }

        // Always update `updated_at`
        update_doc.insert("updated_at", Bson::DateTime(bson_datetime));

        if update_doc.is_empty() {
            return Err(async_graphql::Error::new("No data to update"));
        }

        let update_res = post_collection
            .update_one(doc! {"_id": &post_oid}, doc! {"$set": update_doc}, None)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        if update_res.modified_count > 0 {
            Ok(CmsResponse {
                success: true,
                message: "Post updated successfully".to_string(),
            })
        } else {
            Err(async_graphql::Error::new("Failed to update post"))
        }
    }

    async fn remove_post(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let db = ctx.data::<Database>()?;
        let post_collection = db.collection::<Post>("posts");

        let post_oid = ObjectId::parse_str(&id.as_str())
            .map_err(|_| async_graphql::Error::new("Invalid post ID"))?;

        let delete_res = post_collection
            .delete_one(doc! {"_id": post_oid}, None)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(delete_res.deleted_count > 0)
    }
}
