use std::process::{Child, Output};
use std::sync::Arc;
use std::{io, thread};

use parking_lot::Mutex;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

use crate::SemaphoreSignaller;

#[derive(Clone, Debug)]
pub struct AsyncOutput {
    output: Arc<Mutex<Option<io::Result<Output>>>>,
    semaphore_signaller: SemaphoreSignaller,
}

impl AsyncOutput {
    pub fn waiting_for(
        child: Child,
        semaphore_signaller: SemaphoreSignaller,
    ) -> Result<Self, io::Error> {
        let async_output = Self {
            output: Arc::new(Mutex::new(None)),
            semaphore_signaller,
        };

        let async_output_clone = async_output.clone();

        thread::Builder::new()
            .name("process_child_async_wait_with_output".into())
            .spawn(move || {
                let output = child.wait_with_output();
                async_output_clone.output.lock().replace(output);
                async_output_clone.semaphore_signaller.signal();
            })
            .map(|_| async_output)
    }

    pub fn take_output(&self) -> Option<io::Result<Output>> {
        self.output.lock().take()
    }
}

#[no_mangle]
pub fn process_async_output_take_output(
    async_output: *mut ValueBox<AsyncOutput>,
) -> *mut ValueBox<Output> {
    async_output
        .with_ref(|async_output| {
            async_output
                .take_output()
                .map_or(Ok(None), |v| v.map_err(|error| error.into()).map(Some))
                .map(|output| {
                    output.map_or_else(
                        || std::ptr::null_mut(),
                        |output| value_box!(output).into_raw(),
                    )
                })
        })
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub fn process_async_output_drop(ptr: *mut ValueBox<AsyncOutput>) {
    ptr.release();
}
