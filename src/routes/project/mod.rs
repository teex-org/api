use rocket::Route;
mod post;
mod get;
mod get_mines;
mod delete;
mod put;
mod kick;

pub fn all_routes() -> Vec<Route> {
    routes![
        post::route,
        get::route,
        get_mines::route,
        delete::route,
        put::route,
        kick::route,
        ]
}
