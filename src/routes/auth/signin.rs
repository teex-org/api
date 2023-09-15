

use rocket::serde::{Serialize,Deserialize};
use rocket::serde::json::Json;
use rocket::http::Status;
use rusqlite::Result;

use crate::routes::log::*;

use bcrypt::hash;
use sha2::{Sha256, Digest};
use rand::Rng;

use rocket::State;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;



#[derive(Serialize)]
pub struct Success {
    token : String
}


#[derive(Debug, Deserialize)]
pub struct Body {
    email    : String,
    name     : String,
    password : String
}
#[post("/signin", data = "<body>")]
pub fn route(body: Json<Body>, db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<Json<Success>, Status> {
    let mut conn = db_pool.get().unwrap();

    let hashed_pwd = hash(&body.password, bcrypt::DEFAULT_COST).map_err(|err| send_error(500,format!("Can't hash password with bcrypt : {}",err)))?;
    println!("Password hashed");

    let clear_token = format!("{}-{}",&body.email,rand::thread_rng().gen_range(0..=10000000));
    let mut hasher = Sha256::new();
    hasher.update(clear_token.as_bytes());
    let token = format!("{:x}",hasher.finalize());
    println!("token made");
    
   




    let tx = conn.transaction().unwrap();

    tx.execute("INSERT INTO user (email, name, password) VALUES (?, ?, ?)", &[&body.email, &body.name, &hashed_pwd])
    .map_err(|err| 
        match err { 
            rusqlite::Error::SqliteFailure(sqlite_err, _) =>{
                if sqlite_err.code == rusqlite::ErrorCode::ConstraintViolation {
                    return send_error(409,"User already in DB".to_string());
                }
                send_error(500,err.to_string())
            }
            _ => send_error(500,err.to_string())
        }
    )?;
    log("query 1/2 : user added in DB".to_string());
    
    tx.execute("INSERT INTO connexion (token, email) VALUES (?, ?)", &[&token, &body.email])
    .map_err(|err| send_error(500,err.to_string()))?;
    log("query 2/2 : token,... added in connexion".to_string());

    tx.commit().unwrap();
    log("transaction done : user corrcly created".to_string());

    
    Ok(Json(Success{token: token}))
}
