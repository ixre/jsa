#![feature(proc_macro_hygiene, decl_macro)]
#![feature(never_type)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use crate::jsa::ItemManager;

pub mod http;
mod jsa;

const VERSION: &str = "1.0";
static mut MANAGER: Option<ItemManager> = None;
static mut DEBUG_MODE: bool = false;

pub fn init(conf: &str, debug: bool) {
    unsafe {
        DEBUG_MODE = debug;
        MANAGER = Some(ItemManager::new(conf.to_owned()).unwrap());
    }
}
