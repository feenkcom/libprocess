#[macro_use]
extern crate log;

pub mod child;
pub mod command;
pub mod exit_status;
pub mod output;
mod async_buffer;

#[no_mangle]
pub fn process_test() -> bool {
    return true;
}
