use std::thread;
use std::thread::JoinHandle;
use std::hash::Hash;
use std::fmt::{Debug, Display};
use std::sync::mpsc::{Sender, channel};
use lru::LRU;
use super::message::{Message, MessageWrapper, Response};
use super::state::State;

pub struct Actor<K, V>
where
    K: Eq + Hash + Debug + Display + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    thread: JoinHandle<()>,
    sender: Sender<MessageWrapper<K, V>>,
}

impl<K, V> Actor<K, V>
where
    K: Eq + Hash + Debug + Display + Clone + Send + Sync,
    V: Send + Sync + Clone,
{
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = channel::<MessageWrapper<K, V>>();

        let thread = thread::spawn(move || match LRU::new(capacity) {
            Some(lru) => {
                let mut state = State::new(lru);

                for message in rx {
                    state.handle(message).unwrap();
                }
            }
            None => (),
        });

        Actor {
            thread: thread,
            sender: tx,
        }
    }

    pub fn clear(&self) -> bool {
        match self.send(Message::Clear) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get(&self, key: K) -> Option<V> {
        match self.send(Message::Get(key)) {
            Some(Response::Value(value)) => Some(value),
            Some(Response::Absent) => None,
            _ => None,
        }
    }

    pub fn insert(&self, key: K, value: V) -> bool {
        match self.send(Message::Insert(key, value)) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn remove(&self, key: K) -> Option<V> {
        match self.send(Message::Remove(key)) {
            Some(Response::Value(value)) => Some(value),
            Some(Response::Absent) => None,
            _ => None,
        }
    }

    pub fn join(self) -> bool {
        match self.thread.join() {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn send(&self, kind: Message<K, V>) -> Option<Response<V>> {
        let (tx, rx) = channel::<Response<V>>();

        match self.sender.send(MessageWrapper {
            sender: tx,
            kind: kind,
        }) {
            Ok(_) => {}
            Err(_) => return None,
        };

        match rx.recv() {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
}
