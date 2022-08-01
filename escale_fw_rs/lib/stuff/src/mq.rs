use alloc::vec::Vec;

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
        F: FnMut(&M, &mut dyn FnMut(M) -> ()) -> MessageProcessingStatus,
    {
        let mut new = Vec::<M>::default();
        self.msgs
            .retain(|msg| match f(msg, &mut (|m| new.push(m))) {
                MessageProcessingStatus::Ignored => true,
                MessageProcessingStatus::Processed => false,
            });
        self.msgs.append(&mut new);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_mq_is_empty() {
        let mut mq = MessageQueue::<i32>::default();
        mq.process(|_, _| panic!("the queue must be empty"));
    }

    #[test]
    fn a_message_can_be_pushed_ignored_processed() {
        let mut mq = MessageQueue::<i32>::default();
        mq.push(1);
        mq.process(|&m, _| {
            assert_eq!(m, 1);
            MessageProcessingStatus::Ignored
        });
        mq.process(|&m, _| {
            assert_eq!(m, 1);
            MessageProcessingStatus::Processed
        });
        mq.process(|_, _| panic!("the queue must be empty"));
    }

    #[test]
    fn a_message_can_be_pushed_when_processing() {
        let mut mq = MessageQueue::<i32>::default();
        mq.push(1);
        mq.process(|&m, push| {
            assert_eq!(m, 1);
            push(2);
            MessageProcessingStatus::Processed
        });
        mq.process(|&m, _| {
            assert_eq!(m, 2);
            MessageProcessingStatus::Processed
        });
        mq.process(|_, _| panic!("the queue must be empty"));
    }
}
