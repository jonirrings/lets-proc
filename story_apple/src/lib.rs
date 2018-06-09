extern crate crossbeam_utils;
extern crate rand;

mod eat_apple;
mod pick_apple;

use std::sync::atomic::AtomicBool;
use std::sync::{Condvar, Mutex};

use crossbeam_utils::scoped;

pub struct AppleBasket {
    // max volume
    pub max_vol: u32,
    // apple count
    pub count: (Mutex<u32>, Condvar),
    //close basket if it's full
    pub open: AtomicBool,
}

fn pick_eat(kid: usize) {
    // empty basket
    let basket = AppleBasket {
        max_vol: 50,
        open: AtomicBool::new(true),
        count: (Mutex::new(0), Condvar::new()),
    };
    print!("A parent with {} children goes to pick apples.\n", kid);
    scoped::scope(|scope| {
        // thread -- parent pick apple
        let parent = scope.spawn(|| pick_apple::pick_thread(&basket));
        // threads -- children eat apple
        let mut kids = Vec::with_capacity(kid);
        for _ in 0..kid {
            kids.push(scope.spawn(|| eat_apple::eat_thread(&basket)));
        }
        // wait until all threads over
        parent.join().unwrap();
        for handle in kids {
            handle.join().unwrap();
        }
    });

    print!("They are back on the way home.\n");
}

#[no_mangle]
pub extern "C" fn plug_task() {
    pick_eat(2);
}

#[cfg(test)]
mod tests {
    use super::plug_task;

    #[test]
    fn plug_task_test() {
        assert_eq!(plug_task(), ());
    }
}
