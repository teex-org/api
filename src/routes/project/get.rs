use rocket::serde::Serialize;
use rocket::serde::json::Json;
use rocket::http::Status;
use rusqlite::Result;
use rocket::State;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use crate::routes::bearertoken::BearerToken;

use crate::routes::auth_tools::*;
use crate::routes::discover_rigt::*;
use crate::routes::log::*;


#[derive(Serialize)]
pub struct Task {
    id             : i64,
    id_list        : i64,
    name           : String,
    descr          : String,
    tag            : i64,
    priority       : i64,
    state          : i64,
    id_last_editor : String,
}


#[derive(Serialize)]
pub struct List {
    id    : i64,
    name  : String,
    tasks : Vec<Task>
}


#[derive(Serialize)]
pub struct User {
    email : String,
    name  : String,
    right : i64,
}

#[derive(Serialize)]
pub struct Project {
    id    : i64,
    name  : String,
    lists : Vec<List>,
    users : Vec<User>,
}

#[get("/<id>")]
pub fn route(id: i64, token: BearerToken, db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<Json<Project>, Status> {
    let conn = db_pool.get().unwrap();

    let caller_email  = email_from_token(&conn, &token.value).map_err(|err| send_error(500, err.to_string()))?;
    if  caller_email == "" { return Err(send_error(401, "Oops : can't find email associate to this token".to_string()));}
    log("request made by ".to_string() + &caller_email);
    
    if right_from_project(&conn, &caller_email, &id).map_err(|err| send_error(500,err.to_string()))? == -1{
        return Err(send_error(403, format!("can't get project {} because you are not in the project",id) ));
    }

    let mut project = Project{
        id    : id,
        name  : String::new(),
        lists : Vec::new(),
        users : Vec::new(),
    };

    let name : String = conn.query_row("SELECT name FROM project WHERE id = ?", &[&id], |row| row.get(0))
    .map_err(|err| 
        match err{
            rusqlite::Error::QueryReturnedNoRows => send_error(404,format!("User {} not found", &err.to_string())),
            _ => send_error(500,format!("DB : error tryin to get user :\n {}", &err.to_string()))
        }
    )?;
    project.name = name;


    // USERS
    let mut stmt = conn.prepare("SELECT u.email, u.name, pu.right FROM user u JOIN project_user pu ON u.email = pu.email WHERE pu.id_project = ?")
    .map_err(|err| send_error(500, err.to_string()))?;

    let user_rows = stmt.query_map(&[&id], |row| {
        Ok(User {
            email : row.get(0)?,
            name  : row.get(1)?,
            right : row.get(2)?,
        })
    }).map_err(|err| send_error(500, err.to_string()))?;

    let mut users = Vec::new();
    for user_row in user_rows {
        users.push(user_row.map_err(|err| send_error(500, err.to_string()))?);
    }
    project.users = users;


    //LIST 
    let mut stmt = conn.prepare("SELECT id, name FROM list WHERE id_project = ?")
    .map_err(|err| send_error(500, err.to_string()))?;

    let list_rows = stmt.query_map(&[&id], |row| {
        Ok(List {
            id    : row.get(0)?,
            name  : row.get(1)?,
            tasks : Vec::new()
        })
    }).map_err(|err| send_error(500, err.to_string()))?;

    let mut lists = Vec::new();
    for list_row in list_rows {
        let mut list = list_row.map_err(|err| send_error(500, err.to_string()))?;
        


        let mut stmt = conn.prepare("SELECT * FROM task WHERE id_list = ?")
        .map_err(|err| send_error(500, err.to_string()))?;
        
        let tasks_rows = stmt.query_map(&[&list.id], |row| {
            Ok(Task {
                id             : row.get(0)?,
                name           : row.get(1)?,
                tag            : row.get(2)?,
                priority       : row.get(3)?,
                state          : row.get(4)?,
                descr          : row.get(5)?,
                id_last_editor : row.get(6)?,
                id_list        : row.get(7)?
            })
        }).map_err(|err| send_error(500, err.to_string()))?;
        
        
        for task_row in tasks_rows{
            let task = task_row.map_err(|err| send_error(500, err.to_string()))?;
            list.tasks.push(task);
        }

        lists.push(list);
    }
    project.lists = lists;
    
    Ok(Json(project))
}





