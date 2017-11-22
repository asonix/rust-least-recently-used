extern crate least_recently_used;

use least_recently_used::LRU;

fn main() {
    let mut lru = LRU::new(6).unwrap();

    for i in 1..8 {
        println!("");
        println!("Inserting: {}", i);
        lru.insert(i, i);
        println!("{}", lru);
    }

    println!("");
    println!("Getting 1");
    lru.get(&1);
    println!("{}", lru);

    println!("");
    println!("Getting 2");
    lru.get(&2);
    println!("{}", lru);

    println!("");
    println!("Inserting 8");
    lru.insert(8, 8);
    println!("{}", lru);

    println!("");
    println!("Getting 3");
    lru.get(&3);
    println!("{}", lru);

    println!("");
    println!("Getting 2");
    lru.get(&2);
    println!("{}", lru);

    println!("");
    println!("Removing 2");
    lru.remove(&2);
    println!("{}", lru);

    println!("");
    println!("Getting 2");
    lru.get(&2);
    println!("{}", lru);

    println!("");
    println!("Inserting 9");
    lru.insert(9, 9);
    println!("{}", lru);

    println!("");
    println!("Getting 4");
    lru.get(&4);
    println!("{}", lru);

    println!("");
    println!("Inserting 10");
    lru.insert(10, 10);
    println!("{}", lru);

    println!("");
    println!("Getting 5");
    lru.get(&5);
    println!("{}", lru);

    println!("");
    println!("Inserting 9");
    lru.insert(9, 13);
    println!("{}", lru);
}
