#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(never_type)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::sync::{Mutex, MutexGuard};

use crate::jsa::ItemManager;

pub use self::models::user::{User,UserFlag};
use crate::models::domain;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool as DPool;
use diesel::r2d2::PooledConnection;

pub mod config;
pub mod http;
mod jsa;
mod models;
mod paths;
pub mod repo;
mod schema;
mod user2;
pub mod util;
pub mod errors;

// App name
const NAME: &str = "JSA";
// App version
const VERSION: &str = "1.0";

type PgPool = DPool<ConnectionManager<PgConnection>>;
type Pool = PooledConnection<ConnectionManager<PgConnection>>;

pub fn conn() -> Pool {
    let p = POOL.lock().unwrap();
    p.as_ref().unwrap().get().unwrap()
}

lazy_static! {
    static ref MANAGER: Mutex<Option<ItemManager>> = Mutex::new(None);
    static ref DEBUG_MODE: Mutex<bool> = Mutex::new(false);
    static ref CONF_PATH: Mutex<String> = Mutex::new(String::from("./conf"));
    static ref POOL: Mutex<Option<PgPool>> = Mutex::new(None);
}

pub fn init(conf: &str, debug: bool) {
    *MANAGER.lock().unwrap() = Some(ItemManager::new(conf.to_owned()).unwrap());
    *DEBUG_MODE.lock().unwrap() = debug;
    *CONF_PATH.lock().unwrap() = conf.to_string();
    let mut conf_path = CONF_PATH.lock().unwrap().clone();
    conf_path.push_str("/config.toml");
    let cfg = config::read_config(&conf_path);
    *POOL.lock().unwrap() = Some(init_pool(&cfg.db_url));
    repo::init_data();
}

fn init_pool(db_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    DPool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
