use rocket::http::Status;
use rusqlite::Result;

use rocket::State;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::routes::log::*;
use crate::routes::bearertoken::BearerToken;
use crate::routes::discover_rigt::*;
use crate::routes::auth_tools::*;


#[delete("/<id>/kick/<email>")]
pub fn route(id: i64, email: String, token: BearerToken, db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<(), Status> {
    let conn = db_pool.get().unwrap();
    
    let caller_email  = email_from_token(&conn, &token.value).map_err(|err| send_error(500, err.to_string()))?;
    if  caller_email == "" { return Err(send_error(401, "Oops : can't find email associate to this token".to_string()));}
    log("request made by ".to_string() + &caller_email);


    if right_from_project(&conn, &caller_email, &id).map_err(|err| send_error(500,err.to_string()))? == -1{
        return Err(send_error(403, format!("can't edit project {} because you are not in the project",id) ));
    }

    conn.execute("DELETE FROM project_user WHERE id_project = ? AND email = ", &[&id.to_string(),&email]).map_err(|err| send_error(500, err.to_string()))?;

    Ok(())
}


