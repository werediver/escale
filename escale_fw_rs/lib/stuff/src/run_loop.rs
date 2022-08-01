use alloc::vec::Vec;
use core::marker::PhantomData;

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
