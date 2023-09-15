use rocket::http::Status;
use rusqlite::Result;

use crate::routes::log::*;
use crate::routes::bearertoken::BearerToken;
use crate::routes::auth_tools::*;

use rocket::State;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;


#[delete("/")]
pub fn route(token: BearerToken,db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<(), Status> {
    
    let mut conn = db_pool.get().unwrap();

    let caller_email  = email_from_token(&conn, &token.value).map_err(|err| send_error(500, err.to_string()))?;
    if  caller_email == "" { return Err(send_error(401, "Oops : can't find email associate to this token".to_string()));}
    log("request made by ".to_string() + &caller_email);
    

    let tx = conn.transaction().unwrap();

    let _nbl = tx.execute("DELETE FROM user WHERE email = ?", &[&caller_email])
    .map_err(|err| send_error(500, err.to_string()) )?;
    log("query 1/2 : user's row removed".to_string());

    let _nbl = tx.execute("DELETE FROM connexion WHERE email = ?", &[&caller_email]).map_err(|err| send_error(500,err.to_string()))?;
    log("query 2/2 : project_user mapped".to_string());

    tx.commit().unwrap();
    log("transaction done : user correcly deleted".to_string());
    
    Ok(())
}





