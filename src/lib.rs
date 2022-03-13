//! Rust version of the `Twitter snowflake algorithm` .
//!

use std::hint::spin_loop;
use std::time::{SystemTime, UNIX_EPOCH};

/// The `SnowflakeIdGenerator` type is snowflake algorithm wrapper.
#[derive(Copy, Clone, Debug)]
pub struct SnowflakeIdGenerator {
    /// epoch used by the snowflake algorithm.
    epoch: SystemTime,

    /// last_time_millis, last time generate id is used times millis.
    last_time_millis: i64,

    /// machine_id, is use to supplement id machine or sectionalization attribute.
    pub machine_id: i32,

    /// node_id, is use to supplement id machine-node attribute.
    pub node_id: i32,

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
    /// Constructs a new `SnowflakeIdGenerator` using the UNIX epoch.
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
        Self::with_epoch(machine_id, node_id, UNIX_EPOCH)
    }

    /// Constructs a new `SnowflakeIdGenerator` using the specified epoch.
    /// Please make sure that machine_id and node_id is small than 32(2^5);
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::{Duration, UNIX_EPOCH};
    /// use snowflake::SnowflakeIdGenerator;
    ///
    /// // 1 January 2015 00:00:00
    /// let discord_epoch = UNIX_EPOCH + Duration::from_millis(1420070400000);
    /// let id_generator = SnowflakeIdGenerator::with_epoch(1, 1, discord_epoch);
    /// ```
    pub fn with_epoch(machine_id: i32, node_id: i32, epoch: SystemTime) -> SnowflakeIdGenerator {
        //TODO:limit the maximum of input args machine_id and node_id
        let last_time_millis = get_time_millis(epoch);

        SnowflakeIdGenerator {
            epoch,
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

        let mut now_millis = get_time_millis(self.epoch);

        //supplement code for 'clock is moving backwards situation'.

        // If the milliseconds of the current clock are equal to
        // the number of milliseconds of the most recently generated id,
        // then check if enough 4096 are generated,
        // if enough then busy wait until the next millisecond.
        if now_millis == self.last_time_millis {
            if self.idx == 0 {
                now_millis = biding_time_conditions(self.last_time_millis, self.epoch);
                self.last_time_millis = now_millis;
            }
        } else {
            self.last_time_millis = now_millis;
            self.idx = 0;
        }

        // last_time_millis is 64 bits，left shift 22 bit，store 42 bits ， machine_id left shift 17 bits，
        // node_id left shift 12 bits ,idx complementing bits.
        self.last_time_millis << 22
            | ((self.machine_id << 17) as i64)
            | ((self.node_id << 12) as i64)
            | (self.idx as i64)
    }

    /// The basic guarantee time punctuality.
    ///
    /// Basic guarantee time punctuality.
    /// sometimes one millis can't use up 4096 ID, the property of the ID isn't real-time.
    /// But setting time after every 4096 calls.
    /// # Examples
    ///
    /// ```
    /// use snowflake::SnowflakeIdGenerator;
    ///
    /// let mut id_generator = SnowflakeIdGenerator::new(1, 1);
    /// id_generator.generate();
    /// ```
    pub fn generate(&mut self) -> i64 {
        self.idx = (self.idx + 1) % 4096;

        // Maintenance `last_time_millis` for every 4096 ids generated.
        if self.idx == 0 {
            let mut now_millis = get_time_millis(self.epoch);

            if now_millis == self.last_time_millis {
                now_millis = biding_time_conditions(self.last_time_millis, self.epoch);
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
    /// Lazy generate.
    /// Just start time record last_time_millis it consume every millis ID.
    /// Maybe faster than standing time.
    /// # Examples
    ///
    /// ```
    /// use snowflake::SnowflakeIdGenerator;
    ///
    /// let mut id_generator = SnowflakeIdGenerator::new(1, 1);
    /// id_generator.lazy_generate();
    /// ```
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
    /// Constructs a new `SnowflakeIdBucket` using the UNIX epoch.
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
        Self::with_epoch(machine_id, node_id, UNIX_EPOCH)
    }

    /// Constructs a new `SnowflakeIdBucket` using the specified epoch.
    /// Please make sure that machine_id and node_id is small than 32(2^5);
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::{Duration, UNIX_EPOCH};
    /// use snowflake::SnowflakeIdBucket;
    ///
    /// // 1 January 2015 00:00:00
    /// let discord_epoch = UNIX_EPOCH + Duration::from_millis(1420070400000);
    /// let id_generator_bucket = SnowflakeIdBucket::with_epoch(1, 1, discord_epoch);
    /// ```
    pub fn with_epoch(machine_id: i32, node_id: i32, epoch: SystemTime) -> Self {
        let snowflake_id_generator = SnowflakeIdGenerator::with_epoch(machine_id, node_id, epoch);
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

    fn generate_ids(&mut self) {
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

#[inline(always)]
/// Get the latest milliseconds of the clock.
pub fn get_time_millis(epoch: SystemTime) -> i64 {
    SystemTime::now()
        .duration_since(epoch)
        .expect("Time went mackward")
        .as_millis() as i64
}

#[inline(always)]
// Constantly refreshing the latest milliseconds by busy waiting.
fn biding_time_conditions(last_time_millis: i64, epoch: SystemTime) -> i64 {
    let mut latest_time_millis: i64;
    loop {
        latest_time_millis = get_time_millis(epoch);
        if latest_time_millis > last_time_millis {
            return latest_time_millis;
        }
        spin_loop();
    }
}
