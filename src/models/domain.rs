use crate::conn;
use diesel::dsl::count;
use diesel::prelude::*;
use crate::schema::d_domain;

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

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "d_domain"]
#[primary_key("id")]
#[changeset_for(d_domain)]
pub struct Domain {
    pub id: i32,
    pub user_id: i32,
    pub hash: String,
    pub domain: String,
    pub flag: i16,
    pub state: i16,
    pub notes: String,
    pub create_time: i32,
}

pub fn get_domain() {
    use crate::schema::d_domain::dsl::*;
    use diesel::result::Error;
    let _count: Result<i64, Error> = d_domain.select(count(id)).first(&conn());
    //assert_eq!(Ok(0), count);
}
