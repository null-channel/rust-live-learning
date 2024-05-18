use crate::{
    storage::sqlite::{get_all_todos, new_todo, update_todo, TodoState},
    types::Todo,
};
use axum::{extract::State, http::StatusCode, Json};

pub async fn get_todos(State(state): TodoState) -> (StatusCode, Json<Option<Vec<Todo>>>) {
    let conn = state.acquire().await.unwrap();
    let todos_result = get_all_todos(conn).await;
    match todos_result {
        Ok(todos) => return (StatusCode::OK, Json(Some(todos))),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn post_todo(
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

pub async fn root() -> (StatusCode, String) {
    (StatusCode::OK, String::from("You have done fantastic work"))
}
