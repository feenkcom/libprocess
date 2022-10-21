use std::process::{ExitStatus, Output};

use array_box::ArrayBox;
use string_box::StringBox;
use value_box::{BoxerError, ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn process_output_status(output: *mut ValueBox<Output>) -> *mut ValueBox<ExitStatus> {
    output
        .to_ref()
        .map(|output| ValueBox::new(output.status.clone()).into_raw())
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub fn process_output_stderr(output_ptr: *mut ValueBox<Output>) -> *mut ValueBox<ArrayBox<u8>> {
    output_ptr.with_not_null_return(std::ptr::null_mut(), |output| {
        ValueBox::new(ArrayBox::from_vector(output.stderr.clone())).into_raw()
    })
}

#[no_mangle]
pub fn process_output_stderr_string(output_ptr: *mut ValueBox<Output>) -> *mut ValueBox<StringBox> {
    output_ptr.with_not_null_return(std::ptr::null_mut(), |output| {
        match String::from_utf8(output.stderr.clone()) {
            Ok(string) => ValueBox::new(StringBox::from_string(string)).into_raw(),
            Err(error) => {
                error!(
                    "[{}] Failed to convert stdout stderr to string: {:?}",
                    line!(),
                    error
                );
                std::ptr::null_mut()
            }
        }
    })
}

#[no_mangle]
pub fn process_output_stdout(output_ptr: *mut ValueBox<Output>) -> *mut ValueBox<ArrayBox<u8>> {
    output_ptr.with_not_null_return(std::ptr::null_mut(), |output| {
        ValueBox::new(ArrayBox::from_vector(output.stdout.clone())).into_raw()
    })
}

#[no_mangle]
pub fn process_output_stdout_string(output: *mut ValueBox<Output>) -> *mut ValueBox<StringBox> {
    output
        .to_ref()
        .and_then(|output| {
            String::from_utf8(output.stdout.clone())
                .map_err(|error| BoxerError::AnyError(Box::new(error)).into())
        })
        .map(|stdout| StringBox::from_string(stdout))
        .into_raw()
}

#[no_mangle]
pub fn process_output_drop(ptr: *mut ValueBox<Output>) {
    ptr.release();
}
