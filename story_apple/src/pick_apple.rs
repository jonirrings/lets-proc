use rand::thread_rng;
use rand::RngCore;
use std::sync::atomic::Ordering;
use std::thread;
use std::thread::{current, sleep, ThreadId};
use std::time::Duration;
use AppleBasket;

/**
 * The parent should pick apples on one hand,
 * and also need to deal with special events,
 * such as protecting children from wild dogs,
 * on the other hand.
**/
struct EventTab {
    // probability of this event occuring
    // and at a time only one event can occur
    prob: u32,
    // events occur while picking apples
    event: fn(ThreadId),
}

static EVENT_TABS: [EventTab; 4] = [
    EventTab {
        event: e_bird,
        prob: 8,
    },
    EventTab {
        event: e_wild_dog,
        prob: 5,
    },
    EventTab {
        event: e_neighbor,
        prob: 10,
    },
    EventTab {
        event: e_break,
        prob: 6,
    },
];

fn e_bird(per: ThreadId) {
    let ms_time = 1000;
    sleep(Duration::from_millis(1000));
    println!(
        "[P{:?}]:
A bird flies around the apple tree,
with eyes staring on apples.
It takes {} ms to scare it away.",
        per, ms_time
    );
}

fn e_wild_dog(per: ThreadId) {
    let ms_time = 2500;
    sleep(Duration::from_millis(2500));
    println!(
        "[P{:?}]:
A wild dog runs fast towards here,
whose barks make us crazy.
Drive it away with {} ms.",
        per, ms_time
    );
}

fn e_neighbor(per: ThreadId) {
    let ms_time = 500;
    sleep(Duration::from_millis(500));
    println!(
        "[P{:?}]:
An acquaintance walks past,
saying hello to us.
It's polite to take {} ms to greet him.",
        per, ms_time
    );
}

fn e_break(per: ThreadId) {
    let ms_time = 2000;
    sleep(Duration::from_millis(2000));
    println!(
        "[P{:?}]:
Tired, and better to take a break with {} ms.",
        per, ms_time
    );
}

fn gen_event() -> Option<&'static EventTab> {
    // used to gen random event
    let mut rng = thread_rng();
    let p = rng.next_u32() % 100;
    let mut total = 0u32;
    // event based on probability
    for event_tab in EVENT_TABS.iter() {
        total += event_tab.prob;
        if p < total {
            return Some(event_tab);
        }
    }
    None
}

fn job_pick(bask: &AppleBasket, per: ThreadId) {
    // pick an apple
    thread::sleep(Duration::from_millis(500));
    let (ref lock, ref cvar) = bask.count;
    let mut count = lock.lock().unwrap();
    *count += 1;
    println!("[P{:?}]: Pick an apple. ({}/{})", per, *count, bask.max_vol);
    // from 0 to 1
    if *count == 1 {
        cvar.notify_one();
    }
    // basket is full
    if *count == bask.max_vol {
        // close basket
        bask.open.store(false, Ordering::Relaxed);
        println!("Basket is full and then close it.");
        // back home, wake all children
        cvar.notify_all();
    }
}

pub fn pick_thread(bask: &AppleBasket) {
    let per = current().id();
    while bask.open.load(Ordering::Relaxed) {
        match gen_event() {
            Some(ref event_tab) => (event_tab.event)(per),
            None => job_pick(&*bask, per),
        }
    }
}
