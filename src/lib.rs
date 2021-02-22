pub mod schedule;
pub mod parsers;
pub mod errors;
mod thread;
mod uuid;

pub use crate::uuid::uuid::IDMode;

pub mod timer {
    use std::sync::{Mutex,Arc};
    use std::time;
    use crate::schedule::schedule;
    use crate::schedule::schedule::{TaskAction, ClosuresAction};
    use crate::errors::errors::{TResult, TError, TErrorKind};
    use crate::IDMode;
    use lazy_static::*;
    use crate::thread::threads::{TaskPool};
    use crate::uuid::uuid::{set_seed, next_timestamp_id, next_big_id};
    use simple_log;
    use simple_log::LogConfigBuilder;

    #[derive(Clone)]
    struct InterData {
        config:Arc<Mutex<Config>>,
        thread_pool:Arc<Mutex<TaskPool>>,
    }

    #[derive(Clone)]
    pub struct Config {
        pub debug:bool,
        pub debug_log:String,
        pub thread_count:i32,
        pub id_seed:i64, // 起始ID
        pub id_type:IDMode,
    }

    lazy_static! {
        static ref Inter:InterData = InterData{
           config:Arc::new(Mutex::new(Config{
                debug:false,
                debug_log:String::new(),
                thread_count:4,
                id_seed:1, // 起始ID
                id_type:IDMode::SequenceId,
           })),
            thread_pool:Arc::new(Mutex::new(TaskPool::new(time::Duration::from_secs(1),num_cpus::get())))
         };
    }

    fn next_uuid() -> u64 {
        match Inter.config.lock().unwrap().id_type {
            IDMode::SequenceId => { next_big_id() as u64 }
            IDMode::TimestampId => { next_timestamp_id() as u64 }
        }
    }

    /// init schedule system [用于初始化调度系统，通过Config]
    ///
    /// # Examples
    ///
    /// ```
    /// use grapeTimerR::{timer::Config,IDMode, timer};
    ///
    /// let conf = Config{
    ///         // output log info
    ///         debug: false,
    ///         debug_log:String::from("logs/grapeTimer.log"),
    ///         thread_count: 10,
    ///         // 初始化全局ID的起始ID，可以自行控制
    ///         // Initialize the starting ID of the global ID, which can be controlled by yourself
    ///         id_seed: 1,
    ///         id_type: IDMode::SequenceId
    ///     };
    ///
    /// timer::init_schedule(conf);
    /// ```
    pub fn init_schedule(conf:Config) -> TResult<()> {
        let mut l_config = Inter.config.lock().unwrap();
        l_config.thread_count = conf.thread_count;
        l_config.id_seed = conf.id_seed;
        l_config.debug = conf.debug;
        l_config.id_type = conf.id_type;

        let config = LogConfigBuilder::builder()
            .path(conf.debug_log)
            .size(1 * 100)
            .roll_count(10)
            .level("debug")
            .output_file()
            .output_console()
            .build();
        let r = simple_log::new(config);
        match r {
            Err(e) => {
                return Err(TError::new(TErrorKind::Other(e)));
            }
            Ok(v) => {}
        }

        Inter.thread_pool.lock().unwrap().rebuild(conf.thread_count,conf.debug);
        set_seed(l_config.id_seed);
        Ok(())
    }

    /// create a new ticker action [创建一个计时器任务]
    ///
    /// # Examples
    ///
    /// ```
    /// use grapeTimerR::timer;
    /// use std::time;
    ///
    /// fn executer_task(id:u64) {
    ///     println!("on function mode:{}",chrono::Local::now().to_rfc2822());
    /// }
    /// // 使用函数方式执行代码 Use function to execute code
    ///  timer::spwan_ticker(time::Duration::from_millis(5000),2,executer_task);
    ///  // 使用闭包模式 Use closure function
    ///  timer::spwan_ticker(time::Duration::from_millis(5000),2,|x| {
    ///         println!("on ticker:{}",chrono::Local::now().to_rfc2822());
    ///     });
    /// ```
    pub fn spwan_ticker(tick:time::Duration, loopCount:i32, f: impl Fn(u64) + Send+Sync + 'static) -> TResult<u64> {
        let task_action =  ClosuresAction::new("", next_uuid(), loopCount, tick, f);
        let r = Inter.thread_pool.lock();
        match r {
            Err(e) => { Err(TError::new(TErrorKind::Other(e.to_string()))) },
            Ok(mut v) => {
                let task_id = task_action.id();
                v.spwan(Arc::new(task_action));
                Ok(task_id)
            }
        }
    }

    /// create a new trait ticker action [创建一个Trait模式计时器任务]
    ///
    /// # Examples
    ///
    /// ```
    /// use grapeTimerR::schedule::schedule::TaskAction;
    /// use std::sync::Arc;
    ///
    ///
    /// struct ExempleAction {}
    ///
    /// // 首先我们定义一个结构体
    /// //First we define a struct
    /// impl TaskAction for ExempleAction {
    ///     // 实际执行的代码段
    ///     // Code snippet executed
    ///     fn execute(&self, id: u64) {
    ///         println!("on trait struct:{}",chrono::Local::now().to_rfc2822());
    ///     }
    ///
    ///     // 不使用的话，返回一个空字符串
    ///     // If not used, return an empty string,like ""
    ///     fn date_format(&self) -> &str {
    ///         return ""
    ///     }
    ///
    ///     // 如果你不使用date_format，就必须使用这个参数，否则异常。
    ///     // If you don't use date_format, you must use this parameter, otherwise it is panic.
    ///     // 时间单位 毫秒 ，time unit is millisecond
    ///     fn tick(&self) -> u64 {
    ///         return 5000;
    ///     }
    ///
    ///     // 这里需要自定义ID或将其设置为一个组的ID，所以停止任务会停止这个组
    ///     // Here you need to customize the ID or set it to the GroupId or TaskType Id,
    ///     // so stopping the task will stop the group
    ///     fn id(&self) -> u64 {
    ///         return 18888;
    ///     }
    ///
    ///     // 循环的次数
    ///     fn loop_count(&self) -> i32 {
    ///        return 15;
    ///     }
    /// }
    ///    // 使用trait任务，可以简化部分实际逻辑
    ///    // Using trait tasks can simplify part of the actual logic
    ///    timer::spwan_trait(Arc::new(ExempleAction{}));
    /// ```
    pub fn spwan_trait(ft:Arc<dyn TaskAction>) -> TResult<u64> {
        let r = Inter.thread_pool.lock();
        match r {
            Err(e) => { Err(TError::new(TErrorKind::Other(e.to_string()))) },
            Ok(mut v) => {
                let taskId = ft.id();
                v.spwan(ft);
                Ok(taskId)
            }
        }
    }

    /// create a new ticker action [创建一个计时器任务]
    ///
    /// # Examples
    ///
    /// ```
    /// use grapeTimerR::timer;
    /// timer::spwan_date("day 19:30:00",1,|id| {
    ///        println!("on date:{}",chrono::Local::now().to_rfc2822());
    /// });
    /// ```
    pub fn spwan_date(dateformate:&str, loopCount:i32, f: impl Fn(u64) + Send+Sync + 'static) -> TResult<u64> {
        let task_action =  ClosuresAction::new(dateformate, next_uuid(), loopCount, time::Duration::from_secs(0), f);
        let r = Inter.thread_pool.lock();
        match r {
            Err(e) => { Err(TError::new(TErrorKind::Other(e.to_string()))) },
            Ok(mut v) => {
                let task_id = task_action.id();
                v.spwan(Arc::new(task_action));
                Ok(task_id)
            }
        }
    }

    /// stop a ticker action [停止一个计时器任务]
    ///
    /// # Examples
    ///
    /// ```
    /// use grapeTimerR::timer::stop_ticker;
    /// stop_ticker(123);
    /// ```
    pub fn stop_ticker(id:u64) -> TResult<()> {
        let r = Inter.thread_pool.lock();
        match r {
            Err(e) => { Err(TError::new(TErrorKind::Other(e.to_string()))) },
            Ok(mut v) => {
                let r = v.stop_task(id)?;
                Ok(r)
            }
        }
    }

    /// wait main thread forever [永远阻塞主线程，非必须调用]
    ///
    pub fn wait_forever() {
        loop {
            std::thread::sleep(time::Duration::from_secs(1));
        }
    }
}
