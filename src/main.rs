use actix_web::{http::header, web, App, HttpServer};
use async_graphql::Schema;
use dotenv::dotenv;
use mongodb::Database;
use actix_cors::Cors;
use schema::{MutationRoot, QueryRoot};
use crate::schema::{graphql_handler, public_graphql_playground};


mod models;
mod auth;
mod db;
mod schema;


pub type MySchema = Schema<QueryRoot, MutationRoot, async_graphql::EmptySubscription>;

fn create_schema(db: Database) -> MySchema {
    Schema::build(QueryRoot::default(), MutationRoot::default(), async_graphql::EmptySubscription)
        .data(db)
        .finish()
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db = db::get_database().await;
    println!("Connected to database: {}", db.name());
    let schema = create_schema(db.clone());

    HttpServer::new(move || {

         let cors = Cors::permissive()
            .allowed_methods(vec!["GET", "POST", "DELETE", "UPDATE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                header::ACCEPT,
            ])
            .allow_any_header()
            .max_age(3600)
            .supports_credentials();


        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(schema.clone()))
            .route("/register", web::post().to(auth::register_user))
            .route("/login", web::post().to(auth::login_user))
            .route("/graphql", web::post().to(graphql_handler))
            .service(public_graphql_playground)
            .wrap(cors)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}