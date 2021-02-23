use crate::prelude::*;
use alloc::{boxed::Box, collections::VecDeque};
use core::pin::Pin;

static TASK_QUEUE: StaticMutex<Channel<Pin<Box<dyn Future<Output = ()>>>>> =
    StaticMutex::Unitialized;

pub struct Executor;

impl Executor {
    pub unsafe fn init() {
        TASK_QUEUE.init();
    }

    pub fn spawn(f: impl Future<Output = ()> + 'static) {
        let f = unsafe { Pin::new_unchecked(Box::new(f)) };
        TASK_QUEUE.spin_lock().blocking_send(f);
    }

    pub fn run() {
        spin_on::spin_on(async {
            let mut tasks: VecDeque<Pin<Box<dyn Future<Output = ()>>>> = Default::default();
            loop {
                while let Some(t) = TASK_QUEUE.lock().await.recv().await {
                    tasks.push_back(t);
                }
                while let Some(mut task) = tasks.pop_front() {
                    let res = future::poll_once(&mut task).await;
                    if res.is_none() {
                        tasks.push_back(task);
                    }
                    while let Some(t) = TASK_QUEUE.lock().await.recv().await {
                        tasks.push_back(t);
                    }
                    RTOS::do_yield();
                }
            }
        })
    }
}
