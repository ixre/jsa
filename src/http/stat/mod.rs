use rocket::Route;

mod up;

pub fn routes() -> Vec<Route> {
    routes![up::site_po]
}
