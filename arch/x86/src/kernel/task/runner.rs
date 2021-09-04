// Copied and edited from: Phillip Opperman https://os.phil-opp.com/async-await/

use alloc::boxed::Box;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use alloc::task::Wake;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use crossbeam_queue::SegQueue;
use kinfo::info::set_info;
use super::{interrupt_wake, interrupt_wakeups};

pub type Task = Pin<Box<dyn Future<Output = ()>>>;
type TaskId = usize;

pub struct Executor {
    task_queue: VecDeque<Task>,
    wake_queue: Arc<SegQueue<TaskId>>,
    pending_tasks: BTreeMap<TaskId, Task>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            task_queue: VecDeque::new(),
            wake_queue: Arc::new(SegQueue::new()),
            pending_tasks: BTreeMap::new(),
        }
    }

    pub fn spawn(&mut self, task: impl Future<Output = ()> + 'static) {
        self.task_queue.push_back(Box::pin(task))
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.apply_interrupt_wakeups();
            self.wake_waiting_tasks();
            self.hlt_if_idle();
        }
    }

    fn run_ready_tasks(&mut self) {
        while let Some(mut task) = self.task_queue.pop_front() {
            let waker = self.create_waker(&task).into();
            let mut context = Context::from_waker(&waker);
            match task.as_mut().poll(&mut context) {
                Poll::Ready(()) => {} // task done
                Poll::Pending => {
                    // add task to pending_tasks and wait for wakeup
                    let task_id = Self::task_id(&task);
                    if self.pending_tasks.insert(task_id, task).is_some() {
                        unsafe {
                            set_info("not ok");
                            kinfo!("Task with the same ID is already pending");
                        }
                    }
                }
            }
        }
    }

    fn apply_interrupt_wakeups(&mut self) {
        while let Ok(waker) = interrupt_wakeups().pop() {
            waker.wake();
        }
    }

    fn wake_waiting_tasks(&mut self) {
        while let Ok(task_id) = self.wake_queue.pop() {
            if let Some(task) = self.pending_tasks.remove(&task_id) {
                self.task_queue.push_back(task);
            } else {
                unsafe {
                    set_info("not ok");
                    kinfo!("Woken task not found in pending_tasks");
                }
            }
        }
    }

    fn hlt_if_idle(&self) {
        if self.task_queue.is_empty() {
            x86_64::instructions::interrupts::disable();
            if interrupt_wakeups().is_empty() {
                x86_64::instructions::interrupts::enable_and_hlt();
            } else {
                x86_64::instructions::interrupts::enable();
            }
        }
    }

    fn task_id(task: &Task) -> TaskId {
        let future_ref: &dyn Future<Output = ()> = &**task;
        future_ref as *const _ as *const () as usize
    }

    fn create_waker(&self, task: &Task) -> Arc<Waker> {
        Arc::new(Waker {
            wake_queue: self.wake_queue.clone(),
            task_id: Self::task_id(task),
        })
    }
}

pub struct Waker {
    wake_queue: Arc<SegQueue<TaskId>>,
    task_id: TaskId,
}

impl Wake for Waker {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_queue.push(self.task_id);
    }
}
