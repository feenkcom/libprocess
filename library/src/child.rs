use crate::async_buffer::AsynchronousBuffer;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use std::process::{Child, ExitStatus, Output};

#[no_mangle]
pub fn process_child_kill(child_ptr: *mut ValueBox<Child>) -> bool {
    child_ptr.with_not_null_return(false, |child| child.kill().is_ok())
}

#[no_mangle]
pub fn process_child_kill_with_signal(
    child_ptr: *mut ValueBox<Child>,
    signal: libc::c_int,
) -> bool {
    child_ptr.with_not_null_return(false, |child| {
        #[cfg(target_os = "windows")]
        {
            child.kill().is_ok()
        }
        #[cfg(not(target_os = "windows"))]
        {
            let can_kill = match child.try_wait() {
                Ok(status) => status.is_none(),
                Err(error) => {
                    error!("[{}] Failed to get the exit status of a process with id {} due to {:?}. We will try to kill it anyway", line!(), child.id(), error);
                    true
                },
            };

            if can_kill {
                let kill_result = unsafe { libc::kill(child.id() as libc::pid_t, signal) };

                let result = (if kill_result == -1 {
                    Err(std::io::Error::last_os_error())
                } else {
                    Ok(kill_result)
                }).map(drop);

                match result {
                    Ok(_) => { true }
                    Err(error) => {
                        error!("[{}] Failed to kill a process with id {} due to {:?}. We will try to kill it anyway", line!(), child.id(), error);
                        false
                    }
                }

            } else {
                false
            }
        }
    })
}

#[no_mangle]
pub fn process_child_id(child_ptr: *mut ValueBox<Child>) -> u32 {
    child_ptr.with_not_null_return(0, |child| child.id())
}

#[no_mangle]
pub fn process_child_is_terminated(child_ptr: *mut ValueBox<Child>) -> bool {
    child_ptr.with_not_null_return(true, |child| match child.try_wait() {
        Ok(status) => status.is_some(),
        Err(_) => true,
    })
}

#[no_mangle]
pub fn process_child_take_asynchronous_stdout(
    child_ptr: *mut ValueBox<Child>,
) -> *mut ValueBox<AsynchronousBuffer> {
    child_ptr.with_not_null_return(std::ptr::null_mut(), |child| match child.stdout.take() {
        None => std::ptr::null_mut(),
        Some(stdout) => ValueBox::new(AsynchronousBuffer::new(stdout)).into_raw(),
    })
}

#[no_mangle]
pub fn process_child_take_asynchronous_stderr(
    child_ptr: *mut ValueBox<Child>,
) -> *mut ValueBox<AsynchronousBuffer> {
    child_ptr.with_not_null_return(std::ptr::null_mut(), |child| match child.stderr.take() {
        None => std::ptr::null_mut(),
        Some(stderr) => ValueBox::new(AsynchronousBuffer::new(stderr)).into_raw(),
    })
}

#[no_mangle]
pub fn process_child_wait(child_ptr: *mut ValueBox<Child>) -> *mut ValueBox<ExitStatus> {
    child_ptr.with_not_null_return(std::ptr::null_mut(), |child| {
        child.wait().map_or(std::ptr::null_mut(), |exit_status| {
            ValueBox::new(exit_status).into_raw()
        })
    })
}

#[no_mangle]
pub fn process_child_try_wait(child_ptr: *mut ValueBox<Child>) -> *mut ValueBox<ExitStatus> {
    child_ptr.with_not_null_return(std::ptr::null_mut(), |child| {
        child
            .try_wait()
            .map_or(std::ptr::null_mut(), |exit_status| {
                exit_status.map_or(std::ptr::null_mut(), |exit_status| {
                    ValueBox::new(exit_status).into_raw()
                })
            })
    })
}

/// Consumes the child
#[no_mangle]
pub fn process_child_wait_with_output(
    mut child_ptr: *mut ValueBox<Child>,
) -> *mut ValueBox<Output> {
    child_ptr.with_not_null_value_consumed_return(std::ptr::null_mut(), |child| {
        child
            .wait_with_output()
            .map_or(std::ptr::null_mut(), |output| {
                ValueBox::new(output).into_raw()
            })
    })
}

#[no_mangle]
pub fn process_child_drop(ptr: &mut *mut ValueBox<Child>) {
    ptr.drop();
}
