use std::hash::Hash;
use std::fmt::{Debug, Display};
use std::sync::mpsc::Sender;

pub enum Message<K, V>
where
    K: Eq + Hash + Debug + Display + Clone + Send + Sync,
    V: Send + Sync,
{
    Clear,
    Get(K),
    Insert(K, V),
    Remove(K),
}

pub struct MessageWrapper<K, V>
where
    K: Eq + Hash + Debug + Display + Clone + Send + Sync,
    V: Send + Sync,
{
    pub sender: Sender<Response<V>>,
    pub kind: Message<K, V>,
}

pub enum Response<V>
where
    V: Send + Sync,
{
    Ok,
    Absent,
    Value(V),
}
