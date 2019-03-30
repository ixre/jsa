use crate::http::console::PagingParams;
use rocket_contrib::json::JsonValue;
use crate::{conn, User};
use crate::repo::{DomainRepo, UserRepo};
use rocket::request::Form;
use rocket::http::Cookies;
use crate::models::user::UserFlag;
use crate::http::{session_user, Context};

#[post("/domain/list", data = "<p>")]
pub fn domain_list(p: PagingParams) -> JsonValue {
    let begin = ((p.page - 1) * p.rows) as i64;
    let over = begin + p.rows as i64;
    let (total, rows) = DomainRepo::take_domains(&conn(), begin, over);
    json!({"total":total,"rows":rows})
}

#[derive(FromForm, Debug)]
pub struct DomainEntity{
    pub domain:String,
    pub user_id:i32,
    pub notes:String,
    pub state:bool
}



#[post("/domain/save", data = "<domain>")]
pub fn save_domain(ctx: Context,domain: Form<DomainEntity>) -> JsonValue {
    let i =  UserFlag::SuperUser as i16;
    let u = session_user(&ctx.req.cookies()).unwrap();
    if (u.flag & i != i) && u.id != domain.user_id{
       return json!({"code":1,"err_msg":"no such user"});
    }
    json!({})
    /*

    let conn = conn();
    let mut u = UserRepo::get_user(&conn, &user.user).unwrap_or(User {
        id: 0,
        user: user.user.to_lowercase(),
        name: "".to_string(),
        pwd: "".to_string(),
        flag: UserFlag::Enabled as i16,
        email: "".to_string(),
        create_time: util::unix_sec() as i32,
    });
    u.name = user.name.clone();
    if u.name == "" {
        u.name = u.user.to_owned();
    }
    if u.pwd != user.pwd && user.pwd != "" {
        u.pwd = util::pwd(user.pwd.to_owned());
    }
    u.flag = u.flag | user.flag;
    if let Some(u2) = UserRepo::get_user_mail(&conn, &user.email) {
        if u2.user != u.user {
            return json!({"code":1,"err_msg":"邮箱已经被使用"});
        }
    } else {
        u.email = user.email.to_owned();
    }
    let e_f = UserFlag::Enabled as i16;
    if user.enabled {
        u.flag = u.flag | e_f;
    } else if u.flag & e_f == e_f {
        let super_f = UserFlag::SuperUser as i16;
        if u.flag & super_f == super_f {
            return json!({"code":1,"err_msg":"不能停用超级管理员"});
        }
        u.flag = u.flag ^ e_f;
    }
    match UserRepo::save_user(&conn, &u) {
        Ok(_) => json!({"code":0}),
        Err(err) => json!({"code":1,"err_msg":err.message()}),
    }
    */
}