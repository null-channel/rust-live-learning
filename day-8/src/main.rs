use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
use core::panic;
use sqlx::sqlite::SqlitePool;

pub mod routes;
pub mod storage;
pub mod types;

use routes::{get_todos, post_todo, root};

#[derive(Parser)]
struct CliArgs {
    #[arg(short, long)]
    port: String,
    database_url: String,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let pool_result = SqlitePool::connect(&args.database_url).await;

    let Ok(pool) = pool_result else {
        panic!("could not connect to the database");
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(get_todos))
        .route("/todos", post(post_todo))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
