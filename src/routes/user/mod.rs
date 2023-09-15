use rocket::Route;
mod get;
mod put;
mod delete;

pub fn all_routes() -> Vec<Route> {
    routes![
        get::route,
        put::route,
        delete::route
        ]
}
