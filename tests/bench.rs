// TODO:The default status is annotated.
// Make sure all versions can be compiled, open comments if you need a bench.
// #![feature(test)]
// extern crate test;

// use snowflake::SnowflakeIdBucket;
// use test::Bencher;

// #[bench]
// fn bench_generate(b: &mut Bencher) {
//     let mut snowflake_id_bucket = SnowflakeIdBucket::new(1, 1);
//     b.iter(|| snowflake_id_bucket.get_id());
// }

// #[bench]
// fn bench_generate_ids(b: &mut Bencher) {
//     let mut snowflake_id_bucket = SnowflakeIdBucket::new(1, 1);
//     b.iter(|| snowflake_id_bucket.generate_ids());
// }
