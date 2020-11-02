use snowflake::SnowflakeIdGenerator;

#[test]
fn test_generate() {
    let mut id_generator = SnowflakeIdGenerator::new(1, 1);
    let mut ids = vec![];

    for _ in 0..99 {
        for _ in 0..10000 {
            ids.push(id_generator.generate());
        }

        ids.sort();
        ids.dedup();

        assert_eq!(10000, ids.len());
        println!("{}", ids[9999]);

        ids.clear();
    }
}

#[test]
fn test_real_time_generate() {
    let mut id_generator = SnowflakeIdGenerator::new(1, 1);
    let mut ids = vec![];

    for _ in 0..99 {
        for _ in 0..10000 {
            ids.push(id_generator.real_time_generate());
        }

        ids.sort();
        ids.dedup();

        assert_eq!(10000, ids.len());
        println!("{}", ids[9999]);

        ids.clear();
    }
}

#[test]
fn test_lazy_generate() {
    let mut id_generator = SnowflakeIdGenerator::new(1, 1);
    let mut ids = vec![];

    for _ in 0..99 {
        for _ in 0..10000 {
            ids.push(id_generator.lazy_generate());
        }

        ids.sort();
        ids.dedup();

        assert_eq!(10000, ids.len());
        println!("{}", ids[9999]);

        ids.clear();
    }
}
