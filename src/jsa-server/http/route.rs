use rocket::fairing::AdHoc;
use rocket::http::Header;
use rocket::Request;
use rocket::response::Responder;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;

use super::console;
use super::index;
use super::WrappedResult;

pub fn mount_routes(r: Rocket) -> Rocket {
    let r = r
        .mount(
            "/",
            routes![index::index, index::all, index::favicon, index::board],
        )
        .mount("/static", StaticFiles::from("./static"))
        .mount("/console/api", routes![console::login,
            console::check_session,
            console::initial,
            console::logout])
        .mount("/console", StaticFiles::from("./app"));
    attach_user_middleware(r)
}

/// Check user session middleware.
fn attach_user_middleware(r: Rocket) -> Rocket {
    r.attach(AdHoc::on_response("", |req, rsp| {
        rsp.set_header(Header::new("JSA-Version", "1.0"));
        rsp.remove_header("Server");
        let path = req.uri().path();
        let method = req.method().to_string();
        let api_req = path.starts_with("/console/api/"); // Is a api request
        let login_req = path.starts_with("/console/api/login"); // Is a login request
        let is_login_ok = check_login(req);
        // Set CORS header for api request if user is logged.
        if login_req || (api_req && is_login_ok) {
            if let Some(o) = req.headers().get_one("Origin") {
                rsp.set_header(Header::new("Access-Control-Allow-Origin", String::from(o)));
                rsp.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
                rsp.set_header(Header::new(
                    "Access-Control-Allow-Headers",
                    "Content-Type,X-Requested-With",
                ));
            }
        }
        // If login ok or send a login request
        if is_login_ok || login_req {
            return;
        }
        // Redirect to login page if request path '/board' or api request!
        if api_req {
            rsp.merge(
                WrappedResult::new(-100, "access denied", "")
                    .respond_to(req)
                    .unwrap(),
            );
            return;
        }
    }))
}

fn check_login(req: &Request) -> bool {
    if let Some(_) = req.cookies().get("SessionID").map(|cookie| cookie.value()) {
        return true;
    }
    false
}
