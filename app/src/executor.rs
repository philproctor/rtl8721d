use crate::prelude::*;
use alloc::boxed::Box;
use core::pin::Pin;

#[macro_export]
macro_rules! spawn {
    ($( $inner:tt )*) => {
        crate::Executor::spawn(
            $( $inner )*
        );
    };
}

struct Task(Pin<Box<dyn Future<Output = ()>>>);

//Tasks in the executor will never step on each other as they use a coop model
unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Task {
    fn new(f: impl Future<Output = ()> + 'static) -> Self {
        debug!("task created");
        Self(unsafe { Pin::new_unchecked(Box::new(f)) })
    }
}

static mut TASK_QUEUE: heapless::spsc::Queue<Task, consts::U8> =
    heapless::spsc::Queue(heapless::i::Queue::new());

pub struct Executor;

impl Executor {
    pub fn spawn(f: impl Future<Output = ()> + 'static) {
        let f = Task::new(f);
        debug!("spawning task");
        unsafe { TASK_QUEUE.enqueue(f).unwrap_or_default() };
        debug!("task spawned");
    }

    // This is the device's main loop
    pub fn run() {
        spin_on::spin_on(async {
            debug!("Executor run loop started");
            let mut tasks_pending = heapless::Vec::<_, heapless::consts::U16>::new();
            let mut tasks_done_for_loop = heapless::Vec::<_, heapless::consts::U16>::new();
            loop {
                // requeue tasks from last loop
                while let Some(task) = tasks_done_for_loop.pop() {
                    tasks_pending.push(task).unwrap_or_default();
                }

                // retrieve the incoming queue...
                while let Some(task) = unsafe { TASK_QUEUE.dequeue() } {
                    tasks_pending.push(task).unwrap_or_default();
                }

                // poll every task once, this should have them run until they can't any more
                while let Some(mut task) = tasks_pending.pop() {
                    let res = future::poll_once(&mut task.0).await;
                    if res.is_none() {
                        tasks_done_for_loop.push(task).unwrap_or_default();
                    }
                }

                // explicitly yield to the RTOS so that its tasks can run too
                RTOS::do_yield();
            }
        })
    }
}
