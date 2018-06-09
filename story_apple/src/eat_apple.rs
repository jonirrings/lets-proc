use pick_apple::AppleBasket;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::thread::ThreadId;
use std::time::Duration;

fn job_eat(bask: &AppleBasket, kid: ThreadId) {
    let ms_time = 3000;
    let duration = Duration::from_millis(ms_time);
    thread::sleep(duration);
    let (ref lock, ref cvar) = bask.count;
    let mut count = lock.lock().unwrap();
    count = cvar.wait(count).unwrap();
    if *count > 0 {
        // no apples
        *count -= 1;
        println!("[K{:?}]: Eat an apple. ({}/{})", kid, *count, bask.max_vol);
    };
}

pub fn eat_thread(bask: Arc<AppleBasket>) {
    let kid = thread::current().id();
    while bask.open.load(Ordering::Relaxed) {
        job_eat(&*bask, kid);
    }
}
