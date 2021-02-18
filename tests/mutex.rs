use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread;

#[test]
fn mutex() {
    let x = Mutex::new(0);
    let h = thread::spawn(move || {
        let mut s = x.lock().unwrap();
        *s = 1;
    });
    h.join().unwrap();
    //println!("{:#?}",x.lock().unwrap());
}

#[test]
fn arc_mutex() {
    let global = Arc::new(Mutex::new(0));
    let mut threads = vec![];
    for _i in 0..5 {
        let clone = global.clone();
        threads.push(thread::spawn(move || {
            let mut s = clone.lock().unwrap();
            *s += 1;
        }));
    }
    threads.into_iter().for_each(|t| {
        t.join();
    });
    println!("{:?}", *global.lock().unwrap())
}

#[test]
fn rw_lock() {
    let global = Arc::new(RwLock::new(0));
    let mut threads = vec![];
    for _i in 0..5 {
        let clone = global.clone();
        threads.push(thread::spawn(move || {
            let mut s = clone.write().unwrap();
            *s += 1;
        }));
    }
    threads.into_iter().for_each(|t| {
        t.join();
    });
    println!("{:?}", *global.read().unwrap())
}

#[test]
fn test_die_lock() {
    let x = Arc::new(Mutex::new(0));
    let clone = (x.clone(), x.clone());
    {
        let mut v = clone.0.lock().unwrap();
        *v += 1;
    }
    clone.1.lock();
}
