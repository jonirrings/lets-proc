extern crate rand;

use rand::prelude::*;
use std::char;

fn is_printable_ascii(ch: u32) -> Option<char> {
    if ch <= 0x20u32 || ch >= 0x7eu32 {
        return None;
    }
    return char::from_u32(ch);
}

fn print_alien_ton() {
    let mut ws = Vec::with_capacity(64);
    let mut rng = thread_rng();
    loop {
        let rand_u32 = rng.next_u32() % 128;
        match is_printable_ascii(rand_u32) {
            None => continue,
            Some(ch) => ws.push(ch),
        }
        if ws.len() >= 64 {
            break;
        }
    }
    println!("Alien Words: {}", ws.iter().collect::<String>())
}

#[no_mangle]
pub extern "C" fn plug_task() {
    print_alien_ton();
}
