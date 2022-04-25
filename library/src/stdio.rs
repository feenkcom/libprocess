use boxer::string::BoxerString;
use boxer::{ReturnBoxerResult, ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use std::fs::OpenOptions;
use std::process::Stdio;

#[no_mangle]
pub fn process_stdio_null() -> *mut ValueBox<Stdio> {
    ValueBox::new(Stdio::null()).into_raw()
}

#[no_mangle]
pub fn process_stdio_inherit() -> *mut ValueBox<Stdio> {
    ValueBox::new(Stdio::inherit()).into_raw()
}

#[no_mangle]
pub fn process_stdio_piped() -> *mut ValueBox<Stdio> {
    ValueBox::new(Stdio::piped()).into_raw()
}

#[no_mangle]
pub fn process_stdio_file(
    path: *mut ValueBox<BoxerString>,
    create: bool,
    append: bool,
    truncate: bool,
) -> *mut ValueBox<Stdio> {
    path.to_ref()
        .and_then(|path| {
            OpenOptions::new()
                .write(true)
                .create(create)
                .append(append)
                .truncate(truncate)
                .open(path.as_str())
                .map_err(|error| error.into())
                .map(|file| file.into())
        })
        .into_raw()
}

#[no_mangle]
pub fn process_stdio_drop(stdio: &mut *mut ValueBox<Stdio>) {
    stdio.drop();
}
