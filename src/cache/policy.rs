// A cache policy controls HOW eviction decisions are made.
//
// Examples:
// - LRU  -> remove least recently used
// - LFU  -> remove least frequently used
// - TTL  -> remove expired entries
//
// The cache itself stores data.
// The policy only tracks behavior.

pub trait CachePolicy {
    // Associated type:
    // The policy works with some kind of key.
    //
    // Example:
    // String
    // u64
    // UUID
    //
    // The cache decides the concrete type later.
    type Key;

    // Called whenever a new key is inserted into the cache.
    //
    // The policy may:
    // - track insertion order
    // - initialize counters
    // - start TTL tracking
    fn on_insert(&mut self, key: &Self::Key);

    // Called whenever a key is successfully accessed.
    //
    // Example:
    // cache.get("user_1")
    //
    // Policies use this to update:
    // - recency (LRU)
    // - frequency (LFU)
    fn on_access(&mut self, key: &Self::Key);

    // Called whenever a key is removed from the cache.
    //
    // IMPORTANT:
    // The policy must clean up its internal tracking state.
    //
    // Otherwise it may track keys that no longer exist.
    fn on_remove(&mut self, key: &Self::Key);

    // Ask the policy:
    //
    // "Which key should be evicted?"
    //
    // Returns:
    // - Some(key) -> evict this key
    // - None      -> nothing to evict
    //
    // The cache engine will later remove the returned key.
    fn choose_victim(&self) -> Option<Self::Key>;
}
