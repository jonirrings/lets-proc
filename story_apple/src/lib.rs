extern crate rand;

mod eat_apple;
mod pick_apple;

fn pick_eat(kid: u64) {
    // thread -- parent pick apple
    // threads -- children eat apple
}

#[no_mangle]
pub extern "C" fn plug_task() {
    pick_eat(2);
}
