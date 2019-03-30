
use crate::schema::d_domain;
use crate::schema::d_domain::dsl::*;
use diesel::dsl::*;
use diesel::prelude::*;
use crate::models::domain::*;
use crate::{Pool, util};
use crate::errors::DataError;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::models::user::NewUser;

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
    pub fn get(conn:&Pool,domain_id:i32)->Option<Domain>{
        match d_domain.filter(id.eq(domain_id))
            .get_result::<Domain>(conn) {
            Ok(u) => Some(u),
            _ => None,
        }
    }

    pub fn save(conn:&Pool,v:&Domain)->Result<usize,DataError>{
        if v.domain.trim().len() == 0 {
            return Err(DataError::from("域名不能为空".to_string()));
        }
        if d_domain.filter(domain.eq(v.domain.to_owned()))
            .select(count(id)).first(conn).unwrap_or(0) != 0{
            return Err(DataError::from("域名已经添加".to_owned()));
        }
        let mut v = v.clone();
        v.domain = v.domain.to_lowercase();
        if v.id <= 0{
            v.hash = util::short_hash(v.domain.to_owned());
            v.create_time = util::unix_sec() as i32;
            return diesel::insert_into(d_domain::table)
                .values(NewDomain::from(&v))
                .execute(conn)
                .map_err(From::from);
        }
        diesel::insert_into(d_domain::table)
            .values(&v)
            .on_conflict(id)
            .do_update()
            .set(&v)
            .execute(conn)
            .map_err(From::from)
    }
}