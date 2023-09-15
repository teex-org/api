use rocket::Route;
mod post;
mod delete;
mod put;
mod update_state;


pub fn all_routes() -> Vec<Route> {
    routes![
        post::route,
        delete::route,
        put::route,
        update_state::route,
        ]
}
