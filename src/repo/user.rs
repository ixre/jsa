use diesel::dsl::*;
use diesel::prelude::*;

use crate::errors::DataError;
use crate::models::user::{User, NewUser};
use crate::{Pool, UserFlag};
use crate::schema::u_user::dsl::*;
use crate::schema::u_user;

pub struct UserRepo{}
impl UserRepo {
    pub fn take_users(conn:&Pool,begin: i64, over: i64) -> (i64, Vec<User>) {
        if let Ok(n) = u_user.select(count(id)).first(conn){
            let rows = u_user.offset(begin)
                .limit(over - begin)
                .load::<User>(conn)
                .expect("Error query user rows");
            return (n,rows);
        }
        return (0,vec![]);
    }

    pub fn get_user(conn:&Pool,user_v: &str) -> Option<User> {
        match u_user.filter(user.eq(user_v))
            .get_result::<User>(conn) {
            Ok(u) => Some(u),
            Err(err) => None
        }
    }
    /// Get activated user by email
    pub fn get_user_mail(conn:&Pool,email_v: &str) -> Option<User> {
        let i = UserFlag::Activated as i16;
        match u_user.filter(email.eq(email_v))
            .load::<User>(conn) {
            Ok(u) =>{
                u.into_iter().find(|p|p.flag & i == i)
            }
            Err(err) => None
        }
    }
    pub fn save_user(conn:&Pool,user_v: &User) -> Result<usize,DataError> {
        if user_v.user.trim().len() == 0 {
            return Err(DataError::from("用户不能为空".to_string()));
        }
        if user_v.pwd.len() == 0 {
            return Err(DataError::from("未设置密码".to_string()));
        }
        if user_v.email.len() == 0 {
            return Err(DataError::from("电子邮箱不能为空".to_string()));
        }

        let iu = NewUser::from(user_v);
        diesel::insert_into(u_user::table)
            .values(user_v)
            .on_conflict(id)
            .do_update()
            .set(user_v)
            .execute(conn).map_err(From::from)

//        if user_v.id <= 0{
//            let iu = NewUser::from(user_v);
//            return diesel::insert_into(u_user::table)
//                .values(&iu)
//                .execute(conn).map_err(From::from)
//        }
//        diesel::update(u_user.filter(id.eq(user_v.id)))
//            .set(*user_v)
//            .conflict(id)
//            .execute(conn)
//            .map_err(From::from)
    }
}
