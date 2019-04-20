use diesel::dsl::*;
use diesel::prelude::*;

use crate::errors::DataError;
use crate::models::user::{NewUser, User};
use crate::schema::u_user;
use crate::schema::u_user::dsl::*;
use crate::{Pool, UserFlag};

pub struct UserRepo {}
impl UserRepo {
    pub fn take_users(conn: &Pool, begin: i64, over: i64) -> (i64, Vec<User>) {
        if let Ok(n) = u_user.select(count(id)).first(conn) {
            let rows = u_user
                .offset(begin)
                .limit(over - begin)
                .order(create_time.desc())
                .load::<User>(conn)
                .expect("Error query user rows");
            return (n, rows);
        }
        return (0, vec![]);
    }

    pub fn get(conn: &Pool, user_id: i32) -> Option<User> {
        match u_user.filter(id.eq(user_id)).get_result::<User>(conn) {
            Ok(u) => Some(u),
            _ => None,
        }
    }

    pub fn get_user(conn: &Pool, user_v: &str) -> Option<User> {
        match u_user.filter(user.eq(user_v)).get_result::<User>(conn) {
            Ok(u) => Some(u),
            _ => None,
        }
    }
    /// Get activated user by email
    pub fn get_user_mail(conn: &Pool, email_v: &str) -> Option<User> {
        let i = UserFlag::Activated as i16;
        match u_user.filter(email.eq(email_v)).load::<User>(conn) {
            Ok(u) => u.into_iter().find(|p| p.flag & i == i),
            Err(_err) => None,
        }
    }
    pub fn save_user(conn: &Pool, v: &User) -> Result<usize, DataError> {
        if v.user.trim().len() == 0 {
            return Err(DataError::from("用户不能为空".to_string()));
        }
        if v.pwd.len() == 0 {
            return Err(DataError::from("未设置密码".to_string()));
        }
        if v.email.len() == 0 {
            return Err(DataError::from("电子邮箱不能为空".to_string()));
        }
        let mut v = v.clone();
        v.user = v.user.to_lowercase();
        if v.id <= 0 {
            v.flag = v.flag | UserFlag::Activated as i16;
            return diesel::insert_into(u_user::table)
                .values(NewUser::from(&v))
                .execute(conn)
                .map_err(From::from);
        }
        diesel::insert_into(u_user::table)
            .values(&v)
            .on_conflict(id)
            .do_update()
            .set(&v)
            .execute(conn)
            .map_err(From::from)
    }
}
