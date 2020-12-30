//! The snowflake algorithm rust version.
//!

use std::{
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// The `SnowflakeIdGenerator` type is snowflake algorithm wrapper.
#[derive(Copy, Clone, Debug)]
pub struct SnowflakeIdGenerator {
    /// last_time_millis, last time generate id is used times millis.
    last_time_millis: i64,

    /// machine_id, is use to supplement id machine or sectionalization attribute.
    machine_id: i32,

    /// node_id, is use to supplement id machine-node attribute.
    node_id: i32,

    /// auto-increment record.
    idx: u16,
}

/// The `SnowflakeIdBucket` type is snowflake-id-bucket it easy to get id also have a id buffer.
#[derive(Clone, Debug)]
pub struct SnowflakeIdBucket {
    /// Hidden the `SnowflakeIdGenerator` in bucket .
    snowflake_id_generator: SnowflakeIdGenerator,

    /// The bucket buffer;
    bucket: Vec<i64>,
}

impl SnowflakeIdGenerator {
    /// Constructs a new `SnowflakeIdGenerator`.
    /// Please make sure that machine_id and node_id is small than 32(2^5);
    ///
    /// # Examples
    ///
    /// ```
    /// use snowflake::SnowflakeIdGenerator;
    ///
    /// let id_generator = SnowflakeIdGenerator::new(1, 1);
    /// ```
    pub fn new(machine_id: i32, node_id: i32) -> SnowflakeIdGenerator {
        //TODO:limit the maximum of input args machine_id and node_id
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

    /// The real_time_generate keep id generate time is eq call method time.
    ///
    /// # Examples
    ///
    /// ```
    /// use snowflake::SnowflakeIdGenerator;
    ///
    /// let mut id_generator = SnowflakeIdGenerator::new(1, 1);
    /// id_generator.real_time_generate();
    /// ```
    pub fn real_time_generate(&mut self) -> i64 {
        self.idx = (self.idx + 1) % 4096;

        //if idx == 0 , check last_time_millis is not eq now_time_millis ,that can safe generate
        //huozhe... sleep 1 millis...

        let mut now_millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went mackward")
            .as_millis() as i64;

        //TODO:supplement code for 'clock is moving backwards situation'.

        //must use real-time millis generate.
        if now_millis == self.last_time_millis {
            //if that millis is exaust, wait....
            if self.idx == 0 {
                //TODO: Relace sleep with loop.
                sleep(Duration::from_millis(1));
                now_millis += 1;
                self.last_time_millis = now_millis;
            }
        } else {
            self.last_time_millis = now_millis;
        }

        //last_time_millis is 64 bits，left shift 22 bit，store 42 bits ， machine_id left shift 17 bits，
        // node_id left shift 12 bits ,idx complementing bits.
        self.last_time_millis << 22
            | ((self.machine_id << 17) as i64)
            | ((self.node_id << 12) as i64)
            | (self.idx as i64)
    }

    /// The basic guarantee time punctuality.
    ///
    /// # Examples
    ///
    /// ```
    /// use snowflake::SnowflakeIdGenerator;
    ///
    /// let mut id_generator = SnowflakeIdGenerator::new(1, 1);
    /// id_generator.generate();
    /// ```
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

            //TODO:supplement code for 'clock is moving backwards situation'..
            if now_millis == self.last_time_millis {
                now_millis += 1;

                //TODO: Relace sleep with loop.
                sleep(Duration::from_millis(1));
            }

            self.last_time_millis = now_millis;
        }

        //last_time_millis is 64 bits，left shift 22 bit，store 42 bits ， machine_id left shift 17 bits，
        //node_id left shift 12 bits ,idx complementing bits.
        self.last_time_millis << 22
            | ((self.machine_id << 17) as i64)
            | ((self.node_id << 12) as i64)
            | (self.idx as i64)
    }

    /// The lazy generate.
    ///
    /// # Examples
    ///
    /// ```
    /// use snowflake::SnowflakeIdGenerator;
    ///
    /// let mut id_generator = SnowflakeIdGenerator::new(1, 1);
    /// id_generator.lazy_generate();
    /// ```
    //lazy generate.
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
    /// Constructs a new `SnowflakeIdBucket`.
    /// Please make sure that machine_id and node_id is small than 32(2^5);
    ///
    /// # Examples
    ///
    /// ```
    /// use snowflake::SnowflakeIdBucket;
    ///
    /// let id_generator_bucket = SnowflakeIdBucket::new(1, 1);
    /// ```
    pub fn new(machine_id: i32, node_id: i32) -> Self {
        let snowflake_id_generator = SnowflakeIdGenerator::new(machine_id, node_id);
        let bucket = Vec::new();

        SnowflakeIdBucket {
            snowflake_id_generator,
            bucket,
        }
    }

    /// Generate id.
    ///
    /// # Examples
    ///
    /// ```
    /// use snowflake::SnowflakeIdBucket;
    ///
    /// let mut id_generator_bucket = SnowflakeIdBucket::new(1, 1);
    /// let id = id_generator_bucket.get_id();
    ///
    /// ```
    pub fn get_id(&mut self) -> i64 {
        //247 ns/iter
        // after self.bucket.push(self.snowflake_id_generator.generate());

        // 7 ns/iter
        // after self.bucket.push(self.snowflake_id_generator.lazy_generate());

        //500 ns/iter
        // after self.bucket.push(self.snowflake_id_generator.real_time_generate());
        if self.bucket.is_empty() {
            self.generate_ids();
        }
        self.bucket.pop().unwrap()
    }

    pub fn generate_ids(&mut self) {
        // 30,350 -- 50,000 ns/iter
        //self.bucket.push(self.snowflake_id_generator.lazy_generate());

        // 1,107,103 -- 1,035,018 ns/iter
        //self.bucket.push(self.snowflake_id_generator.generate());

        // 2,201,325 -- 2,082,187 ns/iter
        //self.bucket.push(self.snowflake_id_generator.real_time_generate());

        for _ in 0..4091 {
            self.bucket
                .push(self.snowflake_id_generator.lazy_generate());
        }
    }
}
