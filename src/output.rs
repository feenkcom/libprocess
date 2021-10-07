use boxer::array::BoxerArrayU8;
use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use std::ffi::CStr;
use std::process::{ExitStatus, Output};
use std::string::FromUtf8Error;

#[no_mangle]
pub fn process_output_status(output_ptr: *mut ValueBox<Output>) -> *mut ValueBox<ExitStatus> {
    output_ptr.with_not_null_return(std::ptr::null_mut(), |output| {
        ValueBox::new(output.status.clone()).into_raw()
    })
}

#[no_mangle]
pub fn process_output_stderr(output_ptr: *mut ValueBox<Output>) -> *mut ValueBox<BoxerArrayU8> {
    output_ptr.with_not_null_return(std::ptr::null_mut(), |output| {
        ValueBox::new(BoxerArrayU8::from_vector(output.stderr.clone())).into_raw()
    })
}

#[no_mangle]
pub fn process_output_stderr_string(
    output_ptr: *mut ValueBox<Output>,
) -> *mut ValueBox<BoxerString> {
    output_ptr.with_not_null_return(std::ptr::null_mut(), |output| {
        match String::from_utf8(output.stderr.clone()) {
            Ok(string) => ValueBox::new(BoxerString::from_string(string)).into_raw(),
            Err(error) => std::ptr::null_mut(),
        }
    })
}

#[no_mangle]
pub fn process_output_stdout(output_ptr: *mut ValueBox<Output>) -> *mut ValueBox<BoxerArrayU8> {
    output_ptr.with_not_null_return(std::ptr::null_mut(), |output| {
        ValueBox::new(BoxerArrayU8::from_vector(output.stdout.clone())).into_raw()
    })
}

#[no_mangle]
pub fn process_output_stdout_string(
    output_ptr: *mut ValueBox<Output>,
) -> *mut ValueBox<BoxerString> {
    output_ptr.with_not_null_return(std::ptr::null_mut(), |output| {
        match String::from_utf8(output.stdout.clone()) {
            Ok(string) => ValueBox::new(BoxerString::from_string(string)).into_raw(),
            Err(error) => std::ptr::null_mut(),
        }
    })
}

#[no_mangle]
pub fn process_output_drop(ptr: &mut *mut ValueBox<Output>) {
    ptr.drop();
}
