use rocket::*;
use rocket_contrib::json::Json;
use rusqlite::*;
use rocket::http::Status;
use sha256::digest;
use super::db_connect;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub id: Option<i64>
}

#[post("/signup", format = "json", data = "<data>")]
pub fn user_sign_up(data: Json<User>) -> rocket::response::status::Custom<String> {
    let mut user: User = data.into_inner();
    let original_password = user.password.clone();
    if user.username.len() <= 4 || user.password.len() <= 4 {return  rocket::response::status::Custom(Status::BadRequest, "Username or password too shor".to_string())}
    user.password = digest(user.password);
    let conn = match db_connect() {
        Ok(conn) => conn,
        Err(err) => {return rocket::response::status::Custom(Status::BadRequest, err.to_string() + " 28")}
    };
    let result = conn.execute(
        "INSERT INTO users (username, password) VALUES (?, ?)",
        [user.username.clone(), user.password.clone()]
    );

    match result {
        Ok(_) => {
            let last_id = conn.last_insert_rowid();
            user.id = Some(last_id);
            user.password = original_password;
            return rocket::response::status::Custom(Status::Accepted, serde_json::to_string(&user).unwrap())
        }
        Err(err) => {
            return rocket::response::status::Custom(Status::BadRequest, err.to_string() + " 42")
        }
    }
}

#[post("/signin", format = "json", data = "<data>")]
pub fn user_sign_in(data: Json<User>) -> rocket::response::status::Custom<String> {
    let mut user: User = data.into_inner();
    let original_password = user.password.clone();
    user.password = digest(user.password);

    let conn = match db_connect() {
        Ok(conn) => conn,
        Err(err) => {return rocket::response::status::Custom(Status::BadRequest, err.to_string() + " 28")}
    };
    let mut query: Statement = match conn.prepare("SELECT * FROM users WHERE username = ? AND password = ?") {
        Ok(query) => query,
        Err(err) => return rocket::response::status::Custom(Status::BadRequest, err.to_string() + " 55")
    };
    let mut rows: Rows = match query.query(params![user.username.clone(), user.password.clone()]) {
        Ok(rows) => rows,
        Err(err) => return rocket::response::status::Custom(Status::BadRequest, err.to_string() + " 59")
    };
    let mut users: Vec<User> = vec![];
    while let Some(row) = rows.next().unwrap() {
        let username: String = row.get("username").unwrap();
        let password: String = row.get("password").unwrap();
        let id: i64 = row.get("id").unwrap();
        users.push(User { username, password, id: Some(id) })
    };
    match users.len() {
        1 => user.id = users[0].id,
        0 => return rocket::response::status::Custom(Status::BadRequest, format!("Expected 1 row, found {} (User not found)", users.len())),
        _ => return rocket::response::status::Custom(Status::InternalServerError, format!("Expected 1 row, found {}", users.len()))
    };

    user.password = original_password;
    return rocket::response::status::Custom(Status::Accepted, serde_json::to_string(&user).unwrap())
}

pub fn authenticate_user(user: User) -> Result<User, String> {
    let mut new_user = user;
    new_user.password = digest(new_user.password);
    let conn = match db_connect() {
        Ok(conn) => conn,
        Err(err) => {return Err(err.to_string())}
    };

    let mut query: Statement = match conn.prepare("SELECT * FROM users WHERE username = ? AND password = ?") {
        Ok(query) => query,
        Err(err) => return Err(err.to_string())
    };
    let mut rows: Rows = match query.query(params![new_user.username.clone(), new_user.password.clone()]) {
        Ok(rows) => rows,
        Err(err) => return Err(err.to_string())
    };
    let mut users: Vec<User> = vec![];
    while let Some(row) = rows.next().unwrap() {
        let username: String = row.get("username").unwrap();
        let password: String = row.get("password").unwrap();
        let id: i64 = row.get("id").unwrap();
        users.push(User { username, password, id: Some(id) })
    };
    match users.len() {
        1 => new_user.id = users[0].id,
        _ => return Err("user not found".to_string()) 
    };
    return Ok(new_user)
}