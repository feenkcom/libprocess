#![allow(non_snake_case)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate value_box;
#[macro_use]
extern crate phlow;

pub mod async_buffer;
pub mod child;
pub mod command;
pub mod exit_status;
pub mod output;
pub mod stdio;

pub use phlow_extensions::CoreExtensions;
pub use phlow_ffi::*;
pub use value_box_ffi::*;

import_extensions!(CoreExtensions);

#[no_mangle]
pub fn process_test() -> bool {
    return true;
}

#[no_mangle]
pub fn process_init_env_logger() {
    if let Err(error) = env_logger::try_init() {
        eprintln!(
            "[{}] Failed to initialize env.logger due to {:?}",
            line!(),
            error
        );
    }
}
