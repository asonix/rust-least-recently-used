use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct LRUNode<K> {
    key: K,
    next: RefCell<Option<Rc<LRUNode<K>>>>,
    prev: RefCell<Option<Weak<LRUNode<K>>>>,
}

impl<K> LRUNode<K> {
    pub fn new(key: K) -> Self {
        LRUNode {
            key: key,
            next: RefCell::new(None),
            prev: RefCell::new(None),
        }
    }

    pub fn get_key(&self) -> &K {
        &self.key
    }

    pub fn set_next(&self, next: Rc<Self>) {
        *self.next.borrow_mut() = Some(next);
    }

    pub fn clear_next(&self) -> Option<Rc<Self>> {
        let next = self.next.borrow().clone();
        *self.next.borrow_mut() = None;
        next
    }

    pub fn get_next(&self) -> Option<Rc<Self>> {
        match *self.next.borrow() {
            Some(ref next) => Some(Rc::clone(next)),
            None => None,
        }
    }

    pub fn set_prev(&self, prev: Weak<Self>) {
        *self.prev.borrow_mut() = Some(prev);
    }

    pub fn clear_prev(&self) -> Option<Rc<Self>> {
        let prev = self.prev.borrow().clone();
        *self.prev.borrow_mut() = None;

        match prev {
            Some(prev) => Weak::upgrade(&prev),
            None => None,
        }
    }

    pub fn get_prev(&self) -> Option<Rc<Self>> {
        match *self.prev.borrow() {
            Some(ref prev) => Weak::upgrade(prev),
            None => None,
        }
    }

    pub fn remove(&self) {
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
