use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use core::panic;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use sqlx::{
    pool::{Pool, PoolConnection},
    Sqlite,
};

type TodoState = State<sqlx::SqlitePool>;

#[derive(Parser)]
struct CliArgs {
    #[arg(short, long)]
    port: String,
    database_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Todo {
    id: Option<i64>,
    title: String,
    completed: bool,
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

async fn root() -> (StatusCode, String) {
    (StatusCode::OK, String::from("You have done fantastic work"))
}

async fn get_all_todos(mut sql_con: PoolConnection<Sqlite>) -> Result<Vec<Todo>, sqlx::Error> {
    let req = sqlx::query!(
        r#"
SELECT id, title, completed
FROM todos
ORDER BY id
    "#
    )
    .fetch_all(&mut *sql_con)
    .await?;

    let mut todos = Vec::new();
    for result in req {
        let new_todo = Todo {
            id: Some(result.id),
            title: result.title,
            completed: result.completed,
        };

        todos.push(new_todo);
    }
    Ok(todos)
}
async fn new_todo(
    todo: Todo,
    mut sql_con: PoolConnection<Sqlite>,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
INSERT INTO todos (title, completed)
VALUES ( ?, ? )
    "#,
        todo.title,
        todo.completed
    )
    .execute(&mut *sql_con)
    .await
}
async fn update_todo(
    todo: Todo,
    mut sql_con: PoolConnection<Sqlite>,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
UPDATE todos
SET title = ?, completed = ?
WHERE id = ?
    "#,
        todo.title,
        todo.completed,
        todo.id
    )
    .execute(&mut *sql_con)
    .await
}

async fn get_todos(State(state): TodoState) -> (StatusCode, Json<Option<Vec<Todo>>>) {
    let conn = state.acquire().await.unwrap();
    let todos_result = get_all_todos(conn).await;
    match todos_result {
        Ok(todos) => return (StatusCode::OK, Json(Some(todos))),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

async fn post_todo(
    State(state): TodoState,
    Json(payload): Json<Todo>,
) -> (StatusCode, Json<Option<Todo>>) {
    let conn = state.acquire().await.unwrap();
    let saved = match payload.id {
        Some(_id) => {
            let _ = update_todo(payload.clone(), conn).await;
            Some(payload)
        }
        None => {
            let todo = Todo {
                id: None,
                title: payload.title,
                completed: payload.completed,
            };
            let _ = new_todo(todo.clone(), conn).await;
            Some(todo)
        }
    };

    (StatusCode::OK, Json(saved))
}
