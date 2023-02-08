use std::process::ExitStatus;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn process_exit_status_success(exit_status: *mut ValueBox<ExitStatus>) -> bool {
    exit_status
        .with_ref_ok(|exit_status| exit_status.success())
        .or_log(false)
}

#[no_mangle]
pub fn process_exit_status_has_code(exit_status: *mut ValueBox<ExitStatus>) -> bool {
    exit_status
        .with_ref_ok(|exit_status| exit_status.code().is_some())
        .or_log(false)
}

#[no_mangle]
pub fn process_exit_status_code(exit_status: *mut ValueBox<ExitStatus>) -> i32 {
    exit_status
        .with_ref_ok(|exit_status| exit_status.code().unwrap_or(-1))
        .or_log(-1)
}

#[no_mangle]
pub fn process_exit_status_drop(exit_status: *mut ValueBox<ExitStatus>) {
    exit_status.release();
}
