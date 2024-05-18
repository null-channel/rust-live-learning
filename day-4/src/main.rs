use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::vec;

type TodoState = State<Arc<Mutex<Vec<Todo>>>>;

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

    let app_state = Arc::new(Mutex::new(vec![
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
    ]));

    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(get_todos))
        .route("/todos", post(post_todo))
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> (StatusCode, String) {
    (StatusCode::OK, String::from("You have done fantastic work"))
}

async fn get_todos(State(state): TodoState) -> (StatusCode, Json<Vec<Todo>>) {
    let todos = state.lock().unwrap();
    (StatusCode::OK, Json(todos.to_vec()))
}

async fn post_todo(State(state): TodoState, Json(payload): Json<Todo>) -> (StatusCode) {
    let mut todos = state.lock().unwrap();
    todos.push(payload);
    StatusCode::OK
}
