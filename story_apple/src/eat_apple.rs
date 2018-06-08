use pick_apple::AppleBasket;
use std::thread;
use std::time::Duration;

fn job_eat(bask: &mut AppleBasket, kid: u64) {
    let ms_time = 3000;
    if !bask.open {
        return;
    }
    let &(ref lock, ref cvar) = &*bask.count;
    let mut count = lock.lock().unwrap();
    if bask.open && *bask.count <= 0 {
        // no apples
        cvar.wait(count);
        if bask.open {
            *bask.count -= 1;
            print!(
                "[K{}]: Eat an apple. ({}/{})\n",
                kid, *bask.count, *bask.max_vol
            );
        }
    }
    let duration = Duration::from_millis(ms_time);
    thread::sleep(duration);
}

pub fn eat_thread(bask: &mut AppleBasket) {
    let kid = 9994u64;
    if bask.open {
        job_eat(bask, kid);
    }
}
