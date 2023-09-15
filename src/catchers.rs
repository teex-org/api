
use rocket::Request;
use rocket::response::status;

#[catch(401)]
pub fn unauthorized(_req: &Request) -> status::Custom<&'static str> {
    status::Custom(rocket::http::Status::Unauthorized, "")
}

#[catch(403)]
pub fn forbidden(_req: &Request) -> status::Custom<&'static str> {
    status::Custom(rocket::http::Status::Forbidden, "")
}

#[catch(404)]
pub fn not_found(_req: &Request) -> status::Custom<&'static str> {
    status::Custom(rocket::http::Status::NotFound, "")
}

#[catch(409)]
pub fn conflict(_req: &Request) -> status::Custom<&'static str> {
    status::Custom(rocket::http::Status::Conflict, "")
}

#[catch(500)]
pub fn internal_server_error(_req: &Request) -> status::Custom<&'static str> {
    status::Custom(rocket::http::Status::InternalServerError, "")
}