#![feature(proc_macro_hygiene, decl_macro)]
use rocket::*;
use rocket_cors::{AllowedOrigins, CorsOptions};
#[allow(unused)]
use rocket_contrib::json::Json;
#[allow(unused)]
use serde::{Deserialize, Serialize};
use rusqlite::*;
#[allow(unused)]
use rocket::*;
#[allow(unused)]
use rocket::http::Status;
mod users;
mod tasks;
pub fn db_connect() -> Result<Connection> {
    let conn = Connection::open("./data.db")?;
    Ok(conn)
}





fn main() {
    let allowed_origins = AllowedOrigins::all();

    rocket::ignite()
        .attach(CorsOptions::default().allowed_origins(allowed_origins).to_cors().unwrap())
        .mount("/", routes![
            users::user_sign_up,
            users::user_sign_in,
            tasks::create_task,
            tasks::retrieve_tasks,
            tasks::edit_task,
            tasks::delete_task
        ])
    .launch();
}
