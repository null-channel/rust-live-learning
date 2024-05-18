use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Json;
use axum::Router;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::vec;

#[derive(Parser)]
struct CliArgs {
    #[arg(short, long)]
    port: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let app_state = Arc::new(vec![
        Todo {
            id: 1,
            title: "Hello World".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "Hello My Friend".to_string(),
            completed: true,
        },
    ]);

    let app = Router::new().route("/", get(root)).with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(todos): State<Arc<Vec<Todo>>>) -> (StatusCode, Json<Vec<Todo>>) {
    (StatusCode::OK, Json(todos.as_ref().to_vec()))
}
