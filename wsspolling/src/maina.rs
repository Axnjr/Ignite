mod Igniter;
mod Threads;
mod learn_rust;

use axum::extract::Json;
use axum::routing::post;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::clone;
use std::env;
use Igniter::IgniteReq;
use Igniter::igniter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    dotenv().ok();

    let url = env::var("DB_URL").expect("DB Connection URL not found !!");
    let db_client = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?
    ;

    let app = axum::Router::new()
        .route("/ignite", post(|body: Json<IgniteReq>| async { 
            igniter(body, db_client).await
        })) // .layer( ServiceBuilder::new().layer(CorsLayer::permissive()).layer(layer) )
    ;

    println!("Starting server");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}