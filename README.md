# arc-cache

A generic, in-memory cache engine built in Rust. Supports multiple eviction policies (LRU, LFU, TTL) and demonstrates core Rust concepts including generics, traits, ownership, references, and lifetimes.

## Features

- Generic over any key `K` and value `V` type
- Three eviction strategies via a `CachePolicy<K>` trait:
  - **LRU** — Least Recently Used
  - **LFU** — Least Frequently Used
  - **TTL** — Time-To-Live expiry
- `get()` returns borrowed references with explicit lifetimes (`Option<&'a V>`)
- `CacheSnapshot<'a, K, V>` — scoped immutable read view into the cache
- Hit/miss/eviction statistics with a `Display` trait implementation

## Project Structure

```
arc-cache/
├── src/
│   ├── lib.rs
│   ├── cache.rs       # Generic Cache<K, V, P> struct
│   ├── policy.rs      # CachePolicy trait + LRU, LFU, TTL impls
│   ├── entry.rs       # CacheEntry<V> with TTL metadata
│   ├── snapshot.rs    # CacheSnapshot<'a, K, V> — lifetime demo
│   ├── stats.rs       # Hit/miss/eviction counters
│   └── error.rs       # CacheError enum
└── tests/
    └── integration.rs
```

## Getting Started

### Prerequisites

- Rust 1.70+ — install via [rustup.rs](https://rustup.rs)

### Build & Run

```bash
git clone https://github.com/YOUR_USERNAME/arc-cache.git
cd arc-cache
cargo build
cargo test
```

## Usage Example

```rust
use arc_cache::{Cache, policy::LruPolicy};

let mut cache: Cache<&str, u32, LruPolicy<&str>> = Cache::new(3);

cache.insert("hits", 42);
cache.insert("score", 100);

if let Some(val) = cache.get(&"hits") {
    println!("hits = {}", val);
}

println!("{}", cache.stats());
```

## Rust Concepts Demonstrated

| Concept | Where |
|---|---|
| Generics | `Cache<K, V, P>`, `CacheEntry<V>` |
| Traits | `CachePolicy<K>`, `Cacheable`, `Display` for stats |
| Ownership | Value moves on `insert()`, owned returns on `remove()` |
| References | `get()` returns `Option<&'a V>` |
| Lifetimes | `CacheSnapshot<'a, K, V>`, lifetime bounds on `get()` |

## License

MIT
