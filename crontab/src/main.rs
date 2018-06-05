extern crate libloading;

use libloading::{Library, Symbol};
use std::env;
type PlugTask = unsafe fn() -> ();

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
