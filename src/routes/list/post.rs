

use rocket::serde::{Serialize,Deserialize};
use rocket::serde::json::Json;
use rocket::http::Status;
use rusqlite::Result;
use crate::routes::bearertoken::BearerToken;

use rocket::State;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::routes::auth_tools::*;
use crate::routes::discover_rigt::*;
use crate::routes::log::*;


#[derive(Serialize)]
pub struct Success {
    id : i64,
}


#[derive(Debug, Deserialize)]
pub struct Body {
    id_project : i64,
    name       : String
}
#[post("/", data = "<body>")]
pub fn route(body: Json<Body>,token: BearerToken, db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<Json<Success>, Status> {

    let conn = db_pool.get().unwrap();

    let caller_email  = email_from_token(&conn, &token.value).map_err(|err| send_error(500, err.to_string()))?;
    if  caller_email == "" { return Err(send_error(401, "Oops : can't find email associate to this token".to_string()));}
    log("request made by ".to_string() + &caller_email);
    
    if right_from_project(&conn, &caller_email, &body.id_project).map_err(|err| send_error(500,err.to_string()))? == -1{
        return Err(send_error(403, format!("can't create list in project {} because you not in the project",&body.id_project) ));
    }


    conn.execute("INSERT INTO list (name,id_project) VALUES (?,?)", &[&body.name,&body.id_project.to_string()])
    .map_err(|err| send_error(500, err.to_string()))?;
    
    let list_id = conn.last_insert_rowid();
    log(format!("id of the list : {}",list_id));

    
    Ok(Json(Success{id: list_id}))
}
