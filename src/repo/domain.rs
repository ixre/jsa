
use std::hash::{Hash};

use diesel::dsl::*;
use diesel::prelude::*;

use crate::errors::DataError;
use crate::models::domain::*;

use crate::schema::d_domain;
use crate::schema::d_domain::dsl::*;
use crate::{util, Pool};

pub struct DomainRepo {}

impl DomainRepo {
    pub fn take_domains(conn: &Pool, uid: i32, begin: i64, over: i64) -> (i64, Vec<Domain>) {
        if let Ok(n) = d_domain
            .select(count(id))
            .filter(user_id.eq(uid))
            .first(conn)
        {
            let rows = d_domain
                .filter(user_id.eq(uid))
                .offset(begin)
                .limit(over - begin)
                .order(create_time.desc())
                .load::<Domain>(conn)
                .expect("Error query domain rows");
            return (n, rows);
        }
        return (0, vec![]);
    }
    /// Gets domain by domain_id
    pub fn get(conn: &Pool, domain_id: i32) -> Option<Domain> {
        match d_domain.filter(id.eq(domain_id)).get_result::<Domain>(conn) {
            Ok(u) => Some(u),
            _ => None,
        }
    }

    /// Gets domain by hash string
    pub fn get_domain(conn: &Pool, hash_str: String) -> Option<Domain> {
        match d_domain
            .filter(hash.eq(hash_str))
            .get_result::<Domain>(conn)
        {
            Ok(u) => Some(u),
            _ => None,
        }
    }

    pub fn save(conn: &Pool, v: &Domain) -> Result<usize, DataError> {
        if v.domain.trim().len() == 0 {
            return Err(DataError::from("域名不能为空".to_string()));
        }
        if d_domain
            .filter(domain.eq(v.domain.to_owned()))
            .filter(id.ne(v.id))
            .select(count(id))
            .first(conn)
            .unwrap_or(0)
            != 0
        {
            return Err(DataError::from("域名已经添加".to_owned()));
        }
        let mut v = v.clone();
        v.domain = v.domain.to_lowercase();
        if v.id <= 0 {
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
