use boxer::array::BoxerArrayU8;
use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use encoding_rs::{CoderResult, UTF_8};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct AsynchronousBuffer {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl AsynchronousBuffer {
    pub fn new<R>(mut stream: R) -> Self
    where
        R: Read + Send + 'static,
    {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let vec = buffer.clone();
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
                            vec.lock().expect("!lock").push(buf[0])
                        } else {
                            error!("[{}] Unexpected number of bytes: {}", line!(), got);
                            break;
                        }
                    }
                }
            })
            .expect("!thread");
        AsynchronousBuffer { buffer }
    }

    pub fn poll(&mut self) -> Vec<u8> {
        match self.buffer.lock() {
            Ok(mut buffer) => buffer.drain(0..).collect(),
            Err(error) => {
                error!("[{}] Failed to lock the buffer due to {:?}", line!(), error);
                vec![]
            }
        }
    }

    pub fn poll_string(&mut self) -> String {
        match self.buffer.lock() {
            Ok(mut buffer) => {
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
            Err(error) => {
                error!("[{}] Failed to lock the buffer due to {:?}", line!(), error);
                "".to_string()
            }
        }
    }
}

#[no_mangle]
pub fn process_async_buffer_poll(
    buffer_ptr: *mut ValueBox<AsynchronousBuffer>,
) -> *mut ValueBox<BoxerArrayU8> {
    buffer_ptr.with_not_null_return(std::ptr::null_mut(), |buffer| {
        ValueBox::new(BoxerArrayU8::from_vector(buffer.poll())).into_raw()
    })
}

#[no_mangle]
pub fn process_async_buffer_poll_string(
    buffer_ptr: *mut ValueBox<AsynchronousBuffer>,
) -> *mut ValueBox<BoxerString> {
    buffer_ptr.with_not_null_return(std::ptr::null_mut(), |buffer| {
        ValueBox::new(BoxerString::from_string(buffer.poll_string())).into_raw()
    })
}

#[no_mangle]
pub fn process_async_buffer_drop(ptr: &mut *mut ValueBox<AsynchronousBuffer>) {
    ptr.drop();
}
