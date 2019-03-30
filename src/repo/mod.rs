use std::io;
use std::path::Path;

use diesel::dsl::*;
use diesel::prelude::*;
use diesel_migrations::run_pending_migrations_in_directory;

use crate::models::user::{NewUser, UserFlag};
use crate::{conn, util, Pool};

pub use self::user::*;
pub use self::domain::*;
mod user;
mod domain;

/// Check database,if no data initialize somethings
pub fn init_data() {
    let conn = conn();
    match run_pending_migrations_in_directory(&conn, Path::new("./migrations"), &mut io::stdout()) {
        Ok(_) => println!("run migrations successfully!"),
        Err(err) => panic!("check init data error {:#?}", err),
    }
    use crate::schema::u_user::dsl::*;
    let init = |r: Result<i64, diesel::result::Error>, p: fn(conn: &Pool)| match r {
        Ok(r) => {
            if r == 0 {
                p(&conn);
            }
        }
        Err(err) => panic!("check init data error {:#?}", err),
    };
    // initialize users
    init(u_user.select(count(id)).first(&conn), |conn| {
        let u = NewUser {
            user: "root".to_string(),
            name: "root".to_string(),
            pwd: util::pwd("123456"),
            flag: UserFlag::Enabled as i16
                | UserFlag::Activated as i16
                | UserFlag::SuperUser as i16,
            email: "".to_string(),
            create_time: util::unix_sec() as i32,
        };
        use crate::schema::u_user;
        diesel::insert_into(u_user::table)
            .values(&u)
            .execute(conn)
            .expect("Error insert user");
    });
}
