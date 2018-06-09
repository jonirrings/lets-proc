use std::sync::atomic::Ordering;
use std::thread::{current, sleep, ThreadId};
use std::time::Duration;
use AppleBasket;

fn job_eat(basket: &AppleBasket, kid: ThreadId) {
    if !basket.open.load(Ordering::Relaxed) {
        return;
    }
    {
        let (ref lock, ref cvar) = basket.count;
        let mut count = lock.lock().unwrap();
        while basket.open.load(Ordering::Relaxed) && *count == 0 {
            count = cvar.wait(count).unwrap();
        }
        if basket.open.load(Ordering::Relaxed) {
            *count -= 1;
            println!(
                "[K{:?}]: Eat an apple. ({}/{})",
                kid, *count, basket.max_vol
            );
        };
    }
    sleep(Duration::from_millis(3000));
}

pub fn eat_thread(bask: &AppleBasket) {
    let kid = current().id();
    while bask.open.load(Ordering::Relaxed) {
        job_eat(bask, kid);
    }
}
