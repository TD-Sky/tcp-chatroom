mod message_queue;

use std::sync::OnceLock;

use message_queue::MessageQueue;

static MQ: OnceLock<MessageQueue> = OnceLock::new();

#[inline]
pub fn mq<'a>() -> &'a MessageQueue {
    MQ.get_or_init(MessageQueue::default)
}
