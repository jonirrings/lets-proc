use rand::thread_rng;
use rand::RngCore;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::ThreadId;
use std::time::Duration;

pub struct AppleBasket {
    // apple count
    pub count: Arc<(Mutex<u32>, Condvar)>,
    // max volume
    pub max_vol: u32,
    //close basket if it's full
    pub open: bool,
}

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
    event: fn(u64),
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

fn e_bird(per: u64) {
    let ms_time = 1000;
    let duration = Duration::from_millis(ms_time);
    thread::sleep(duration);
    print!(
        "[P{:?}]:\n
A bird flies around the apple tree,\n
with eyes staring on apples.\n
It takes {} ms to scare it away.\n",
        per, ms_time
    );
}

fn e_wild_dog(per: u64) {
    let ms_time = 2500;
    let duration = Duration::from_millis(ms_time);
    thread::sleep(duration);
    print!(
        "[P{:?}]:\n
A wild dog runs fast towards here,\n
whose barks make us crazy.\n
Drive it away with {} ms.\n",
        per, ms_time
    );
}

fn e_neighbor(per: u64) {
    let ms_time = 500;
    let duration = Duration::from_millis(ms_time);
    thread::sleep(duration);
    print!(
        "[P{:?}]:\n
An acquaintance walks past,\n
saying hello to us.\n
It's polite to take {} ms to greet him.\n",
        per, ms_time
    );
}

fn e_break(per: u64) {
    let ms_time = 2000;
    let duration = Duration::from_millis(ms_time);
    thread::sleep(duration);
    print!(
        "[P{:?}]:\n
Tired, and better to take a break with {} ms.\n",
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
    return None;
}

fn job_pick(bask: &mut AppleBasket, per: u64) {
    // basket is full
    if !bask.open {
        return;
    }
    let ms_time = 500;
    let duration = Duration::from_millis(ms_time);
    // pick an apple
    thread::sleep(duration);
    {
        let &(ref lock, ref cvar) = &*bask.count;
        let mut count = lock.lock().unwrap();
        *count += 1;
        print!(
            "[P{:?}]: Pick an apple. ({}/{})\n",
            per, *count, bask.max_vol
        );
        // from 0 to 1
        if *count == 1 {
            cvar.notify_one();
        }
        // basket is full
        if *count >= bask.max_vol {
            // close basket
            bask.open = false;
            print!("Basket is full and then close it.\n");
            // back home, wake all children
            cvar.notify_all();
        }
    }
}

pub fn pick_thread(bask: &mut AppleBasket) {
    let per = 9999u64;
    while bask.open {
        // random event
        let do_event = gen_event();
        match do_event {
            Some(ref event_tab) => (event_tab.event)(per),
            None => job_pick(bask, per),
        }
    }
}
