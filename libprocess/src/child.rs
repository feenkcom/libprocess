use crate::async_buffer::AsynchronousBuffer;
use crate::{AsyncOutput, SemaphoreSignaller};
use std::process::{Child, ChildStdin, ChildStdout, ExitStatus, Output};
use value_box::{BoxerError, ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

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
        .map(|stdout| AsynchronousBuffer::new(stdout, None))
        .map(|stdout| value_box!(stdout))
        .into_raw()
}

#[no_mangle]
pub fn process_child_take_asynchronous_stdout_with_semaphore(
    child: *mut ValueBox<Child>,
    semaphore_callback: extern "C" fn(usize),
    semaphore_index: usize,
) -> *mut ValueBox<AsynchronousBuffer> {
    child
        .with_mut(|child| {
            child
                .stdout
                .take()
                .ok_or_else(|| BoxerError::AnyError("There is no stdout in Child".into()))
        })
        .map(|stdout| {
            AsynchronousBuffer::new(
                stdout,
                Some(SemaphoreSignaller::new(semaphore_callback, semaphore_index)),
            )
        })
        .map(|stdout| value_box!(stdout))
        .into_raw()
}

#[no_mangle]
pub fn process_child_take_stdout(child: *mut ValueBox<Child>) -> *mut ValueBox<ChildStdout> {
    child
        .with_mut(|child| {
            child
                .stdout
                .take()
                .ok_or_else(|| BoxerError::AnyError("There is no stdout in Child".into()))
        })
        .map(|stdout| value_box!(stdout))
        .into_raw()
}

#[no_mangle]
pub fn process_child_take_stdin(child: *mut ValueBox<Child>) -> *mut ValueBox<ChildStdin> {
    child
        .with_mut(|child| {
            child
                .stdin
                .take()
                .ok_or_else(|| BoxerError::AnyError("There is no stdin in Child".into()))
        })
        .map(|stdin| value_box!(stdin))
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
        .map(|stderr| AsynchronousBuffer::new(stderr, None))
        .map(|stderr| value_box!(stderr))
        .into_raw()
}

#[no_mangle]
pub fn process_child_take_asynchronous_stderr_with_semaphore(
    child: *mut ValueBox<Child>,
    semaphore_callback: extern "C" fn(usize),
    semaphore_index: usize,
) -> *mut ValueBox<AsynchronousBuffer> {
    child
        .with_mut(|child| {
            child
                .stderr
                .take()
                .ok_or_else(|| BoxerError::AnyError("There is no stderr in Child".into()))
        })
        .map(|stderr| {
            AsynchronousBuffer::new(
                stderr,
                Some(SemaphoreSignaller::new(semaphore_callback, semaphore_index)),
            )
        })
        .map(|stderr| value_box!(stderr))
        .into_raw()
}

#[no_mangle]
pub fn process_child_wait(child: *mut ValueBox<Child>) -> *mut ValueBox<ExitStatus> {
    child
        .with_mut(|child| child.wait().map_err(|error| error.into()))
        .map(|status| value_box!(status))
        .into_raw()
}

#[no_mangle]
pub fn process_child_try_wait(child: *mut ValueBox<Child>) -> *mut ValueBox<ExitStatus> {
    child
        .with_mut(|child| child.try_wait().map_err(|error| error.into()))
        .map(|exit_status| {
            exit_status
                .map(|exit_status| value_box!(exit_status).into_raw())
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
        .map(|output| value_box!(output))
        .into_raw()
}

#[no_mangle]
pub fn process_child_async_wait_with_output(
    child: *mut ValueBox<Child>,
    semaphore_callback: extern "C" fn(usize),
    semaphore_index: usize,
) -> *mut ValueBox<AsyncOutput> {
    child
        .take_value()
        .and_then(|child| {
            AsyncOutput::waiting_for(
                child,
                SemaphoreSignaller::new(semaphore_callback, semaphore_index),
            )
            .map_err(|error| error.into())
            .map(|output| value_box!(output))
        })
        .into_raw()
}

#[no_mangle]
pub fn process_child_drop(ptr: *mut ValueBox<Child>) {
    ptr.release();
}
