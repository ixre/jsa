use crate::conn;
use diesel::dsl::count;
use diesel::prelude::*;

/// The state of domain
pub enum DomainState {
    Normal = 1,
    Stopped = 2,
    Paused = 3,
}

pub enum DomainFlag {
    /// If flag contain 2,System will open statistics
    /// function for domain.
    Stat = 2,
}

#[derive(Queryable)]
#[table_name = "d_domain"]
pub struct Domain {
    pub id: i32,
    pub user_id: i32,
    pub hash: String,
    pub domain: String,
    pub flag: i32,
    pub state: i32,
    pub notes: String,
    pub create_time: i64,
}

pub fn get_domain() {
    use crate::schema::d_domain::dsl::*;
    use diesel::result::Error;
    let count: Result<i64, Error> = d_domain.select(count(id)).first(&conn());
    //assert_eq!(Ok(0), count);
}
