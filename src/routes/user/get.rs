use rocket::serde::Serialize;
use rocket::serde::json::Json;
use rocket::http::Status;
use rusqlite::Result;

use rocket::State;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::routes::log::*;
use crate::routes::bearertoken::BearerToken;
use crate::routes::auth_tools::*;


#[derive(Serialize)]
pub struct Success {
    email : String,
    name  : String
}

#[get("/<email>")]
pub fn route(email: String, token: BearerToken, db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<Json<Success>, Status> {
    let conn = db_pool.get().unwrap();

    let caller_email  = email_from_token(&conn, &token.value).map_err(|err| send_error(500, err.to_string()))?;
    if  caller_email == "" { return Err(send_error(401, "Oops : can't find email associate to this token".to_string()));}
    log("request made by ".to_string() + &caller_email);
    
    let result = conn.query_row("SELECT * FROM user WHERE email = ?", &[&email], 
    |row| {
        Ok(
            Success{
                email    : row.get(0)?,
                name     : row.get(1)?,
            }
        )
    })
    .map_err(|err| 
        
        match err{
            rusqlite::Error::QueryReturnedNoRows => send_error(404,format!("User {} not found", &err.to_string())),
            _ => send_error(500,format!("DB : error tryin to get user :\n {}", &err.to_string()))
        }
    )?;
    
    Ok(Json(result))
}





