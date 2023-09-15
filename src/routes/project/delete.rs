use rocket::http::Status;
use rusqlite::Result;

use crate::routes::auth_tools::*;
use crate::routes::discover_rigt::*;
use crate::routes::log::*;

use rocket::State;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::routes::bearertoken::BearerToken;

#[delete("/<id_project>")]
pub fn route(id_project :i64,token: BearerToken,db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<(), Status> {
    
    let mut conn = db_pool.get().unwrap();
    
    let caller_email  = email_from_token(&conn, &token.value).map_err(|err| send_error(500, err.to_string()))?;
    if  caller_email == "" { return Err(send_error(401, "Oops : can't find email associate to this token".to_string()));}
    log("request made by ".to_string() + &caller_email);
    
    if right_from_project(&conn, &caller_email, &id_project).map_err(|err| send_error(500,err.to_string()))? != 1{
        return Err(send_error(403, format!("can't delete project {} because you have not owner right",id_project) ));
    }

    let tx = conn.transaction().unwrap();

    let _nbl = tx.execute("DELETE FROM project WHERE id = ?", &[&id_project])
    .map_err(|err| send_error(500, err.to_string()))?;
    log("query 1/2 : project row removed".to_string());

    let _nbl = tx.execute("DELETE FROM project_user WHERE id_project = ?", &[&id_project])
    .map_err(|err| send_error(500,err.to_string()))?;
    log("query 2/2 : project_user projects deleted".to_string());

    tx.commit().unwrap();
    log("transaction done : project correcly deleted".to_string());
    
    Ok(())
}





