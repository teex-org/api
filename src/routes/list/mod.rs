use rocket::Route;
mod post;
mod delete;
mod put;


pub fn all_routes() -> Vec<Route> {
    routes![
        post::route,
        delete::route,
        put::route
        ]
}
