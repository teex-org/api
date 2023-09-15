mod signin;
mod login;
mod edit_pwd;
use rocket::Route;

pub fn all_routes() -> Vec<Route> {
    routes![
        signin::route,
        login::route,
        edit_pwd::route
        ]
}
mod token{
    use rand::Rng;
    use sha2::{Sha256, Digest};
    pub fn create_token(email : &String) -> String{
        let clear_token = format!("{}-{}",email,rand::thread_rng().gen_range(0..=10000000));
        let mut hasher = Sha256::new();
        hasher.update(clear_token.as_bytes());
        format!("{:x}",hasher.finalize())
    }
}
