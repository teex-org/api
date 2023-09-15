#[macro_use] extern crate rocket;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

mod routes;
mod catchers;



use crate::catchers::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(Pool::new(SqliteConnectionManager::file("database/database.sqlite")).unwrap())
    .register("/", catchers![unauthorized,forbidden,not_found,internal_server_error,conflict])
    .mount("/api/auth",     routes::auth::all_routes())
    .mount("/api/user",     routes::user::all_routes())
    .mount("/api/project",  routes::project::all_routes())
    .mount("/api/list",     routes::list::all_routes())
    .mount("/api/task",     routes::task::all_routes())
}
