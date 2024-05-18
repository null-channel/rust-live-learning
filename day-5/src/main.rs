use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type TodoState = State<Arc<Mutex<HashMap<u64, Todo>>>>;

#[derive(Parser)]
struct CliArgs {
    #[arg(short, long)]
    port: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Todo {
    id: Option<u64>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let app_state = Arc::new(Mutex::new(HashMap::from([
        (
            1,
            Todo {
                id: Some(1),
                title: "Hello World".to_string(),
                completed: false,
            },
        ),
        (
            2,
            Todo {
                id: Some(2),
                title: "Hello My Friend".to_string(),
                completed: true,
            },
        ),
    ])));

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
    let todos = state.lock().unwrap().values().cloned().collect();
    (StatusCode::OK, Json(todos))
}

async fn post_todo(
    State(state): TodoState,
    Json(payload): Json<Todo>,
) -> (StatusCode, Json<Option<Todo>>) {
    let mut todos = state.lock().unwrap();

    let saved = match payload.id {
        Some(id) => {
            todos.insert(id, payload.clone());
            Some(payload)
        }
        None => {
            let Ok(id) = u64::try_from(todos.len() + 1) else {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
            };
            let new_todo = Todo {
                id: Some(id),
                title: payload.title,
                completed: payload.completed,
            };
            todos.insert(id, new_todo.clone());
            Some(new_todo)
        }
    };

    (StatusCode::OK, Json(saved))
}
