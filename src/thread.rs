
pub mod threads {
    use log::*;
    use tokio::{runtime,time};
    use crossbeam::channel::{unbounded,Receiver,Sender};
    use std::{ thread,sync::Mutex,sync::Once,time::Duration,
               sync::atomic::AtomicI8, sync::atomic::Ordering,
               sync::Arc };
    use chrono::{Local};
    use crate::schedule::schedule::{TaskAction};
    use crate::errors::errors::{TError, TResult, TErrorKind};
    use std::borrow::{BorrowMut, Borrow};
    use std::ops::Add;
    use crate::uuid::uuid::next_big_id;
    use crate::parsers::parsers::parser_timestamp;
    use std::sync::mpsc::channel;

    pub struct TaskPool {
        rt:tokio::runtime::Runtime,
        stop_tx:Sender<u64>,
        stop_rx:Receiver<u64>,
        debug:bool,
    }

    impl TaskPool {
        pub fn new(tick:Duration,count:i32) -> TaskPool {
            let (tx,rx) =unbounded();

            TaskPool {
                rt: runtime::Builder::new_multi_thread()
                    .worker_threads(count as usize)
                    .enable_all()
                    .build()
                    .unwrap(),
                stop_tx: tx,
                stop_rx: rx,
                debug:false,
            }
        }

        pub fn rebuild(&mut self,count:i32,debug:bool) {
            self.debug = debug;
            self.rt = runtime::Builder::new_multi_thread()
                .worker_threads(count as usize)
                .enable_all()
                .build()
                .unwrap();
        }

        pub fn stop_task(&mut self,id:u64) -> TResult<()> {
            match self.stop_tx.try_send(id) {
                Err(e) => { Err(TError::new(TErrorKind::Other(e.to_string()))) }
                Ok(_) => { Ok(()) }
            }
        }

        pub fn spwan(&self, t:Arc<dyn TaskAction>) {
            let task = t.clone();
            let rx_spwan = self.stop_rx.clone();
            let debug = self.debug;

            self.rt.spawn(async move {
                let mut r_count = 0;
                let max_count = task.loop_count();
                loop {
                    // 先暂停
                    if task.date_format().len() > 0 {
                        let now_time = Local::now().timestamp();
                        let next_tick = parser_timestamp(task.date_format()).unwrap();

                        if debug {
                            debug!("make next tick sec:{} id:{}",(next_tick - now_time) as u64,task.id());
                        }

                        // 等待一下，让出这个线程
                        time::sleep(time::Duration::from_secs( (next_tick - now_time) as u64 )).await;
                    }else {
                        if task.tick() <= 0 {
                            break // 异常的任务
                        }

                        if debug {
                            debug!("make next ticker sec:{} id:{}",(task.tick()) as u64,task.id());
                        }

                        time::sleep(time::Duration::from_millis( task.tick() )).await;
                    }

                    if max_count  > 0 && r_count >= max_count {
                        if debug {
                            debug!("task finished:{}",task.id());
                        }
                        break; // 结束这个任务
                    }

                    r_count+=1;//计数
                    if debug {
                        debug!("task run count:{} id:{}",r_count,task.id());
                    }
                    task.execute(task.id());

                    // 检测一下，是不是被强制结束了
                    let sr = rx_spwan.try_recv();
                    match sr {
                        Err(e) => {  },
                        Ok(val) => {
                            if task.id() == val {
                                // 结束这个任务
                                debug!("task stopped:{}",task.id());
                                return;
                            }
                        },
                    }
                }
            });
        }
    }
}