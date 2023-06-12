use super::users::User;
use super::users;
use super::db_connect;
use rocket::*;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use rusqlite::*;
use rocket::http::Status; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    user_id: i64,
    task_id: Option<i64>,
    name: String,
    description: String,
    completed: bool,
    due: String
}               

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskRequest {
    user: User,
    task: Task
}

#[post("/create_task", format = "json", data = "<data>")]
pub fn create_task(data: Json<TaskRequest>) -> rocket::response::status::Custom<String> {
    let request: TaskRequest = data.into_inner();
    let mut task: Task = request.task;  
    let user_id: Option<i64> ;
    match users::authenticate_user(request.user) {
        Ok(us) => {user_id = us.id;},
        Err(err) => {return rocket::response::status::Custom(Status::BadRequest, err.to_string())}
    };
   

    let conn = match db_connect() {
        Ok(conn) => conn,
        Err(err) => {return rocket::response::status::Custom(Status::BadRequest, err.to_string() + " create_task 1")}
    };
    let query = "INSERT INTO tasks (user_id, name, description, completed, due) VALUES (?1, ?2, ?3, ?4, ?5)";
    match conn.execute(query,
        params![user_id, task.name.clone(), task.description.clone(), false, task.due.clone()]) {
            Ok(_) => {
                let id = conn.last_insert_rowid();
                task.task_id = Some(id)
            },
            Err(err) => {return rocket::response::status::Custom(Status::BadRequest, err.to_string() + " create_task 2")}
    }
    println!("{:?}", user_id);
    return rocket::response::status::Custom(Status::Accepted, serde_json::to_string(&task).unwrap());
}

#[post("/get_tasks", format = "json", data = "<data>")]
pub fn retrieve_tasks(data: Json<User>) -> rocket::response::status::Custom<String> {
    let mut user = data.into_inner();
    match users::authenticate_user(user) {
        Ok(user_data) => {user = user_data},
        Err(err) => {println!("user not authed"); ;return rocket::response::status::Custom(Status::BadRequest, err.to_string())}
    };
    let conn = match db_connect() {
        Ok(conn) => conn,
        Err(err) => return rocket::response::status::Custom(Status::InternalServerError, err.to_string())
    };
    let mut query = match conn.prepare("SELECT * FROM tasks WHERE user_id = ?") {
        Ok(query) => query,
        Err(err) => return rocket::response::status::Custom(Status::InternalServerError, err.to_string())
    };
    let mut rows = match query.query(params![user.id]) {
        Ok(rows) => rows,
        Err(err) => return rocket::response::status::Custom(Status::InternalServerError, err.to_string())
    };
    let mut tasks: Vec<Task> = vec![];

    while let Some(row) = rows.next().unwrap() {
        let user_id = row.get("user_id").unwrap();
        let task_id = row.get("task_id").unwrap();
        let name = row.get("name").unwrap();
        let description = row.get("description").unwrap();
        let due = row.get("due").unwrap();
        let completed = row.get("completed").unwrap();
        let task = Task{user_id, task_id, name, description, due, completed};
        tasks.push(task);
    };
    return rocket::response::status::Custom(Status::Accepted, serde_json::to_string(&tasks).unwrap())

}

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskDelete {
    user: User,
    task_id: i64
}

#[delete("/delete_task", format = "json", data = "<data>")]
pub fn delete_task(data: Json<TaskDelete>) -> rocket::response::status::Custom<String> {
    let user = &data.user;
    let user_id: Option<i64>;
    let user_clone = user.clone();
    match users::authenticate_user(user_clone) {
        Ok(us) => {user_id = us.id},
        Err(err) => {return rocket::response::status::Custom(Status::BadRequest, err.to_string())}
    };
    let conn = match db_connect() {
        Ok(conn) => conn,
        Err(err) => return rocket::response::status::Custom(Status::InternalServerError, err.to_string())
    };
    let mut query = match conn.prepare("DELETE FROM tasks WHERE task_id = ?") {
        Ok(query) => query,
        Err(err) => return rocket::response::status::Custom(Status::InternalServerError, err.to_string())
    };
    let _rows =  match query.execute(params![data.task_id]) {
        Ok(rows) => {rows},
        Err(err) => return rocket::response::status::Custom(Status::InternalServerError, err.to_string())
    };
    return retrieve_tasks(rocket_contrib::json::Json(User{username: user.username.clone(), password: user.password.clone(), id: user_id}));
}

#[post("/edit_task", format = "json", data = "<data>")]
pub fn edit_task(data: Json<TaskRequest>) -> rocket::response::status::Custom<String> {
    let user: &User = &data.user;
    let task: &Task = &data.task;
    let user_id: Option<i64>;
    let user_clone = user.clone();
    match users::authenticate_user(user_clone) {
        Ok(us) => {user_id = us.id},
        Err(err) => return rocket::response::status::Custom(Status::BadRequest, err.to_string())
    };
    let conn = match db_connect() {
        Ok(conn) => conn,
        Err(err) => return rocket::response::status::Custom(Status::InternalServerError, err.to_string())
    };
    let mut query = match conn.prepare("
        UPDATE tasks
        SET description = ?,
            name = ?,
            due = ?,
            completed = ?
        WHERE task_id = ? AND user_id = ?
    ") {Ok(query) => query, Err(err) => return rocket::response::status::Custom(Status::InternalServerError, err.to_string())};
    let _rows = match query.execute(params![
        task.description.clone(),
        task.name.clone(),
        task.due.clone(),
        task.completed.clone(),
        task.task_id.clone(),
        user_id.clone()
    ]) {Ok(rows) => rows, Err(err) => return rocket::response::status::Custom(Status::InternalServerError, err.to_string())};
    return retrieve_tasks(rocket_contrib::json::Json(User{username: user.username.clone(), password: user.password.clone(), id: user_id}));
}