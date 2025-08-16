use axum::extract::State;
use chrono::{DateTime, NaiveDateTime, TimeZone};
use sqlx::{pool::PoolConnection, Sqlite};

use crate::types::Todo;

pub type TodoState = State<sqlx::SqlitePool>;

pub async fn get_all_todos(mut sql_con: PoolConnection<Sqlite>) -> Result<Vec<Todo>, sqlx::Error> {
    let req = sqlx::query!(
        r#"
SELECT id, title, due_date, completion_date, weather_at_completion
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
            due_date: result.due_date,
            completion_date: result.completion_date,
            weather_at_completion: result.weather_at_completion,
        };

        todos.push(new_todo);
    }
    Ok(todos)
}

pub async fn get_todo(id: i64, mut sql_con: PoolConnection<Sqlite>) -> Result<Todo, sqlx::Error> {
    let req = sqlx::query!(
        r#"
SELECT id, title, due_date, completion_date, weather_at_completion
FROM todos
WHERE id = ?
    "#,
        id,
    )
    .fetch_one(&mut *sql_con)
    .await?;

    let todo = Todo {
        id: Some(req.id),
        title: req.title,
        due_date: req.due_date,
        completion_date: req.completion_date,
        weather_at_completion: req.weather_at_completion,
    };
    Ok(todo)
}

pub async fn delete_todo(
    id: i64,
    mut sql_con: PoolConnection<Sqlite>,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
DELETE FROM todos WHERE id=?
    "#,
        id,
    )
    .execute(&mut *sql_con)
    .await
}

pub async fn new_todo(
    todo: Todo,
    mut sql_con: PoolConnection<Sqlite>,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
INSERT INTO todos (title, completion_date, due_date)
VALUES ( ?, ?, ? )
    "#,
        todo.title,
        "",
        todo.due_date,
    )
    .execute(&mut *sql_con)
    .await
}

pub async fn update_todo(
    todo: &Todo,
    mut sql_con: PoolConnection<Sqlite>,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
UPDATE todos
SET title = ?, due_date = ?, completion_date = ?
WHERE id = ?
    "#,
        todo.title,
        todo.due_date,
        todo.completion_date,
        todo.id
    )
    .execute(&mut *sql_con)
    .await
}
