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
use rusqlite::params_from_iter;

#[derive(Debug, Deserialize)]
pub struct Body {
    name           : Option<String>,
    descr          : Option<String>,
    tag            : Option<i64>,
    priority       : Option<i64>,
}
#[put("/<id>", data = "<body>")]
pub fn route(id: i64, body: Json<Body>, token: BearerToken, db_pool: &State<Pool<SqliteConnectionManager>>) -> Result<(), Status> {
    let conn = db_pool.get().unwrap();
    
    let caller_email  = email_from_token(&conn, &token.value).map_err(|err| send_error(500, err.to_string()))?;
    if  caller_email == "" { return Err(send_error(401, "Oops : can't find email associate to this token".to_string()));}
    log("request made by ".to_string() + &caller_email);
    
    

    if right_from_task(&conn, &caller_email, &id).map_err(|err| send_error(500,err.to_string()))? != 1{
        return Err(send_error(403, format!("can't delete list {} because you have not owner right",id) ));
    }

    let mut sql            = "UPDATE task SET ".to_string();
    let mut values         = vec![];





    if  let Some(name)     = &body.name     {sql = sql + "name = ? ,";       values.push(name)};
    if  let Some(descr)    = &body.descr    {sql = sql + "descr = ? ,";      values.push(descr)};

    let tag       = body.tag.unwrap_or(-1).to_string();
    let priority  = body.priority.unwrap_or(-1).to_string();
    if  tag != "-1" {
        sql = sql + "tag = ? ,";
        values.push(&tag);
    };
    if  priority != "-1" {
        sql = sql + "priority = ? ,";
        values.push(&priority);
    };


    if sql != "UPDATE task "{
        sql = sql + "id_last_editor = ? ";
        values.push(&caller_email);

        sql = sql + "WHERE id = ?";
        log(format!("{}",&sql));
        for i in &values{
            log(format!("{}",i));
        }

        let id_str = id.to_string();
        values.push(&id_str);
        
        let _nbl = conn.execute(&sql, params_from_iter(values)).map_err(|err| send_error(500, err.to_string()))?;
        
    }

    Ok(())
}


