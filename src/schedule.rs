pub mod schedule {
    use std::time;
    use std::time::Duration;
    use std::ops::FnMut;

    pub trait TaskAction {
        fn execute(&mut self,id:u64) {}
        fn next_tick(&self) -> time::Duration;
        fn id(&self) -> u64;
    }

    // 内部实现的自己绑定自己函数的实现
    struct TaskFuncAction {
        date_format: String,
        id:u64,
        tick:Duration,
        call: Box<dyn FnMut(u64)>,
    }

    // 实现这个trait
    impl TaskAction for TaskFuncAction {
        fn execute(&mut self, id: u64) {

        }

        fn next_tick(&self) -> Duration {
            self.tick
        }

        fn id(&self) -> u64 {
            self.id
        }
    }

    pub struct TaskSchedule {
        id:u64,
        executor:Box<dyn TaskAction>, // 实现taskAction的trait
        next:u64, // 这个任务下一次的执行时间
        tick:time::Duration, //
        loop_count:i32,

    }

    

}