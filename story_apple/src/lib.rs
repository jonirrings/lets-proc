extern crate rand;

mod eat_apple;
mod pick_apple;

use pick_apple::AppleBasket;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn pick_eat(kid: usize) {
    // empty basket
    let basket = AppleBasket {
        max_vol: 50,
        open: AtomicBool::new(true),
        count: (Mutex::new(0), Condvar::new()),
    };
    print!("A parent with {} children goes to pick apples.\n", kid);
    let wrapped_basket = Arc::new(basket);
    // thread -- parent pick apple
    let parent_basket = wrapped_basket.clone();
    let parent = thread::spawn(|| {
        pick_apple::pick_thread(parent_basket);
    });
    // threads -- children eat apple
    let mut kids = Vec::with_capacity(kid);
    for _ in 0..kid {
        let cloned_basket = wrapped_basket.clone();
        kids.push(thread::spawn(move || {
            eat_apple::eat_thread(cloned_basket);
        }));
    }
    // wait until all threads over
    parent.join().unwrap();
    for handle in kids {
        handle.join().unwrap();
    }
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
