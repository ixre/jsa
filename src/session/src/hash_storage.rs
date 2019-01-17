use core::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

use super::Key;
use super::SessionPair;
use super::SessionStore;

type Store<K, V> = RwLock<HashMap<K, RwLock<V>>>;

/// A default implementation of `SessionStore`.
///
/// Session store implemented as a read-write-locked `HashMap`.
///
/// #### To use:
/// ```ignore
/// ```
pub struct HashSessionStore<K: Key> {
    store: Arc<Store<K, K::Value>>
}

impl<K: Key> Clone for HashSessionStore<K> {
    fn clone(&self) -> HashSessionStore<K> {
        HashSessionStore {
            store: self.store.clone()
        }
    }
}

impl<K: Key> HashSessionStore<K> where K: Eq + Hash {
    /// Create a new instance of the session store
    pub fn new() -> HashSessionStore<K> {
        HashSessionStore {
            store: Arc::new(RwLock::new(HashMap::<K, RwLock<K::Value>>::new()))
        }
    }
}


/* A note on clones:
 *
 * Those values hidden behind a RwLock are owned behind that lock.
 * In order for them to be accessed, a reference to the two gating locks
 * (the HashMap and the keyed V) must be kept alive.
 *
 * Instead, all values returned are copies.
 */
impl<K: Key> SessionStore<K> for HashSessionStore<K> where K: Send + Sync + Eq + Hash + Clone, K::Value: Send + Sync + Clone {
    fn insert(&self, key: &K, value: K::Value) {
        // Avoid a WriteLock if possible
        if !self.store.read().unwrap().contains_key(key) {
            // Inserting consumes a key => clone()
            self.store.write().unwrap().insert(key.clone(), RwLock::new(value));
        }
    }
    fn get(&self, key: &K) -> Option<K::Value> {
        match self.store.read().unwrap().get(key) {
            Some(lock) => Some(lock.read().unwrap().clone()),
            None => None
        }
    }

    fn set(&self, key: &K, value: K::Value) -> Option<K::Value> {
        return match self.store.read().unwrap().get(key) {
            // Instead of using swap, which requires a write lock on the HashMap,
            // only take the write locks when the key does not yet exist
            Some(lock) => {
                let old_v = lock.read().unwrap().clone();
                *lock.write().unwrap() = value;
                Some(old_v)
            }
            None => {
                // Inserting consumes a key => clone()
                self.store.write().unwrap().insert(key.clone(), RwLock::new(value));
                None
            }
        };
    }
    fn remove(&self, key: &K) -> bool {
        self.store.write().unwrap().remove(key).is_some()
    }
}