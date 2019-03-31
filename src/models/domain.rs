use crate::conn;
use crate::schema::d_domain;
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

#[derive(Queryable, Insertable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "d_domain"]
#[primary_key("id")]
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

#[derive(Insertable)]
#[table_name = "d_domain"]
pub struct NewDomain {
    pub user_id: i32,
    pub hash: String,
    pub domain: String,
    pub flag: i16,
    pub state: i16,
    pub notes: String,
    pub create_time: i32,
}
impl From<&Domain> for NewDomain {
    fn from(src: &Domain) -> Self {
        Self {
            user_id: src.user_id,
            hash: src.hash.clone(),
            domain: src.domain.clone(),
            flag: src.flag,
            state: src.state,
            notes: src.notes.clone(),
            create_time: src.create_time,
        }
    }
}

pub fn get_domain() {
    use crate::schema::d_domain::dsl::*;
    use diesel::result::Error;
    let _count: Result<i64, Error> = d_domain.select(count(id)).first(&conn());
    //assert_eq!(Ok(0), count);
}
