use rocket::request::Form;
use rocket_contrib::json::JsonValue;

use crate::http::console::PagingParams;
use crate::{User, UserFlag, conn};
use crate::repo::UserRepo;

#[derive(FromForm, Debug)]
pub struct UserEntity {
    pub user: String,
    pub name: String,
    pub pwd: String,
    pub flag: i8,
    pub enabled: bool,
    pub email: String,
}

#[post("/user/list", data = "<p>")]
pub fn user_list(p: PagingParams) -> JsonValue {
    let begin = ((p.page - 1) * p.rows) as i64;
    let over = begin + p.rows as i64;
    let (total, rows) =  UserRepo::take_users(&conn(),begin, over);
    json!({"total":total,"rows":rows})
}

#[post("/user/get?<user>")]
pub fn get_user(user: String) -> JsonValue {
    match User::get_user(&user) {
        Some(u) => json!(u),
        None => json!({"err_msg":"用户不存在"}),
    }
}

#[post("/user/save", data = "<user>")]
pub fn save_user(user: Form<UserEntity>) -> JsonValue {
    let mut u = User::get_user(&user.user).unwrap_or(User {
        user: user.user.to_lowercase(),
        name: "".to_string(),
        pwd: "".to_string(),
        flag: UserFlag::Enabled as i8,
        email: "".to_string(),
        api_tokens: vec![],
    });
    u.name = user.name.clone();
    if u.name == "" {
        u.name = u.user.to_owned();
    }
    if u.pwd != user.pwd && user.pwd != "" {
        u.pwd = User::pwd(user.pwd.to_owned());
    }
    u.flag = u.flag | user.flag;
    if let Some(u2) = User::get_user_mail(&user.email) {
        if u2.user != u.user {
            return json!({"code":1,"err_msg":"邮箱已经被使用"});
        }
    } else {
        u.email = user.email.to_owned();
    }
    let e_f = UserFlag::Enabled as i8;
    if user.enabled {
        u.flag = u.flag | e_f;
    } else if u.flag & e_f == e_f {
        let super_f = UserFlag::SuperUser as i8;
        if u.flag & super_f == super_f {
            return json!({"code":1,"err_msg":"不能停用超级管理员"});
        }
        u.flag = u.flag ^ e_f;
    }
    match User::save_user(&u) {
        Ok(_) => json!({"code":0}),
        Err(err) => json!({"code":1,"err_msg":err}),
    }
}
