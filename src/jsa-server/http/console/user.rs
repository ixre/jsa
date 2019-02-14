use rocket_contrib::json::JsonValue;

use crate::http::console::PagingParams;
use crate::User;

#[post("/user/list", data = "<p>")]
pub fn user_list(p: PagingParams) -> JsonValue {
    let begin = ((p.page - 1) * p.rows) as usize;
    let over = begin+p.rows as usize;
    let (total, rows) = User::take_users(begin,over);
    json!({"total":total,"rows":rows})
}
