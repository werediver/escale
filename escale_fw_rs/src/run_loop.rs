use core::marker::PhantomData;

extern crate alloc;

use alloc::vec::Vec;

pub trait Task<Context> {
    fn run(&mut self, cx: &mut Context) -> TaskStatus;
}

pub enum TaskStatus {
    Pending,
    Done,
}

pub struct Schedule<T, Context>
where
    T: AsMut<dyn Task<Context>>,
{
    tasks: Vec<T>,
    cx: PhantomData<*const Context>,
}

impl<T, Context> Default for Schedule<T, Context>
where
    T: AsMut<dyn Task<Context>>,
{
    fn default() -> Self {
        Schedule {
            tasks: Vec::new(),
            cx: PhantomData,
        }
    }
}

impl<T: AsMut<dyn Task<Context>>, Context> Schedule<T, Context> {
    pub fn push(&mut self, task: T) {
        self.tasks.push(task);
    }
}

impl<T, Context> Task<Context> for Schedule<T, Context>
where
    T: AsMut<dyn Task<Context>>,
{
    fn run(&mut self, cx: &mut Context) -> TaskStatus {
        self.tasks.retain_mut(|task| {
            let task: &mut dyn Task<Context> = task.as_mut();
            let task_status = task.run(cx);

            match task_status {
                TaskStatus::Pending => true,
                TaskStatus::Done => false,
            }
        });
        if self.tasks.is_empty() {
            TaskStatus::Done
        } else {
            TaskStatus::Pending
        }
    }
}

pub struct MessageQueue<M> {
    msgs: Vec<M>,
}

pub enum MessageProcessingStatus {
    Ignored,
    Processed,
}

impl<M> Default for MessageQueue<M> {
    fn default() -> Self {
        MessageQueue { msgs: Vec::new() }
    }
}

impl<M> MessageQueue<M> {
    pub fn push(&mut self, msg: M) {
        self.msgs.push(msg);
    }

    pub fn process<F>(&mut self, mut f: F)
    where
        F: FnMut(&M) -> MessageProcessingStatus,
    {
        self.msgs.retain(|msg| match f(msg) {
            MessageProcessingStatus::Ignored => true,
            MessageProcessingStatus::Processed => false,
        });
    }
}
