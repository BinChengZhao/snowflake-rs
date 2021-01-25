#![feature(test)]
extern crate test;

use snowflake::{SnowflakeIdBucket, SnowflakeIdGenerator};
use test::Bencher;

#[bench]
fn bench_generate_get_id_by_bucket(b: &mut Bencher) {
    let mut snowflake_id_bucket = SnowflakeIdBucket::new(1, 1);
    b.iter(|| snowflake_id_bucket.get_id());
}

#[bench]
fn bench_generate_ids_by_bucket(b: &mut Bencher) {
    let mut snowflake_id_bucket = SnowflakeIdBucket::new(1, 1);
    b.iter(|| snowflake_id_bucket.generate_ids());
}

#[bench]
fn bench_generate_get_id_by_generator_lazy_version(b: &mut Bencher) {
    let mut snowflake_id_generator = SnowflakeIdGenerator::new(1, 1);
    b.iter(|| snowflake_id_generator.lazy_generate());
}

#[bench]
fn bench_generate_get_id_by_generator_general_version(b: &mut Bencher) {
    let mut snowflake_id_generator = SnowflakeIdGenerator::new(1, 1);
    b.iter(|| snowflake_id_generator.generate());
}

#[bench]
fn bench_generate_get_id_by_generator_real_time_version(b: &mut Bencher) {
    let mut snowflake_id_generator = SnowflakeIdGenerator::new(1, 1);
    b.iter(|| snowflake_id_generator.real_time_generate());
}
