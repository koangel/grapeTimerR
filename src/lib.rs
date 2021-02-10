pub mod tickerData;
pub mod schedule;
pub mod parsers;
pub mod errors;

pub mod grape_timer {
    use crate::schedule::schedule;
    use chrono::Duration;
    use crate::schedule::schedule::TaskAction;
    use crate::errors::errors::TError;

    #[derive(Copy, Clone)]
    pub struct Config {
        pub sync_call:bool,
        pub max_count:u32,
        pub debug:bool,
        pub thread_count:i32,
    }

    /// init schedule system [用于初始化调度系统，通过Config]
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn init_schedule(t:std::time::Duration,conf:Config) -> Result<(),TError> {
        Ok(())
    }

    /// create a new ticker action [创建一个计时器任务]
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn new_ticker(tick:Duration, loopCount:i32, f: Box<dyn FnMut(u64)>) -> Result<u64,TError> {
        Ok(1)
    }

    /// create a new trait ticker action [创建一个Trait模式计时器任务]
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn new_ticker_trait(tick:Duration, loopCount:i32, ft:Box<dyn TaskAction>) -> Result<u64,TError> {
        Ok(1)
    }

    /// create a new ticker action [创建一个计时器任务]
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn new_date(dateformate:&str, loopCount:i32, f: Box<dyn FnMut(u64)>) -> Result<u64,TError> {
        Ok(1)
    }

    /// create a new ticker action [创建一个计时器任务]
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn new_date_trait(dateformate:&str, loopCount:i32, ft:Box<dyn TaskAction>) -> Result<u64,TError> {
        Ok(1)
    }

    /// stop a ticker action [停止一个计时器任务]
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn stop_ticker(id:u64) -> Result<(),TError> {
        Ok(())
    }


}
