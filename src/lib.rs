pub mod child;
pub mod command;
pub mod exit_status;
pub mod output;

#[no_mangle]
pub fn process_test() -> bool {
    return true;
}