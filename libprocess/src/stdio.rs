use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Write;
use std::process::{ChildStdin, ChildStdout, Stdio};

use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

#[no_mangle]
pub fn process_stdio_null() -> *mut ValueBox<Stdio> {
    value_box!(Stdio::null()).into_raw()
}

#[no_mangle]
pub fn process_stdio_inherit() -> *mut ValueBox<Stdio> {
    value_box!(Stdio::inherit()).into_raw()
}

#[no_mangle]
pub fn process_stdio_piped() -> *mut ValueBox<Stdio> {
    value_box!(Stdio::piped()).into_raw()
}

#[no_mangle]
pub fn process_stdio_file(
    path: *mut ValueBox<StringBox>,
    create: bool,
    append: bool,
    truncate: bool,
) -> *mut ValueBox<Stdio> {
    path.with_ref(|path| {
        let mut options = OpenOptions::new();
        options
            .read(true)
            .write(true)
            .create(create)
            .append(append)
            .truncate(if append { false } else { truncate });
        options
            .open(path.as_str())
            .map_err(|error| {
                std::io::Error::new(
                    ErrorKind::Other,
                    format!(
                        "Failed to open file {} with options {:?} due to {}",
                        path.as_str(),
                        &options,
                        error
                    ),
                )
                .into()
            })
            .map(|file| file.into())
    })
    .map(|stdio| value_box!(stdio))
    .into_raw()
}

#[no_mangle]
pub fn process_stdio_from_child_stdout(stdout: *mut ValueBox<ChildStdout>) -> *mut ValueBox<Stdio> {
    stdout
        .take_value()
        .map(|stdout| value_box!(stdout.into()))
        .into_raw()
}

#[no_mangle]
pub fn process_child_stdin_write_string(
    stdin: *mut ValueBox<ChildStdin>,
    string: *mut ValueBox<StringBox>,
) {
    stdin
        .with_mut(|stdin| {
            string.with_ref(|string| {
                write!(stdin, "{}", string.as_str()).map_err(|error| error.into())
            })
        })
        .log();
}

#[no_mangle]
pub fn process_child_stdin_close(stdin: *mut ValueBox<ChildStdin>) {
    stdin.take_value().log();
}

#[no_mangle]
pub fn process_stdio_drop(stdio: *mut ValueBox<Stdio>) {
    stdio.release();
}
