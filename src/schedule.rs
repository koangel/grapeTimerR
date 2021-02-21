pub mod schedule {
    use std::time;
    use std::time::Duration;
    use std::ops::FnMut;
    use std::borrow::{Borrow, BorrowMut};
    use crate::errors::errors::TError;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicI64, AtomicI32};

    pub trait TaskAction : Send + Sync {
        fn execute(&self,id:u64) {}
        fn date_format(&self) -> &str;
        fn tick(&self) -> u64;
        fn id(&self) -> u64;
        fn loop_count(&self) -> i32;
    }

    struct TaskEmpty {}
    impl TaskAction for TaskEmpty {
        fn date_format(&self) -> &str {
            ""
        }

        fn tick(&self) -> u64 {
            0
        }

        fn id(&self) -> u64 {
            0
        }

        fn loop_count(&self) -> i32 {
            0
        }
    }

    // 内部实现的自己绑定自己函数的实现
    pub struct ClosuresAction {
        date_format: String,
        id:u64,
        loop_count:i32,
        tick:Duration,
        call:Arc<dyn Fn(u64) + Send + Sync + 'static>,
    }

    impl ClosuresAction {
        pub fn new(date:&str,idx:u64,loopC:i32,t:time::Duration,f: impl Fn(u64) + Send+Sync + 'static) -> ClosuresAction {
            ClosuresAction {
                date_format: String::from(date),
                id: idx,
                tick:t,
                loop_count: loopC,
                call:Arc::new(f),
            }
        }
    }

    // 实现这个trait
    impl TaskAction for ClosuresAction {
        fn execute(&self,id:u64) {
            (self.call)(id);
        }

        fn date_format(&self) -> &str {
            self.date_format.as_str()
        }
        fn tick(&self) -> u64 { self.tick.as_millis() as u64 }
        fn id(&self) -> u64 {
            self.id
        }
        fn loop_count(&self) -> i32 {
            self.loop_count
        }
    }
}