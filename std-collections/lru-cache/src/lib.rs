#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    mp: BTreeMap<usize, V>,
    hash: HashMap<K, usize>,
    cap: usize,
    count: usize,
} 

impl<K: Clone + Hash + Ord, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        if capacity <= 0 {
            panic!("");
        }

        Self {
            mp: BTreeMap::<usize, V>::new(),
            hash: HashMap::<K, usize>::new(),
            cap: capacity,
            count: 0,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        match self.hash.get(key) {
            Some(idx) => {
                match self.mp.get(idx) {
                    Some(val) => {
                        let old_order = self.hash.remove(key).unwrap();
                        let val = self.mp.remove(&old_order).unwrap();
                        self.mp.insert(self.count, val);
                        self.hash.insert(key.clone(), self.count);
                        self.count += 1;
                        self.mp.get(&(self.count-1))
                    },
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.hash.contains_key(&key) {
            true => {
                // println!("Contains");
                match self.get(&key) {
                    Some(&ref v_) => {
                        return self.mp.insert(self.count-1, value);
                    },
                    None => {}
                }
            },
            false => {}
        }
        if self.mp.len() >= self.cap {
            self.mp.pop_first();

        }
        let save_count = self.count;
        self.count += 1;
        self.hash.insert(key, save_count);
        self.mp.insert(save_count, value);
        None
    }
}
