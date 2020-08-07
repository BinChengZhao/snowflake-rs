use std::time::{SystemTime, UNIX_EPOCH};

//frist for generate
#[derive(Copy, Clone)]
pub struct SnowflakeIdGenerator {
    last_time_millis: i64,
    machine_id: i32,
    node_id: i32,
    idx: u16,
}

//second for batch
#[derive(Copy, Clone)]
pub struct SnowflakeIdBucket {
    snowflake_id_generator: SnowflakeIdGenerator,
    bucket: [u64; 4096],
}

//TODO:写好程序后-依赖方将生成函数内联到程序内部
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

    pub fn generate(&mut self) -> i64 {
        self.idx = (self.idx + 1) % 4096;

        //if idx == 0 , check last_time_millis is not eq now_time_millis ,that can safe generate
        //huozhe... sleep 1 millis...

        if self.idx == 0 {
            let now_millis = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went mackward")
                .as_millis() as i64;

            if now_millis == self.last_time_millis {
                //sleep
            }

            self.last_time_millis = now_millis;
        }

        // a SnowFlake style

        //https://www.cnblogs.com/jiangxinlingdu/p/8440413.html
        //what's problem...
        // 64 位，统一左移动 22位，保存后42位 ， xxxx， 最后自增保留12位
        //然后进行或运算
        self.last_time_millis << 22
            | ((self.machine_id << 17) as i64)
            | ((self.node_id << 12) as i64)
            | (self.idx as i64)
    }
}


impl SnowflakeIdBucket {
    pub fn get_id(&mut self) -> u64 {
        todo!();
    }

    fn generate_ids(&mut self) {

        //inline ...
        //push bucket
        todo!();
    }
}

#[cfg(test)]
mod test {

    use super::SnowflakeIdGenerator;

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
}
