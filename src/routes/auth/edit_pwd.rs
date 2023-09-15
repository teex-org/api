use rocket::serde::{Serialize,Deserialize};
use rocket::serde::json::Json;
use rocket::http::Status;
use rusqlite::Result;

use crate::routes::auth_tools::*;
use crate::routes::log::*;
use crate::routes::bearertoken::BearerToken;
use super::token::create_token;

use rocket::State;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;


#[derive(Serialize)]
pub struct Success {
    token : String
}


#[derive(Debug, Deserialize)]
pub struct Body {
    password : String
}

#[put("/pwd", data = "<body>")]
pub fn route(body: Json<Body>, token: BearerToken, db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<Json<Success>, Status> {
    let mut conn = db_pool.get().unwrap();


    let caller_email  = email_from_token(&conn, &token.value).unwrap();
    if  caller_email == "" { return Err(send_error(401, "Oops : can't find email associate to this token".to_string()));}
    
    log("request made by ".to_string() + &caller_email);
    
    let new_token = create_token(&caller_email);



    // transaction
    let tx       = conn.transaction().unwrap();

    tx.execute("UPDATE user SET password = ? WHERE email = ?", &[&body.password, &caller_email])
    .map_err(|err| send_error(500,err.to_string()))?;
    log("query 1/2 : Password updated".to_string());
    
    tx.execute("UPDATE connexion SET token = ? WHERE email = ?", &[&new_token, &caller_email])
    .map_err(|err| send_error(500,err.to_string()))?;
    log("query 2/2 : token updated".to_string());

    tx.commit().unwrap();
    log("transaction done : user corrcly updated".to_string());


    //.map_err(|(step, err)|{ send_error(500,format!("DB : error step {} > {}",step, &err.to_string()))})?;
    Ok(Json(Success{token:new_token}))
}


