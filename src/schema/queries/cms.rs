use async_graphql::{Context, Object, Result, SimpleObject};
use mongodb::{bson::doc, Database};
use futures::stream::TryStreamExt;

use crate::models::models::{Post, User};

#[derive(SimpleObject)]
pub struct Author {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub phone_number: Option<String>,
}

#[derive(SimpleObject)]
pub struct GQLPost {
    pub id: String,
    pub title: String,
    pub desc: String,
    pub thumbnail: String,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub author: Author,  // Author as an object
}

#[derive(Default)]
pub struct CmsQuery;

#[Object]
impl CmsQuery {
    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<GQLPost>> {
        let db = ctx.data::<Database>()?;
        let post_collection = db.collection::<Post>("posts");
        let user_collection = db.collection::<User>("users");

        let mut cursor = post_collection.find(doc! {}, None).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let mut posts = Vec::new();
        while let Some(post) = cursor.try_next().await
            .map_err(|e| async_graphql::Error::new(e.to_string()))? 
        {
            let author = user_collection
            .find_one(doc! { "_id": &post.author }, None)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?
            .ok_or_else(|| async_graphql::Error::new("Author not found"))?;


               // If updated_at is None, just return None.
            let updated_at = post.updated_at.map(|dt| dt.to_rfc3339());


           posts.push(GQLPost {
            id: post.id.map(|oid| oid.to_hex()).unwrap_or_default(),  // Convert ObjectId to String
            title: post.title,
            thumbnail: post.thumbnail,
            author: Author {
                id: author.id.map(|oid| oid.to_hex()).unwrap_or_default(),
                email: author.email,
                full_name: author.full_name.unwrap_or_default(),
                phone_number: author.phone_number,
            },
            desc: post.desc,
            created_at: post.created_at.to_string(),
            updated_at,  
        });

        }

        Ok(posts)
    }
}
