use crate::async_buffer::AsynchronousBuffer;
use std::process::{Child, ExitStatus, Output};
use value_box::{BoxerError, ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn process_child_kill(child: *mut ValueBox<Child>) -> bool {
    child
        .with_mut_ok(|child| child.kill().is_ok())
        .or_log(false)
}

#[no_mangle]
pub fn process_child_kill_with_signal(child: *mut ValueBox<Child>, signal: libc::c_int) -> bool {
    child.with_mut_ok(|child| {
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
    }).or_log(false)
}

#[no_mangle]
pub fn process_child_id(child: *mut ValueBox<Child>) -> u32 {
    child.with_ref_ok(|child| child.id()).or_log(0)
}

#[no_mangle]
pub fn process_child_is_terminated(child: *mut ValueBox<Child>) -> bool {
    child
        .with_mut(|child| child.try_wait().map_err(|error| error.into()))
        .map(|exit_status| exit_status.is_some())
        .or_log(true)
}

#[no_mangle]
pub fn process_child_take_asynchronous_stdout(
    child: *mut ValueBox<Child>,
) -> *mut ValueBox<AsynchronousBuffer> {
    child
        .with_mut(|child| {
            child
                .stdout
                .take()
                .ok_or_else(|| BoxerError::AnyError("There is no stdout in Child".into()))
        })
        .map(|stdout| AsynchronousBuffer::new(stdout))
        .into_raw()
}

#[no_mangle]
pub fn process_child_take_asynchronous_stderr(
    child: *mut ValueBox<Child>,
) -> *mut ValueBox<AsynchronousBuffer> {
    child
        .with_mut(|child| {
            child
                .stderr
                .take()
                .ok_or_else(|| BoxerError::AnyError("There is no stderr in Child".into()))
        })
        .map(|stdout| AsynchronousBuffer::new(stdout))
        .into_raw()
}

#[no_mangle]
pub fn process_child_wait(child: *mut ValueBox<Child>) -> *mut ValueBox<ExitStatus> {
    child
        .with_mut(|child| child.wait().map_err(|error| error.into()))
        .into_raw()
}

#[no_mangle]
pub fn process_child_try_wait(child: *mut ValueBox<Child>) -> *mut ValueBox<ExitStatus> {
    child
        .with_mut(|child| child.try_wait().map_err(|error| error.into()))
        .map(|exit_status| {
            exit_status
                .map(|exit_status| ValueBox::new(exit_status).into_raw())
                .unwrap_or(std::ptr::null_mut())
        })
        .or_log(std::ptr::null_mut())
}

/// Consumes the child
#[no_mangle]
pub fn process_child_wait_with_output(child: *mut ValueBox<Child>) -> *mut ValueBox<Output> {
    child
        .take_value()
        .and_then(|child| child.wait_with_output().map_err(|error| error.into()))
        .into_raw()
}

#[no_mangle]
pub fn process_child_drop(ptr: *mut ValueBox<Child>) {
    ptr.release();
}
