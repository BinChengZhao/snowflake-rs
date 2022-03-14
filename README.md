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
test bench_generate_get_id_by_bucket                      ... bench:           5 ns/iter (+/- 0)

test bench_generate_get_id_by_generator_general_version   ... bench:         232 ns/iter (+/- 32)

test bench_generate_get_id_by_generator_lazy_version      ... bench:           2 ns/iter (+/- 0)

test bench_generate_get_id_by_generator_real_time_version ... bench:         249 ns/iter (+/- 22)

test bench_generate_ids_by_bucket                         ... bench:      13,077 ns/iter (+/- 1,263)

```

## License

Licensed under

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


### Contribution

Thank you all very much for your contributions to the project, and if there is anything I can do to help, I would love to help!
