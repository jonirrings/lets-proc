extern crate libloading;

use std::env;
use libloading::{Library, Symbol};
type PlugTask = unsafe fn()->();

fn main() {
    println!("Hello from crontab");
    let library_path = env::args().nth(1).expect("USAGE: loading <LIB>");
    println!("Loading plug_task() from {}", library_path);
    let lib = Library::new(library_path).unwrap();
    unsafe {
        let func: Symbol<PlugTask> = lib.get(b"plug_task").unwrap();
        func();
    }
}
