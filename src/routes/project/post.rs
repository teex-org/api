

use rocket::serde::{Serialize,Deserialize};
use rocket::serde::json::Json;
use rocket::http::Status;
use rusqlite::Result;
use crate::routes::bearertoken::BearerToken;

use rocket::State;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::routes::auth_tools::*;
use crate::routes::log::*;

#[derive(Serialize)]
pub struct Success {
    id : i64,
}


#[derive(Debug, Deserialize)]
pub struct Body {
    name    : String
}
#[post("/", data = "<body>")]
pub fn route(body: Json<Body>,token: BearerToken, db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<Json<Success>, Status> {

    let mut conn = db_pool.get().unwrap();

    let caller_email  = email_from_token(&conn, &token.value).map_err(|err| send_error(500, err.to_string()))?;
    if  caller_email == "" { return Err(send_error(401, "Oops : can't find email associate to this token".to_string()));}
    log("request made by ".to_string() + &caller_email);



    let tx       = conn.transaction().unwrap();

    tx.execute("INSERT INTO project (name) VALUES (?)", &[&body.name])
    .map_err(|err| send_error(500, err.to_string()))?;
    log("query 1/2 : Password update in db".to_string());
    
    let project_id = tx.last_insert_rowid();
    log(format!("id of the project : {}",project_id));

    tx.execute("INSERT INTO project_user (id_project, email, right) VALUES (?,?,?)", &[&project_id.to_string(), &caller_email, "1"])
    .map_err(|err| send_error(500, err.to_string()))?;
    log("query 2/2 : project_user mapped".to_string());

    tx.commit().unwrap();
    log("transaction done : project correcly created".to_string());

    
    Ok(Json(Success{id: project_id}))
}
