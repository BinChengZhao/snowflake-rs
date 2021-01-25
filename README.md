# snowflake-rs
Rust version of the `Twitter snowflake algorithm` .

A crate for quick generating distributed-ids. 


API Docs: https://docs.rs/rs-snowflake

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
rs-snowflake = "*"
```


## Getting Started

```rust
use snowflake::SnowflakeIdGenerator;
fn main() {
   let mut id_generator_generator = SnowflakeIdGenerator::new(1, 1);
   let id = id_generator_generator.real_time_generate();
}
```

```rust
use snowflake::SnowflakeIdBucket;
fn main() {
   let mut id_generator_bucket = SnowflakeIdBucket::new(1, 1);
   let id = id_generator_bucket.get_id();
}
```



```
test bench_generate_get_id_by_bucket                      ... bench:           7 ns/iter (+/- 0)

test bench_generate_get_id_by_generator_general_version   ... bench:         246 ns/iter (+/- 21)

test bench_generate_get_id_by_generator_lazy_version      ... bench:           4 ns/iter (+/- 0)

test bench_generate_get_id_by_generator_real_time_version ... bench:         244 ns/iter (+/- 7)

test bench_generate_ids_by_bucket                         ... bench:      30,312 ns/iter (+/- 4,696)

```

## License

Licensed under

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


### Contribution

Let me see, what do I write.
