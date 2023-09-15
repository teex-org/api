


pub mod auth;
pub mod user;
pub mod project;
pub mod list;
pub mod task;


pub mod log{
    use rocket::http::Status;

    pub fn send_error(code:u16, err: String) -> Status{
        if (code == 500){
            error(format!("{} | {}",code, err));
        }
        else{
            oops(format!("{} | {}",err,code));
        }
        Status::from_code(code).unwrap_or(Status::InternalServerError)
    }

    pub fn error(msg:String){
        println!("!! \x1b[31m{}\x1b[0m",msg)
    } 
    pub fn oops(msg:String){
        println!("•• \x1b[33m{}\x1b[0m",msg)
    } 
    pub fn log(msg:String){
        println!("••  {}",msg)
    } 
}
pub mod auth_tools {
    use rusqlite::{Connection, Error};
    pub fn email_from_token(conn: &Connection,token:&String) -> Result<String,Error>{
        let result = conn.query_row("SELECT email FROM connexion WHERE token = ?", &[token], |row| {row.get(0)});
        match result {
            Err(Error::QueryReturnedNoRows) => Ok("".to_string()),
            Ok(value) => Ok(value),
            Err(e)    => Err(e),
        }
    }
}
pub mod discover_rigt{
    use rusqlite::{Connection, Error};

    pub fn right_from_project(conn: &Connection,email: &String, project_id: &i64) -> Result<i32,Error>{
        let result = conn.query_row("SELECT right FROM project_user WHERE email = ? AND id_project = ?", &[email,&project_id.to_string()], |row| row.get(0));
        match result {
            Err(Error::QueryReturnedNoRows) => Ok(-1),
            Ok(value) => Ok(value),
            Err(e)    => Err(e),
        }
    }
    
    pub fn right_from_list(conn: &Connection,email: &String, list_id: &i64) -> Result<i32,Error>{
        let result = conn.query_row("
        SELECT pu.right 
        FROM project_user AS pu
        JOIN list AS l ON pu.id_project = l.id_project
        WHERE l.id = ? AND pu.email = ?;",
        &[&list_id.to_string(),email], |row| row.get(0));
        match result {
            Err(Error::QueryReturnedNoRows) => {println!("hhh"); Ok(-1)},
            Ok(value) => {println!("{}",value); Ok(value)},
            Err(e)    => Err(e),
        }
    }
    
    pub fn right_from_task(conn: &Connection,email: &String, task_id: &i64) -> Result<i32,Error>{
        let result = conn.query_row("
        SELECT pu.right 
        FROM project_user AS pu
        JOIN list AS l ON pu.id_project = l.id_project
        JOIN task AS t ON l.id = t.id_list
        WHERE t.id = ? AND pu.email = ?;
        ",
        &[&task_id.to_string(),email], |row| row.get(0));
        match result {
            Err(Error::QueryReturnedNoRows) => Ok(-1),
            Ok(value) => Ok(value),
            Err(e)    => Err(e),
        }
    }
}
pub mod bearertoken{
    use rocket::request::{FromRequest, Outcome, Request};
    use rocket::http::Status;

    pub struct BearerToken{
        pub value:  String
    }


    #[derive(Debug)]
    pub struct BearerError;

    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for BearerToken {
        type Error = BearerError;

        async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
            match request.headers().get_one("Authorization") {
                Some(header) if header.starts_with("Bearer ") => {
                    Outcome::Success(BearerToken{value: header[7..].to_string()})
                },
                _ => Outcome::Failure((Status::Unauthorized, BearerError))
            }
        }
    }

}

