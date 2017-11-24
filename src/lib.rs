mod lru_node;
mod lru;
mod multi;

pub use lru::LRU;
pub use multi::Actor;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
