use boxer::string::BoxerString;
use boxer::{ReturnBoxerResult, ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use std::fs::OpenOptions;
use std::io::ErrorKind;
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
        .into_raw()
}

#[no_mangle]
pub fn process_stdio_drop(stdio: &mut *mut ValueBox<Stdio>) {
    stdio.drop();
}
