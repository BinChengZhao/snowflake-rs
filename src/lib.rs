#![feature(test)]
use std::{
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
extern crate test;
//frist for generate id.
#[derive(Copy, Clone)]
pub struct SnowflakeIdGenerator {
    last_time_millis: i64,
    machine_id: i32,
    node_id: i32,
    idx: u16,
}

//second for batch generate id;
#[derive(Clone)]
pub struct SnowflakeIdBucket {
    snowflake_id_generator: SnowflakeIdGenerator,
    bucket: Vec<i64>,
}

impl SnowflakeIdGenerator {
    pub fn new(machine_id: i32, node_id: i32) -> SnowflakeIdGenerator {
        let last_time_millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went mackward")
            .as_millis() as i64;

        SnowflakeIdGenerator {
            last_time_millis,
            machine_id,
            node_id,
            idx: 0,
        }
    }

    //the real_time_generate...
    pub fn real_time_generate(&mut self) -> i64 {
        self.idx = (self.idx + 1) % 4096;

        //if idx == 0 , check last_time_millis is not eq now_time_millis ,that can safe generate
        //huozhe... sleep 1 millis...

        let mut now_millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went mackward")
            .as_millis() as i64;

        //must use real-time millis generate.
        if now_millis == self.last_time_millis {
            //if that millis is exaust, wait....
            if self.idx == 0 {
                sleep(Duration::from_millis(1));
                now_millis += 1;
                self.last_time_millis = now_millis;
            }
        }

        // 64 位，统一左移动 22位，保存后42位 ， xxxx， 最后自增保留12位
        //然后进行或运算
        self.last_time_millis << 22
            | ((self.machine_id << 17) as i64)
            | ((self.node_id << 12) as i64)
            | (self.idx as i64)
    }

    //basic guarantee time punctuality.
    //sometimes one millis can't use up 4096 ID, the property of the ID isn't real-time.
    //But setting time after every 4096 calls.
    pub fn generate(&mut self) -> i64 {
        self.idx = (self.idx + 1) % 4096;

        //if idx == 0 , check last_time_millis is not eq now_time_millis ,that can safe generate
        //huozhe... sleep 1 millis...

        if self.idx == 0 {
            let mut now_millis = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went mackward")
                .as_millis() as i64;

            if now_millis == self.last_time_millis {
                now_millis += 1;
                sleep(Duration::from_millis(1));
            }

            self.last_time_millis = now_millis;
        }

        // 64 位，统一左移动 22位，保存后42位 ， xxxx， 最后自增保留12位
        //然后进行或运算
        self.last_time_millis << 22
            | ((self.machine_id << 17) as i64)
            | ((self.node_id << 12) as i64)
            | (self.idx as i64)
    }

    //lazy generator.
    //Just start time record last_time_millis it consume every millis ID.
    //maybe faster than standing time.
    pub fn lazy_generate(&mut self) -> i64 {
        self.idx = (self.idx + 1) % 4096;

        if self.idx == 0 {
            self.last_time_millis += 1;
        }

        self.last_time_millis << 22
            | ((self.machine_id << 17) as i64)
            | ((self.node_id << 12) as i64)
            | (self.idx as i64)
    }
}

impl SnowflakeIdBucket {
    pub fn new(machine_id: i32, node_id: i32) -> Self {
        let snowflake_id_generator = SnowflakeIdGenerator::new(machine_id, node_id);
        let bucket = Vec::new();

        SnowflakeIdBucket {
            snowflake_id_generator,
            bucket,
        }
    }

    pub fn get_id(&mut self) -> i64 {
        //247 ns/iter
        // after self.bucket.push(self.snowflake_id_generator.generate());

        // 7 ns/iter
        // after self.bucket.push(self.snowflake_id_generator.lazy_generate());

        //45 ns/iter
        // after self.bucket.push(self.snowflake_id_generator.real_time_generate());
        if self.bucket.is_empty() {
            self.generate_ids();
        }
        self.bucket.pop().unwrap()
    }

    pub(crate) fn generate_ids(&mut self) {
        for _ in 0..4095 {

            // 30,350 -- 50,000 ns/iter
            //self.bucket.push(self.snowflake_id_generator.lazy_generate());
            
            // 1,107,103 -- 1,035,018 ns/iter
            //self.bucket.push(self.snowflake_id_generator.generate());

            // 408,433 -- 407,926 ns/iter
            //self.bucket.push(self.snowflake_id_generator.real_time_generate());

            self.bucket
                .push(self.snowflake_id_generator.lazy_generate());
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{SnowflakeIdBucket, SnowflakeIdGenerator};
    use test::Bencher;

    #[test]
    fn test_generate() {
        let mut idgen = SnowflakeIdGenerator::new(1, 1);
        let mut ids = vec![];

        for _ in 0..10 {
            ids.push(idgen.generate());
        }

        for id in ids {
            println!("id: {}", id);
            assert!(format!("{}", id).len() >= 18);
        }
    }

    #[bench]
    fn bench_generate(b: &mut Bencher) {
        let mut snowflake_id_bucket = SnowflakeIdBucket::new(1, 1);
        b.iter(|| snowflake_id_bucket.get_id());
    }

    #[bench]
    fn bench_generate_ids(b: &mut Bencher) {
        let mut snowflake_id_bucket = SnowflakeIdBucket::new(1, 1);
        b.iter(|| snowflake_id_bucket.generate_ids());
    }
}
