extern crate rand;

mod pick_apple;
mod eat_apple;

fn pick_eat(kid: u32) {
// thread -- parent pick apple
// threads -- children eat apple
}

#[no_mangle]
pub extern "C" fn plug_task() {
    pick_eat(2);
}
