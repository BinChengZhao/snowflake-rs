# snowflake-rs
The snowflake algorithm rust version.

A crate for quick generating distributed-ids. 


API Docs: https://docs.rs/rs-snowflake/0.3.0/snowflake/

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
rs-snowflake = "0.3.0"
```


## Getting Started

```rust
use snowflake::SnowflakeIdBucket;
fn main() {
   let mut id_generator_bucket = SnowflakeIdBucket::new(1, 1);
   let id = id_generator_bucket.get_id();
}
```
     

## License

Licensed under

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


### Contribution

Let me see, what do I write.
