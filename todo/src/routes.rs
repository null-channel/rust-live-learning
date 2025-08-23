use crate::{
    storage::sqlite::{get_all_todos, get_todo, new_todo, update_todo, AppState},
    types::Todo,
};
use axum::{extract::State, http::StatusCode, Json};

pub async fn get_todos(State(state): AppState) -> (StatusCode, Json<Option<Vec<Todo>>>) {
    let conn = state.todo_db.acquire().await.unwrap();
    let todos_result = get_all_todos(conn).await;
    match todos_result {
        Ok(todos) => (StatusCode::OK, Json(Some(todos))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

pub async fn delete_todo(State(state): AppState, Json(payload): Json<i64>) -> StatusCode {
    println!("deleting {payload}");
    let result = crate::storage::sqlite::delete_todo(payload, &state.todo_db).await;
    match result {
        Ok(query) => println!("{:?}", query),
        Err(e) => println!("{}", e),
    }
    StatusCode::OK
}

type WeatherClientState =
    State<crate::weather::weather_client::WeatherClient<tonic::transport::Channel>>;

pub async fn post_todo(
    State(app_state): AppState,
    Json(payload): Json<Todo>,
) -> (StatusCode, Json<Option<Todo>>) {
    let saved = match payload.id {
        Some(_id) => {
            //TODO: Check query result to see if the update was successful.
            let old_todo = get_todo(payload.id.unwrap(), &app_state.todo_db).await;
            match old_todo {
                Ok(old) => {
                    if old.due_date == None && payload.due_date.is_some() {
                        let request = crate::weather::GetWeatherRequest {
                            longitude: 39.983334,
                            latitude: -82.983330,
                        };
                        let response = app_state.weather_client.clone().get_weather(request).await;
                        match response {
                            Ok(res) => {
                                // Update the payload with the weather at completion
                            }
                            Err(e) => {
                                println!("Error fetching weather: {}", e);
                                return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
                            }
                        }
                    }
                }
                // TODO: return error (todo not found)
                Err(e) => return (StatusCode::BAD_REQUEST, Json(None)),
            }

            let _ = update_todo(&payload, &app_state.todo_db).await;
            Some(payload)
        }
        None => {
            let todo = Todo {
                id: None,
                title: payload.title,
                due_date: payload.due_date,
                completion_date: None,
                weather_at_completion: None,
            };
            let _ = new_todo(todo.clone(), &app_state.todo_db).await;
            Some(todo)
        }
    };
    (StatusCode::OK, Json(saved))
}

pub async fn root() -> (StatusCode, String) {
    (StatusCode::OK, String::from("You have done fantastic work"))
}
