use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::http::Status;
use rusqlite::Result;


use rocket::State;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use rusqlite::params_from_iter;

use crate::routes::log::*;
use crate::routes::bearertoken::BearerToken;
use crate::routes::auth_tools::*;


#[derive(Debug, Deserialize)]
pub struct Body {
    email : String,
    name  : String
}
#[put("/", data = "<body>")]
pub fn route(body: Json<Body>, token: BearerToken, db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<(), Status> {
    let conn = db_pool.get().unwrap();
    
    let caller_email  = email_from_token(&conn, &token.value).map_err(|err| send_error(500, err.to_string()))?;
    if  caller_email == "" { return Err(send_error(401, "Oops : can't find email associate to this token".to_string()));}
    log("request made by ".to_string() + &caller_email);
    
    let mut sql = "UPDATE user SET ".to_string();
    let mut values = vec![];
    if body.email != "" { sql = sql + "SET email = ? "; values.push(&body.email);}
    if body.name  != "" { sql = sql + "SET name = ? ";  values.push(&body.name) ;}

    if sql != "UPDATE user SET "{
        sql = sql + "WHERE email = ?";
        conn.execute(&sql, params_from_iter(values)).map_err(|err| send_error(500, err.to_string()))?;
    }

    Ok(())
}


