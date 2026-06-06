use std::hash::Hash;
use std::time::{ Duration, SystemTime };

pub struct Entry<K, V> {
    key: K,
    value: V,

    created_at: SystemTime,
    last_accessed: SystemTime,

    ttl: Duration,

    access_count: u32,
}

impl<K: Hash + Eq, V> Entry<K, V> {
    pub fn new(key: K, value: V, ttl: Duration) -> Self {
        let now = SystemTime::now();

        Self {
            key,
            value,

            created_at: now,
            last_accessed: now,

            ttl,

            access_count: 0,
        }
    }
}
