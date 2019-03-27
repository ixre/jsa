use std::collections::HashMap;
use std::io::Read;

use rocket::data::FromData;
use rocket::data::Outcome;
use rocket::data::Transform;
use rocket::data::Transformed;
use rocket::http::Status;
use rocket::outcome::Outcome::Failure;
use rocket::outcome::Outcome::Success;
use rocket::Data;
use rocket::Request;
use rocket::Route;

mod console;
mod user;

/// examples:
/// ```
/// #[post("/user/list", data = "<p>")]
/// pub fn user_list(p: PagingParams) -> JsonValue {
///    json!({page:p.page,rows:p.rows})
/// }
/// ```
pub struct PagingParams {
    page: i32,
    rows: i32,
    data: HashMap<String, String>,
}

impl PagingParams {
    pub fn new() -> Self {
        Self {
            page: 0,
            rows: 0,
            data: HashMap::new(),
        }
    }
}

const NAME_LIMIT: u64 = 256;

///[Rocket trait.FromData](https://api.rocket.rs/v0.4/rocket/data/trait.FromData.html)
impl<'a> FromData<'a> for PagingParams {
    type Error = ();
    type Owned = String;
    type Borrowed = str;

    fn transform(_: &Request, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>> {
        let mut stream = data.open().take(NAME_LIMIT);
        let mut string = String::with_capacity((NAME_LIMIT / 2) as usize);
        let outcome = match stream.read_to_string(&mut string) {
            Ok(_) => Success(string),
            Err(e) => Failure((Status::InternalServerError, ())),
        };
        // Returning `Borrowed` here means we get `Borrowed` in `from_data`.
        Transform::Borrowed(outcome)
    }

    fn from_data(_: &Request, outcome: Transformed<'a, Self>) -> Outcome<Self, Self::Error> {
        // Retrieve a borrow to the now transformed `String` (an &str). This
        // is only correct because we know we _always_ return a `Borrowed` from
        // `transform` above.
        let string = outcome.borrowed()?;
        let mut p = PagingParams::new();
        let url = url::form_urlencoded::parse(string.as_bytes());
        for (key, value) in url.into_owned() {
            match key.as_str() {
                "page" => p.page = value.parse().unwrap_or(1),
                "size" => p.rows = value.parse().unwrap_or(10),
                _ => {
                    p.data.insert(key, value);
                }
            }
        }
        Success(p)
        // Return successfully.
        //Success(PagingParams { first: splits[0], last: splits[1] })
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![
        console::login,
        console::check_session,
        console::initial,
        console::logout,
        user::user_list,
        user::get_user,
        user::save_user
    ]
}
