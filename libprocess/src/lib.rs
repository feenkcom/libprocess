#![allow(non_snake_case)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate phlow;
#[macro_use]
extern crate value_box;

pub use phlow_extensions::CoreExtensions;
pub use phlow_ffi::*;
pub use value_box_ffi::*;

pub use async_output::AsyncOutput;
pub use semaphore_signaller::SemaphoreSignaller;

pub mod async_buffer;
pub mod async_output;
pub mod child;
pub mod command;
pub mod exit_status;
pub mod output;
pub mod semaphore_signaller;
pub mod stdio;

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
