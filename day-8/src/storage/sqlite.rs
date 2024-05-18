use axum::extract::State;
use sqlx::{pool::PoolConnection, Sqlite};

use crate::types::Todo;

pub type TodoState = State<sqlx::SqlitePool>;

pub async fn get_all_todos(mut sql_con: PoolConnection<Sqlite>) -> Result<Vec<Todo>, sqlx::Error> {
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

pub async fn new_todo(
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

pub async fn update_todo(
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
