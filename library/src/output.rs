use std::process::{ExitStatus, Output};

use array_box::ArrayBox;
use string_box::StringBox;
use value_box::{BoxerError, ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn process_output_status(output: *mut ValueBox<Output>) -> *mut ValueBox<ExitStatus> {
    output
        .with_ref_ok(|output| output.status.clone())
        .into_raw()
}

#[no_mangle]
pub fn process_output_stderr(output: *mut ValueBox<Output>) -> *mut ValueBox<ArrayBox<u8>> {
    output
        .with_ref_ok(|output| ArrayBox::from_vector(output.stderr.clone()))
        .into_raw()
}

#[no_mangle]
pub fn process_output_stderr_string(output: *mut ValueBox<Output>) -> *mut ValueBox<StringBox> {
    output
        .with_ref(|output| {
            String::from_utf8(output.stderr.clone())
                .map_err(|error| BoxerError::AnyError(Box::new(error)).into())
        })
        .map(|stdout| StringBox::from_string(stdout))
        .into_raw()
}

#[no_mangle]
pub fn process_output_stdout(output: *mut ValueBox<Output>) -> *mut ValueBox<ArrayBox<u8>> {
    output
        .with_ref_ok(|output| ArrayBox::from_vector(output.stdout.clone()))
        .into_raw()
}

#[no_mangle]
pub fn process_output_stdout_string(output: *mut ValueBox<Output>) -> *mut ValueBox<StringBox> {
    output
        .with_ref(|output| {
            String::from_utf8(output.stdout.clone())
                .map_err(|error| BoxerError::AnyError(Box::new(error)).into())
        })
        .map(|stdout| StringBox::from_string(stdout))
        .into_raw()
}

#[no_mangle]
pub fn process_output_drop(output: *mut ValueBox<Output>) {
    output.release();
}
