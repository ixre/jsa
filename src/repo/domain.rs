
use crate::schema::d_domain::dsl::*;
use diesel::dsl::*;
use diesel::prelude::*;
use crate::models::domain::*;
use crate::Pool;

pub struct DomainRepo {}
impl DomainRepo {
    pub fn take_domains(conn: &Pool, begin: i64, over: i64) -> (i64, Vec<Domain>) {
        if let Ok(n) = d_domain.select(count(id)).first(conn) {
            let rows = d_domain
                .offset(begin)
                .limit(over - begin)
                .order(create_time.desc())
                .load::<Domain>(conn)
                .expect("Error query domain rows");
            return (n, rows);
        }
        return (0, vec![]);
    }
}