use std::io::Read;
use std::sync::Arc;
use std::thread;

use array_box::ArrayBox;
use encoding_rs::{CoderResult, UTF_8};
use parking_lot::Mutex;
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

use crate::SemaphoreSignaller;

pub struct AsynchronousBuffer {
    buffer: Arc<Mutex<Vec<u8>>>,
    is_end_of_file: Arc<Mutex<bool>>,
}

impl AsynchronousBuffer {
    pub fn new<R>(mut stream: R, semaphore_signaller: Option<SemaphoreSignaller>) -> Self
    where
        R: Read + Send + 'static,
    {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let vec = buffer.clone();
        let is_end_of_file = Arc::new(Mutex::new(false));

        let is_end_of_file_clone = is_end_of_file.clone();
        let semaphore_signaller = Arc::new(Mutex::new(semaphore_signaller));

        thread::Builder::new()
            .name("child_stream_to_vec".into())
            .spawn(move || loop {
                let mut buf = [0];
                match stream.read(&mut buf) {
                    Err(err) => {
                        error!("[{}] Error reading from stream: {}", line!(), err);
                        break;
                    }
                    Ok(got) => {
                        if got == 0 {
                            break;
                        } else if got == 1 {
                            vec.lock().push(buf[0])
                        } else {
                            error!("[{}] Unexpected number of bytes: {}", line!(), got);
                            break;
                        }
                    }
                }

                let mut is_end_of_file_lock = is_end_of_file_clone.lock();
                let mut semaphore_signaller_lock = semaphore_signaller.lock();

                *is_end_of_file_lock = true;
                if let Some(signaller) = semaphore_signaller_lock.take() {
                    signaller.signal();
                }
            })
            .expect("!thread");
        AsynchronousBuffer {
            buffer,
            is_end_of_file,
        }
    }

    pub fn poll(&mut self) -> Vec<u8> {
        self.buffer.lock().drain(0..).collect()
    }

    pub fn poll_string(&mut self) -> String {
        let mut buffer = self.buffer.lock();

        let mut string = String::with_capacity(buffer.len());

        let mut decoder = UTF_8.new_decoder();
        let (result, length, _has_replacements) =
            decoder.decode_to_string(buffer.as_slice(), &mut string, false);

        buffer.drain(0..length);

        match result {
            CoderResult::InputEmpty => {}
            CoderResult::OutputFull => {}
        }

        string
    }

    pub fn is_end_of_file_reached(&self) -> bool {
        *self.is_end_of_file.lock()
    }
}

#[no_mangle]
pub fn process_async_buffer_poll(
    buffer: *mut ValueBox<AsynchronousBuffer>,
) -> *mut ValueBox<ArrayBox<u8>> {
    buffer
        .with_mut_ok(|buffer| ValueBox::new(ArrayBox::from_vector(buffer.poll())))
        .into_raw()
}

#[no_mangle]
pub fn process_async_buffer_poll_string(
    buffer: *mut ValueBox<AsynchronousBuffer>,
) -> *mut ValueBox<StringBox> {
    buffer
        .with_mut_ok(|buffer| ValueBox::new(StringBox::from_string(buffer.poll_string())))
        .into_raw()
}

#[no_mangle]
pub fn process_async_buffer_is_end_of_file_reached(
    buffer: *mut ValueBox<AsynchronousBuffer>,
) -> bool {
    buffer
        .with_mut_ok(|buffer| buffer.is_end_of_file_reached())
        .or_log(true)
}

#[no_mangle]
pub fn process_async_buffer_drop(buffer: *mut ValueBox<AsynchronousBuffer>) {
    buffer.release();
}
