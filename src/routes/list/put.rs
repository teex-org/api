use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::http::Status;
use rusqlite::Result;

use rocket::State;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::routes::auth_tools::*;
use crate::routes::discover_rigt::*;
use crate::routes::log::*;

use crate::routes::bearertoken::BearerToken;


#[derive(Debug, Deserialize)]
pub struct Body {
    name  : String
}
#[put("/<id>", data = "<body>")]
pub fn route(id: i64, body: Json<Body>, token: BearerToken, db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<(), Status> {
    let conn = db_pool.get().unwrap();
    
    let caller_email  = email_from_token(&conn, &token.value).map_err(|err| send_error(500, err.to_string()))?;
    if  caller_email == "" { return Err(send_error(401, "Oops : can't find email associate to this token".to_string()));}
    log("request made by ".to_string() + &caller_email);
    

    if right_from_project(&conn, &caller_email, &id).map_err(|err| send_error(500,err.to_string()))? != 1{
        return Err(send_error(403, format!("can't delete list {} because you have not owner right",id) ));
    }

    conn.execute("UPDATE list SET name = ? WHERE id = ?", &[&body.name,&id.to_string()]).map_err(|err| send_error(500, err.to_string()))?;

    Ok(())
}


