use std::process::ExitStatus;
use value_box::{ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn process_exit_status_success(exit_status_ptr: *mut ValueBox<ExitStatus>) -> bool {
    exit_status_ptr.with_not_null_return(false, |exit_status| exit_status.success())
}

#[no_mangle]
pub fn process_exit_status_has_code(exit_status_ptr: *mut ValueBox<ExitStatus>) -> bool {
    exit_status_ptr.with_not_null_return(false, |exit_status| exit_status.code().is_some())
}

#[no_mangle]
pub fn process_exit_status_code(exit_status_ptr: *mut ValueBox<ExitStatus>) -> i32 {
    exit_status_ptr.with_not_null_return(-1, |exit_status| exit_status.code().unwrap_or(-1))
}

#[no_mangle]
pub fn process_exit_status_drop(ptr: *mut ValueBox<ExitStatus>) {
    ptr.release();
}
