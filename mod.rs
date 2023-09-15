pub mod user;
pub mod error_tools;

use rocket::Route;


pub fn all_routes() -> Vec<Route> {
    user::all_routes()
}