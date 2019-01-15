use std::io::Cursor;

use rocket::fairing::AdHoc;
use rocket::http::Header;
use rocket::response::Responder;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;

use crate::http::console;
use crate::http::index;
use crate::http::WrappedResult;

pub fn mount_routes(r: Rocket) -> Rocket {
    let r = r
        .mount(
            "/",
            routes![index::index, index::all, index::favicon, index::board],
        )
        .mount("/static", StaticFiles::from("./static"))
        .mount("/console/api", routes![console::login, console::login2])
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
        if path == "/board" || (
            path.starts_with("/console/api/") && !path.starts_with("/console/api/login")) {
            if method == "GET" {
                rsp.set_sized_body(Cursor::new("<script>location.assign(\
                '/console/#/login?callback=')</script>"));
            } else {
                rsp.merge(
                    WrappedResult::new(-100, "access denied", "")
                        .respond_to(req)
                        .unwrap(),
                );
            }
            return;
        } else {}
    }))
}
