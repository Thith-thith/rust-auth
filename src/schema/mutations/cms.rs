use async_graphql::{Context, InputObject, Object, Result, SimpleObject};
use mongodb::{bson::{doc, oid::ObjectId}, Database};
use chrono::Utc;
use crate::models::models::{Post, User};

#[derive(SimpleObject)]
pub struct CmsResponse {
    pub success: bool,
    pub message: String,
}

#[derive(InputObject)]
struct PostInput{
    pub title: String,
    pub thumbnail: String,
    pub author_id: String,
    pub desc: String,
}



#[derive(Default)]
pub struct CMSMutation;


#[Object]
impl CMSMutation{
    async fn create_post(&self, ctx:&Context<'_>, input: PostInput) -> Result<CmsResponse>{
        let db = ctx.data::<Database>()?;
        let user_collection = db.collection::<User>("users");
        let post_collection = db.collection::<Post>("posts");

        let author_oid = ObjectId::parse_str(&input.author_id)
        .map_err(|_| async_graphql::Error::new("Invalid author ID"))?;

        let author_exist = user_collection.find_one(doc! {"_id": &author_oid}, None)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?
        .is_some();

        if !author_exist{
            return  Err(async_graphql::Error::new("Author not found"));
        }


        let post = Post{
            id: None,
            title: input.title,
            thumbnail: input.thumbnail,
            author: author_oid,
            desc: input.desc,
            created_at: Utc::now(),
            updated_at: None,
        };

        post_collection.insert_one(post.clone(), None).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

     
        Ok(CmsResponse {
            success: true,
            message: "Post created successfully".to_string(),
        })
    }
}