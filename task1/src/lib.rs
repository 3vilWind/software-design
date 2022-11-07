use std::collections::HashMap;
use std::hash::Hash;

use linked_list::{Link, LinkedList};

mod linked_list;

struct KeyValue<K, T> {
    key: K,
    value: T,
}

pub struct LRUCache<K, T>
    where K: Hash + Eq + Copy {
    order: LinkedList<KeyValue<K, T>>,
    map: HashMap<K, Link<KeyValue<K, T>>>,
    max_size: usize,
}

impl<K, T> LRUCache<K, T>
    where K: Hash + Eq + Copy {
    pub fn new(size: usize) -> Self {
        assert_ne!(size, 0, "LRUCache can't be zero-size");

        LRUCache {
            order: LinkedList::new(),
            map: HashMap::with_capacity(size),
            max_size: size,
        }
    }

    pub fn get_or_compute(&mut self, key: K, compute: fn() -> T) -> &T {
        if let Some(value) = self.access(&key) {
            unsafe {
                return &value.as_ref().value.value;
            }
        }

        if self.size() >= self.max_size() {
            self.map.remove(&self.order.pop_right().unwrap().key).unwrap();
        }

        let kv = KeyValue { key: key.clone(), value: compute() };
        let value = self.order.push_left(kv);
        self.map.insert(key, value);
        unsafe {
            &value.as_ref().value.value
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&T> {
        self.access(key).map(|value| {
            unsafe {
                &value.as_ref().value.value
            }
        })
    }

    pub fn max_size(&self) -> usize {
        self.max_size
    }

    pub fn size(&self) -> usize {
        let size = self.map.len();
        assert!(size <= self.max_size, "size > max_size");
        size
    }

    fn access(&mut self, key: &K) -> Option<Link<KeyValue<K, T>>> {
        // it can be just get function,
        // but NLL still not fully implemented in rust
        unsafe {
            self.map.get(&key).map(|value| {
                self.order.move_to_left(value.clone());
                value.clone()
            })
        }
    }
}
