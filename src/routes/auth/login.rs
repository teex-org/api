use rocket::serde::{Serialize,Deserialize};
use rocket::serde::json::Json;
use rocket::http::Status;
use rusqlite::{Result,Error};


use crate::routes::log::*;

use bcrypt::verify;

use rocket::State;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use sha2::{Sha256, Digest};
use rand::Rng;


#[derive(Serialize)]
pub struct Success {
    token : String
}

#[derive(Debug, Deserialize)]
pub struct Body {
    email    : String,
    password : String
}
#[post("/login", data = "<body>")]
pub fn route(body: Json<Body>, db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<Json<Success>, Status> {
    let conn = db_pool.get().unwrap();
    
    // CHECK AUTH
    let hashed_password: String = conn.query_row("SELECT password FROM user WHERE email = ?", &[&body.email], |row| row.get(0))
    .map_err(|err| 
        match err {
            Error::QueryReturnedNoRows => send_error(404, "Oops : no user with this email".to_string()),
            _                          => send_error(500, err.to_string()),
        }
    )?;
    log("Get correct Hashed possword get from DB".to_string());


    let is_correct_password = verify(&body.password, &hashed_password)
    .map_err(|err| send_error(401, err.to_string()))?;
    log("recived password has been hashed correctly".to_string());

    if ! is_correct_password{
        return Err(send_error(401, "Incorrect password".to_string()));
    } 

    // NEW TOKEN
    let clear_token = format!("{}-{}",&body.email,rand::thread_rng().gen_range(0..=10000000));
    let mut hasher  = Sha256::new();
    hasher.update(clear_token.as_bytes());
    let token = format!("{:x}",hasher.finalize());
    log("token made ".to_string() + &token);

    conn.execute("UPDATE connexion SET token = ? WHERE email = ?", &[&token, &body.email]).map_err(|err| send_error(500, err.to_string()))?;
    
    Ok(Json(Success{token: token.to_string()}))
}
