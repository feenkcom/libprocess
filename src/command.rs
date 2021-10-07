use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use std::path::Path;
use std::process::{Child, Command, Output, Stdio};

#[no_mangle]
pub fn process_command_new(name_ptr: *mut ValueBox<BoxerString>) -> *mut ValueBox<Command> {
    name_ptr.with_not_null_return(std::ptr::null_mut(), |name| {
        ValueBox::new(Command::new(name.as_str())).into_raw()
    })
}

#[no_mangle]
pub fn process_command_drop(ptr: &mut *mut ValueBox<Command>) {
    ptr.drop();
}

#[no_mangle]
pub fn process_command_arg(
    command_ptr: *mut ValueBox<Command>,
    arg_ptr: *mut ValueBox<BoxerString>,
) {
    command_ptr.with_not_null(|command| {
        arg_ptr.with_not_null(|arg| {
            command.arg(arg.as_str());
        })
    })
}

#[no_mangle]
pub fn process_command_env(
    command_ptr: *mut ValueBox<Command>,
    key_ptr: *mut ValueBox<BoxerString>,
    value_ptr: *mut ValueBox<BoxerString>,
) {
    command_ptr.with_not_null(|command| {
        key_ptr.with_not_null(|key| {
            value_ptr.with_not_null(|value| {
                command.env(key.as_str(), value.as_str());
            })
        })
    })
}

#[no_mangle]
pub fn process_command_current_dir(
    command_ptr: *mut ValueBox<Command>,
    dir_prt: *mut ValueBox<BoxerString>,
) {
    command_ptr.with_not_null(|command| {
        dir_prt.with_not_null(|dir| {
            command.current_dir(Path::new(dir.as_str()));
        })
    })
}

#[no_mangle]
pub fn process_command_pipe_stdout(
    command_ptr: *mut ValueBox<Command>
) {
    command_ptr.with_not_null(|command| {
        command.stdout(Stdio::piped());
    })
}

#[no_mangle]
pub fn process_command_pipe_stderr(
    command_ptr: *mut ValueBox<Command>
) {
    command_ptr.with_not_null(|command| {
        command.stderr(Stdio::piped());
    })
}

#[no_mangle]
pub fn process_command_pipe_stdin(
    command_ptr: *mut ValueBox<Command>
) {
    command_ptr.with_not_null(|command| {
        command.stdin(Stdio::piped());
    })
}

#[no_mangle]
pub fn process_command_spawn(command_ptr: *mut ValueBox<Command>) -> *mut ValueBox<Child> {
    command_ptr.with_not_null_return(std::ptr::null_mut(), |command| {
        command.spawn().map_or(std::ptr::null_mut(), |child| {
            ValueBox::new(child).into_raw()
        })
    })
}

#[no_mangle]
pub fn process_command_output(command_ptr: *mut ValueBox<Command>) -> *mut ValueBox<Output> {
    command_ptr.with_not_null_return(std::ptr::null_mut(), |command| {
        command.output().map_or(std::ptr::null_mut(), |output| {
            ValueBox::new(output).into_raw()
        })
    })
}
