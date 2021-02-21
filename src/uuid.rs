pub mod uuid {
    use lazy_static::lazy_static;
    use std::{ sync::Arc,thread };
    use std::sync::{atomic::AtomicI64,atomic::AtomicI16,atomic::Ordering  };
    use chrono::{ Local,DateTime };

    #[derive(Debug, Clone,Copy)]
    pub enum IDMode {
        SequenceId,
        TimestampId,
    }

    lazy_static! {
        static ref LARGEID:Arc<AtomicI64> = Arc::new(AtomicI64::new(1));
    }

    // 使用第一种，序列ID
    pub fn set_seed(seed:i64) {
        LARGEID.store(seed,Ordering::Relaxed);
    }

    // 获得下一个ID
    pub fn next_big_id() -> i64 {
        let rvalue = LARGEID.load(Ordering::Relaxed);
        LARGEID.fetch_add(1,Ordering::SeqCst);
        rvalue
    }

    // 使用第二种，时间戳Id
    lazy_static! {
        static ref TIMESTAPID:Arc<AtomicI16> = Arc::new(AtomicI16::new(1));
    }

    pub fn next_timestamp_id() -> i64 {
        TIMESTAPID.fetch_add(1,Ordering::SeqCst);
        let mut ids = TIMESTAPID.load(Ordering::Relaxed);
        if ids >= 99 {
            TIMESTAPID.store(1,Ordering::Relaxed);
            ids = 1
        }

        (Local::now().timestamp() * 100) + (ids as i64)
    }

    // 测试用例
    #[test]
    fn test_set_seed() {
        set_seed(100);
        let mut hv = vec![];
        for i in 1..10 {
            let h = thread::spawn(|| {
                for i in 1..5 {
                    let next = next_big_id();
                    println!("{}",next);
                }
            });
            hv.push(h);
        }

        for tv in hv {
            tv.join();
        }
    }

    #[test]
    fn test_next_timestamp() {
        let mut hv = vec![];
        for i in 1..10 {
            let h = thread::spawn(|| {
                for i in 1..5 {
                    let next = next_timestamp_id();
                    println!("{}",next);
                }
            });
            hv.push(h);
        }

        for tv in hv {
            tv.join();
        }
    }
}