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

use std::sync::{Mutex, MutexGuard};

use crate::jsa::ItemManager;

pub use self::user::User;
pub use self::user::UserFlag;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool as DPool;
use diesel::r2d2::PooledConnection;
use std::ops::Deref;
use std::sync::Arc;

pub mod config;
mod domain;
pub mod http;
mod jsa;
mod models;
mod paths;
mod schema;
mod user;

// App name
const NAME: &str = "JSA";
// App version
const VERSION: &str = "1.0";

pub struct Context {
    pub config: config::Config,
}

pub struct Pool(PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for Pool {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

lazy_static! {
    static ref CONTEXT: Arc<Mutex<Context>> = Arc::new(Mutex::new(Context {
        config: config::Config::new()
    }));
    static ref MANAGER: Mutex<Option<ItemManager>> = Mutex::new(None);
    static ref DEBUG_MODE: Mutex<bool> = Mutex::new(false);
    static ref CONF_PATH: Mutex<String> = Mutex::new(String::from("./conf"));
    static ref POOL: Mutex<Option<DPool<ConnectionManager<PgConnection>>>> = Mutex::new(None);
}

pub fn connection() -> Pool {
    let p = POOL.lock().unwrap();
    Pool(p.as_ref().unwrap().get().unwrap())
}

pub fn init(conf: &str, debug: bool) {
    *MANAGER.lock().unwrap() = Some(ItemManager::new(conf.to_owned()).unwrap());
    *DEBUG_MODE.lock().unwrap() = debug;
    *CONF_PATH.lock().unwrap() = conf.to_string();
    let mut conf_path = CONF_PATH.lock().unwrap().clone();
    conf_path.push_str("/config.toml");
    let cfg = config::read_config(&conf_path);
    *POOL.lock().unwrap() = Some(init_pool(&cfg.db_url));
    *CONTEXT.clone().lock().unwrap() = Context { config: cfg };
    domain::get_domain();
}

fn init_pool(db_url: &str) -> DPool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    DPool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
