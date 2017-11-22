use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt::{Debug, Display, Formatter, Result};

pub struct LRU<K, V>
where
    K: Eq + std::hash::Hash + Debug + Display + Clone,
{
    first: Option<Rc<LRUNode<K>>>,
    last: Option<Rc<LRUNode<K>>>,
    map: HashMap<K, (V, Rc<LRUNode<K>>)>,
    capacity: usize,
    count: usize,
}

impl<K, V> LRU<K, V>
where
    K: Eq + std::hash::Hash + Debug + Display + Clone,
{
    pub fn new(capacity: usize) -> Option<Self> {
        if capacity == 0 {
            None
        } else {
            Some(LRU {
                first: None,
                last: None,
                map: HashMap::with_capacity(capacity),
                capacity: capacity,
                count: 0,
            })
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.get(&key);
            if let Some(ref first) = self.first {
                self.map.insert(key.clone(), (value, first.clone()));
            }
            return;
        }

        let new_node = Rc::new(LRUNode::new(key.clone()));

        if self.count < self.capacity {
            self.map.insert(key.clone(), (value, new_node.clone()));

            if let Some(ref first) = self.first {
                new_node.set_next(first.clone());
                first.set_prev(Rc::downgrade(&new_node));
            } else {
                self.last = Some(new_node.clone());
            }

            self.count += 1;

            self.first = Some(new_node);
        } else {
            self.last = if let Some(ref last) = self.last {
                self.map.remove(&last.key);

                last.get_prev()
            } else {
                None
            };

            if let Some(ref last) = self.last {
                last.clear_next();
            }

            self.map.insert(key.clone(), (value, new_node.clone()));

            self.first = if let Some(ref first) = self.first {
                first.set_prev(Rc::downgrade(&new_node));
                new_node.set_next(first.clone());
                Some(new_node)
            } else {
                None
            };
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        match self.map.get(key) {
            Some(ref tup) => {
                let value = &tup.0;
                let node = &tup.1;

                self.last = if let Some(ref last) = self.last {
                    if Rc::ptr_eq(node, last) {
                        node.get_prev()
                    } else {
                        Some(Rc::clone(last))
                    }
                } else {
                    None
                };

                node.remove();

                if let Some(ref first) = self.first {
                    if !Rc::ptr_eq(node, first) {
                        node.set_next(first.clone());
                        first.set_prev(Rc::downgrade(node));
                    }
                }

                self.first = Some(Rc::clone(node));
                if self.last.is_none() {
                    self.last = Some(Rc::clone(node));
                }

                Some(value)
            }
            None => None,
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.map.remove(key) {
            Some((value, node)) => {
                self.last = if let Some(ref last) = self.last {
                    if Rc::ptr_eq(&node, last) {
                        node.get_prev()
                    } else {
                        Some(Rc::clone(last))
                    }
                } else {
                    None
                };

                self.first = if let Some(ref first) = self.first {
                    if Rc::ptr_eq(&node, first) {
                        node.get_next()
                    } else {
                        Some(Rc::clone(first))
                    }
                } else {
                    None
                };

                node.remove();

                self.count -= 1;

                Some(value)
            }
            None => None,
        }
    }

    pub fn clear(&mut self) {
        self.first = None;
        self.last = None;
        self.map.clear();
        self.count = 0;
    }
}

impl<K, V> Display for LRU<K, V>
where
    K: Eq + std::hash::Hash + Debug + Display + Clone,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut v = Vec::new();

        if let Some(ref first) = self.first {
            let mut node = Some(Rc::clone(first));

            loop {
                node = if let Some(ref inner_node) = node {
                    v.push(inner_node.key.clone());
                    inner_node.get_next()
                } else {
                    break;
                };
            }
        }

        let s = v.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(
            ", ",
        );

        write!(f, "LRU: [{}]", s)
    }
}

struct LRUNode<K> {
    key: K,
    next: RefCell<Option<Rc<LRUNode<K>>>>,
    prev: RefCell<Option<Weak<LRUNode<K>>>>,
}

impl<K> LRUNode<K> {
    fn new(key: K) -> Self {
        LRUNode {
            key: key,
            next: RefCell::new(None),
            prev: RefCell::new(None),
        }
    }

    fn set_next(&self, next: Rc<Self>) {
        *self.next.borrow_mut() = Some(next);
    }

    fn clear_next(&self) -> Option<Rc<Self>> {
        let next = self.next.borrow().clone();
        *self.next.borrow_mut() = None;
        next
    }

    fn get_next(&self) -> Option<Rc<Self>> {
        match *self.next.borrow() {
            Some(ref next) => Some(Rc::clone(next)),
            None => None,
        }
    }

    fn set_prev(&self, prev: Weak<Self>) {
        *self.prev.borrow_mut() = Some(prev);
    }

    fn clear_prev(&self) -> Option<Rc<Self>> {
        let prev = self.prev.borrow().clone();
        *self.prev.borrow_mut() = None;

        match prev {
            Some(prev) => Weak::upgrade(&prev),
            None => None,
        }
    }

    fn get_prev(&self) -> Option<Rc<Self>> {
        match *self.prev.borrow() {
            Some(ref prev) => Weak::upgrade(prev),
            None => None,
        }
    }

    fn remove(&self) {
        if let Some(prev) = self.get_prev() {
            if let Some(next) = self.get_next() {
                next.set_prev(Rc::downgrade(&prev));
                prev.set_next(next);
            } else {
                prev.clear_next();
            }
        } else {
            if let Some(next) = self.get_next() {
                next.clear_prev();
            }
        }

        self.clear_next();
        self.clear_prev();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
