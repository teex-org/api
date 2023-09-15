

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
    id_list  : i64,
    name     : String,
    tag      : i64,
    priority : i64,
}
#[post("/", data = "<body>")]
pub fn route(body: Json<Body>,token: BearerToken, db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<Json<Success>, Status> {

    let conn = db_pool.get().unwrap();

    let caller_email  = email_from_token(&conn, &token.value).map_err(|err| send_error(500, err.to_string()))?;
    if  caller_email == "" { return Err(send_error(401, "Oops : can't find email associate to this token".to_string()));}
    log("request made by ".to_string() + &caller_email);
    
    if right_from_list(&conn, &caller_email, &body.id_list).map_err(|err| send_error(500,err.to_string()))? == -1{
        return Err(send_error(403, format!("can't create list in project {} because you not in the project",&body.id_list) ));
    }


    conn.execute("INSERT INTO task (name,tag,priority,id_last_editor,id_list,state,descr) VALUES (?,?,?,?,?,?,?)", 
    &[&body.name,&body.tag.to_string(),&body.priority.to_string(),&caller_email,&body.id_list.to_string(),"0",""])
    .map_err(|err| send_error(500, err.to_string()))?;
    
    let task_id = conn.last_insert_rowid();
    log(format!("id of the task : {}",task_id));

    
    Ok(Json(Success{id: task_id}))
}
