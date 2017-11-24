extern crate least_recently_used;

use least_recently_used::LRU;
use least_recently_used::Actor;

fn in_thread() {
    let mut lru = LRU::new(6).unwrap();

    for i in 1..8 {
        lru.insert(i, i);
        println!("Inserted {}", i);
    }

    if let Some(v) = lru.get(&1) {
        println!("[failed] Got {}", v);
    } else {
        println!("[expected] Failed to get for 1");
    }

    if let Some(v) = lru.get(&2) {
        println!("[expected] Got {}", v);
    } else {
        println!("[failed] Failed to get for 2");
    }

    lru.insert(8, 8);
    println!("Inserted 8");

    if let Some(v) = lru.get(&3) {
        println!("[failed] Got {}", v);
    } else {
        println!("[expected] Failed to get for 3");
    }

    if let Some(v) = lru.get(&2) {
        println!("[expected] Got {}", v);
    } else {
        println!("[failed] Failed to get for 2");
    }

    if let Some(v) = lru.remove(&2) {
        println!("[expected] Removed {}", v);
    } else {
        println!("[failed] Failed to remove for 2");
    }

    if let Some(v) = lru.get(&2) {
        println!("[failed] Got {}", v);
    } else {
        println!("[expected] Failed to get for 2");
    }

    lru.insert(9, 9);
    println!("Inserted 9");

    if let Some(v) = lru.get(&4) {
        println!("[expected] Got {}", v);
    } else {
        println!("[failed] Failed to get for 4");
    }

    lru.insert(10, 10);
    println!("Inserted 10");

    if let Some(v) = lru.get(&5) {
        println!("[failed] Got {}", v);
    } else {
        println!("[expected] Failed to get for 5");
    }

    lru.insert(9, 13);
    println!("Inserted 9");
}

fn out_of_thread() {
    let lru = Actor::new(6);

    for i in 1..8 {
        if lru.insert(i, i) {
            println!("[expected] Inserted {}", i);
        } else {
            println!("[failed] Failed to insert for {}", i);
        }
    }

    if let Some(v) = lru.get(1) {
        println!("[failed] Got {}", v);
    } else {
        println!("[expected] Failed to get for 1");
    }

    if let Some(v) = lru.get(2) {
        println!("[expected] Got {}", v);
    } else {
        println!("[failed] Failed to get for 2");
    }

    if lru.insert(8, 8) {
        println!("[expected] Inserted 8");
    } else {
        println!("[failed] Failed to insert for 8");
    }

    if let Some(v) = lru.get(3) {
        println!("[failed] Got {}", v);
    } else {
        println!("[expected] Failed to get for 3");
    }

    if let Some(v) = lru.get(2) {
        println!("[expected] Got {}", v);
    } else {
        println!("[failed] Failed to get for 2");
    }

    if let Some(v) = lru.remove(2) {
        println!("[expected] Removed {}", v);
    } else {
        println!("[failed] Failed to remove for 2");
    }

    if let Some(v) = lru.get(2) {
        println!("[failed] Got {}", v);
    } else {
        println!("[expected] Failed to get for 2");
    }

    if lru.insert(9, 9) {
        println!("[expected] Inserted 9");
    } else {
        println!("[failed] Failed to insert for 9");
    }

    if let Some(v) = lru.get(4) {
        println!("[expected] Got {}", v);
    } else {
        println!("[failed] Failed to get for 4");
    }

    if lru.insert(10, 10) {
        println!("[expected] Inserted 10");
    } else {
        println!("[failed] Failed to insert for 10");
    }

    if let Some(v) = lru.get(5) {
        println!("[failed] Got {}", v);
    } else {
        println!("[expected] Failed to get for 5");
    }

    if lru.insert(9, 13) {
        println!("[expected] Inserted 9");
    } else {
        println!("[failed] Failed to insert for 9");
    }
}

fn main() {
    println!("In thread");
    in_thread();
    println!("");
    println!("Out of thread");
    out_of_thread();
}
