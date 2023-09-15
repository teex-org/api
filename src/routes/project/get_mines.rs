use rocket::serde::Serialize;
use rocket::serde::json::Json;
use rocket::http::Status;
use rusqlite::Result;

use crate::routes::auth_tools::*;
use crate::routes::log::*;

use rocket::State;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use crate::routes::bearertoken::BearerToken;




#[derive(Serialize)]
pub struct Success {
    projects   : Vec<Project>
}
#[derive(Serialize)]
pub struct Project {
    id   : i64,
    name : String
}

#[get("/mines")]
pub fn route(token: BearerToken, db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<Json<Success>, Status> {
    let conn = db_pool.get().unwrap();

    let caller_email  = email_from_token(&conn, &token.value).map_err(|err| send_error(500, err.to_string()))?;
    if  caller_email == "" { return Err(send_error(401, "Oops : can't find email associate to this token".to_string()));}
    log("request made by ".to_string() + &caller_email);
    

    let mut stmt = conn.prepare("SELECT p.id, p.name FROM project p JOIN project_user pu ON p.id = pu.id_project WHERE pu.email = ?")
    .map_err(|err| send_error(500, err.to_string()))?;

    let project_rows = stmt.query_map(&[&caller_email], |row| {
        Ok(Project {
            id   : row.get(0)?,
            name : row.get(1)?
        })
    }).map_err(|err| send_error(500, err.to_string()))?;

    let mut projects = Vec::new();
    for project_row in project_rows {
        projects.push(project_row.map_err(|err| send_error(500, err.to_string()))?);
    }

 
    Ok(Json(Success {projects : projects}))
}
