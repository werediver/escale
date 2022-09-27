use alloc::vec::Vec;
use ring::{ring::Ring, ring_state::Overwriting};

pub struct MessageQueue<M, const N: usize> {
    msgs: Ring<M, N, Overwriting>,
}

pub enum MessageProcessingStatus {
    Ignored,
    Processed,
}

impl<M, const N: usize> Default for MessageQueue<M, N> {
    fn default() -> Self {
        MessageQueue {
            msgs: Default::default(),
        }
    }
}

impl<M, const N: usize> MessageQueue<M, N> {
    pub fn push(&mut self, msg: M) {
        _ = self.msgs.push(msg);
    }

    pub fn process<F>(&mut self, mut f: F)
    where
        F: FnMut(&M, &mut dyn FnMut(M)) -> MessageProcessingStatus,
    {
        let mut new = Vec::<M>::default();
        self.msgs
            .retain(|msg| match f(msg, &mut (|m| new.push(m))) {
                MessageProcessingStatus::Ignored => true,
                MessageProcessingStatus::Processed => false,
            });
        self.msgs.append(new);
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
