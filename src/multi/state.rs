use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::mpsc::SendError;

use lru::LRU;
use super::message::{Message, MessageWrapper, Response};

pub struct State<K, V>
where
    K: Eq + Hash + Debug + Display + Clone + Send + Sync,
    V: Send + Sync + Clone,
{
    lru: LRU<K, V>,
}

impl<K, V> State<K, V>
where
    K: Eq + Hash + Debug + Display + Clone + Send + Sync,
    V: Send + Sync + Clone,
{
    pub fn new(lru: LRU<K, V>) -> Self {
        State { lru }
    }

    pub fn handle(&mut self, message: MessageWrapper<K, V>) -> Result<(), SendError<Response<V>>> {
        match message.kind {
            Message::Clear => {
                self.lru.clear();
                message.sender.send(Response::Ok)
            }
            Message::Get(key) => {
                let v = self.lru.get(&key);
                match v {
                    Some(v) => message.sender.send(Response::Value(v.clone())),
                    None => message.sender.send(Response::Absent),
                }
            }
            Message::Insert(key, value) => {
                self.lru.insert(key, value);
                message.sender.send(Response::Ok)
            }
            Message::Remove(key) => {
                match self.lru.remove(&key) {
                    Some(v) => message.sender.send(Response::Value(v)),
                    None => message.sender.send(Response::Absent),
                }
            }
        }
    }
}
