use std::path::Path;
use std::process::{Child, Command, ExitStatus, Output, Stdio};
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn process_command_new(program: *mut ValueBox<StringBox>) -> *mut ValueBox<Command> {
    program
        .with_ref_ok(|name| Command::new(name.as_str()))
        .into_raw()
}

#[no_mangle]
pub fn process_command_drop(command: *mut ValueBox<Command>) {
    command.release();
}

#[no_mangle]
pub fn process_command_arg(command: *mut ValueBox<Command>, arg: *mut ValueBox<StringBox>) {
    command
        .with_mut(|command| {
            arg.with_ref_ok(|arg| {
                command.arg(arg.as_str());
            })
        })
        .log();
}

#[no_mangle]
pub fn process_command_env(
    command: *mut ValueBox<Command>,
    key: *mut ValueBox<StringBox>,
    value: *mut ValueBox<StringBox>,
) {
    command
        .with_mut(|command| {
            key.with_ref(|key| {
                value.with_ref_ok(|value| {
                    command.env(key.as_str(), value.as_str());
                })
            })
        })
        .log();
}

#[no_mangle]
pub fn process_command_env_remove(command: *mut ValueBox<Command>, key: *mut ValueBox<StringBox>) {
    command
        .with_mut(|command| {
            key.with_ref_ok(|key| {
                command.env_remove(key.as_str());
            })
        })
        .log();
}

#[no_mangle]
pub fn process_command_env_clear(command: *mut ValueBox<Command>) {
    command
        .with_mut_ok(|command| {
            command.env_clear();
        })
        .log();
}

#[no_mangle]
pub fn process_command_current_dir(
    command: *mut ValueBox<Command>,
    current_dir: *mut ValueBox<StringBox>,
) {
    command
        .with_mut(|command| {
            current_dir.with_ref_ok(|dir| {
                command.current_dir(Path::new(dir.as_str()));
            })
        })
        .log();
}

#[no_mangle]
pub fn process_command_set_stdout(command: *mut ValueBox<Command>, stdio: *mut ValueBox<Stdio>) {
    command
        .with_mut(|command| {
            stdio.take_value().map(|stdio| {
                command.stdout(stdio);
            })
        })
        .log();
}

#[no_mangle]
pub fn process_command_set_stderr(command: *mut ValueBox<Command>, stdio: *mut ValueBox<Stdio>) {
    command
        .with_mut(|command| {
            stdio.take_value().map(|stdio| {
                command.stderr(stdio);
            })
        })
        .log();
}

#[no_mangle]
pub fn process_command_set_stdin(command: *mut ValueBox<Command>, stdio: *mut ValueBox<Stdio>) {
    command
        .with_mut(|command| {
            stdio.take_value().map(|stdio| {
                command.stdin(stdio);
            })
        })
        .log();
}

#[no_mangle]
pub fn process_command_spawn(command: *mut ValueBox<Command>) -> *mut ValueBox<Child> {
    command
        .with_mut(|command| command.spawn().map_err(|error| error.into()))
        .into_raw()
}

#[no_mangle]
pub fn process_command_output(command: *mut ValueBox<Command>) -> *mut ValueBox<Output> {
    command
        .with_mut(|command| command.output().map_err(|error| error.into()))
        .into_raw()
}

#[no_mangle]
pub fn process_command_status(command: *mut ValueBox<Command>) -> *mut ValueBox<ExitStatus> {
    command
        .with_mut(|command| command.status().map_err(|error| error.into()))
        .into_raw()
}

#[no_mangle]
#[cfg(target_os = "windows")]
pub fn process_windows_creation_flags(command: *mut ValueBox<Command>, flags: u32) {
    use std::os::windows::process::CommandExt;

    command
        .with_mut_ok(|command| {
            command.creation_flags(flags);
        })
        .log();
}

#[no_mangle]
#[cfg(not(target_os = "windows"))]
pub fn process_windows_creation_flags(_command: *mut ValueBox<Command>, _flags: u32) {
    warn!(
        "[{}] tried to set Windows-specific process creation flags on {}",
        line!(),
        std::env::consts::OS
    );
}
