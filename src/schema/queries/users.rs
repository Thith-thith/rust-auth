use futures::stream::TryStreamExt;
use actix_web::{get, web, HttpResponse, Responder};
use async_graphql::{
    http::GraphQLPlaygroundConfig, Context, Object, Result, SimpleObject,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use mongodb::{bson::doc, Database};
use crate::{models::models::User, MySchema};

#[get("/graphiql")]
pub async fn public_graphql_playground() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(async_graphql::http::playground_source(
            GraphQLPlaygroundConfig::new("/graphql").title("GraphQL Playground"),
        ))
}

#[derive(SimpleObject)]
pub struct GQLUser {
    pub id: String,
    pub email: String,
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
}

impl From<User> for GQLUser {
    fn from(user: User) -> Self {
        GQLUser {
            id: user.id.map(|id| id.to_string()).unwrap_or_default(),
            email: user.email,
            full_name: user.full_name,
            phone_number: user.phone_number,
        }
    }
}

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<GQLUser>> {
        let db = ctx.data::<Database>()?;
        let collection = db.collection::<User>("users");

        let mut cursor = collection
            .find(doc! {}, None)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let mut users = Vec::new();
        while let Some(user) = cursor
            .try_next()
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?
        {
            users.push(GQLUser::from(user));
        }
        Ok(users)
    }

    async fn user(&self, ctx: &Context<'_>, email: String) -> Result<Option<GQLUser>> {
        let db = ctx.data::<Database>()?;
        let collection = db.collection::<User>("users");

        let user = collection
            .find_one(doc! { "email": email }, None)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(user.map(GQLUser::from))
    }
}

// pub type MySchema = Schema<Query, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

// pub fn create_schema(db: Database) -> MySchema {
//     Schema::build(Query::default(), async_graphql::EmptyMutation, async_graphql::EmptySubscription)
//         .data(db) // Register database inside schema
//         .finish()
// }


pub async fn graphql_handler(
    schema: web::Data<MySchema>,
    db: web::Data<Database>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();
    request = request.data(db.clone()); // Clone and inject database reference
    let response = schema.execute(request).await;
    GraphQLResponse::from(response)
}
