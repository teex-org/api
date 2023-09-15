use rocket::http::Status;
use rusqlite::Result;

use rocket::State;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::routes::auth_tools::*;
use crate::routes::discover_rigt::*;
use crate::routes::log::*;

use crate::routes::bearertoken::BearerToken;

#[put("/state/<id>")]
pub fn route(id: i64, token: BearerToken, db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<(), Status> {
    let conn = db_pool.get().unwrap();
    
    let caller_email  = email_from_token(&conn, &token.value).map_err(|err| send_error(500, err.to_string()))?;
    if  caller_email == "" { return Err(send_error(401, "Oops : can't find email associate to this token".to_string()));}
    log("request made by ".to_string() + &caller_email);
    
    

    if right_from_task(&conn, &caller_email, &id).map_err(|err| send_error(500,err.to_string()))? != 1{
        return Err(send_error(403, format!("can't delete list {} because you have not owner right",id) ));
    }

    conn.execute("UPDATE task SET state = (state+1)%3 WHERE id = ?", &[&id.to_string()]).map_err(|err| send_error(500, err.to_string()))?;


    Ok(())
}


