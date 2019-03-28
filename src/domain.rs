use crate::connection;
use crate::Pool;
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

pub fn get_domain() {
    use super::schema::domains::dsl::*;
    let d = domains.select(id).filter(id.ne(0));
    //assert_eq!(Ok(2), d.first(&connection()));

    //ctx.pool;
    /*
    let pool = ctx.pool.unwrap();
    let pooled_conn: DB = DB(pool.get().unwrap());
    let conn = pooled_conn.conn();
    //domains::
    // let d : Domain = schema::domains::
    //let d :Domain = schema::domains:

    let d = domains.select(id).filter(id.ne(0));
    assert_eq!(Ok(2), d.first(conn));*/
}
