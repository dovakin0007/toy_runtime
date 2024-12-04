use std::{fs::{File, OpenOptions}, os::windows::io::{AsRawHandle, FromRawHandle, RawHandle}};

use v8;

use crate::bindings::{set_constant_to, set_internal_ref, throw_exception};

pub fn open_sync(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let file_name_path = args.get(0).to_rust_string_lossy(scope);

    let binding = args.get(1).to_rust_string_lossy(scope);
    let flags = binding.as_str();
    let mut file_options = OpenOptions::new();
    let x = match flags {
        "r" => file_options.read(true).open(file_name_path),
        _ => file_options.read(true).open(file_name_path)
    };
    match x {
        Ok(file) => {
            #[cfg(target_os = "windows")]
            if cfg!(windows) {
                let file_ptr = file.as_raw_handle() as usize;
                let file_wrapper = v8::ObjectTemplate::new(scope);
                file_wrapper.set_internal_field_count(1);
                let file_wrapper = file_wrapper.new_instance(scope).unwrap();
                let fd = v8::Number::new(scope, file_ptr as f64);
                set_constant_to(scope, file_wrapper, "fd", fd.into());
                let file = unsafe { File::from_raw_handle(file_ptr as RawHandle) };
                set_internal_ref(scope, file_wrapper, 0, Some(file));
                rv.set(file_wrapper.into());
            }
        },
        Err(e) => {
            let error_message = e.to_string();
            throw_exception(scope, error_message.as_str());
        }
    }
}
