use crate::Pool;
use crate::models::user::User;
use crate::schema::u_user::dsl::*;
use diesel::prelude::*;
use diesel::dsl::*;

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

}
