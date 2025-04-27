use std::ffi::c_void;

use v8::{self, HandleScope};

//Utility functions from https://github.com/aalykiot/dune/blob/main/src/bindings.rs

pub fn set_function_to(
    scope: &mut v8::HandleScope<'_>,
    target: v8::Local<v8::Object>,
    name: &'static str,
    callback: impl v8::MapFnTo<v8::FunctionCallback>,
) {
    let key = v8::String::new(scope, name).unwrap();
    let template = v8::FunctionTemplate::new(scope, callback);
    let val = template.get_function(scope).unwrap();

    target.set(scope, key.into(), val.into());
}

pub fn set_property_to<'s>(
    scope: &mut v8::HandleScope<'s>,
    target: v8::Local<v8::Object>,
    name: &'static str,
    value: v8::Local<v8::Value>,
) {
    let key = v8::String::new(scope, name).unwrap();
    target.set(scope, key.into(), value);
}

pub fn set_constant_to<'s>(
    scope: &mut v8::HandleScope<'s>,
    target: v8::Local<v8::Object>,
    name: &'static str,
    value: v8::Local<v8::Value>,
) {
    let key = v8::String::new(scope, name).unwrap();
    target.define_own_property(scope, key.into(), value, v8::PropertyAttribute::READ_ONLY);
}

pub fn create_object_for<'s>(
    scope: &mut v8::HandleScope<'s>,
    target: v8::Local<v8::Object>,
    name: &'static str,
) -> v8::Local<'s, v8::Object> {
    let key = v8::String::new(scope, name).unwrap();
    let object_template = v8::ObjectTemplate::new(scope);
    let value = object_template.new_instance(scope).unwrap();

    target.set(scope, key.into(), value.into());
    return value;
}

pub fn set_internal_ref<'s, T>(
    scope: &mut v8::HandleScope<'s>,
    target: v8::Local<v8::Object>,
    index: usize,
    data: T,
) {
    let box_ref = Box::new(data);
    let stored_item = Box::leak(box_ref) as *mut T as *mut c_void;
    let v8_external_field = v8::External::new(scope, stored_item);
    target.set_internal_field(index, v8_external_field.into());
}

pub fn get_internal_ref<'s, T>(
    scope: &mut v8::HandleScope<'s>,
    source: v8::Local<v8::Object>,
    index: usize,
) -> &'s mut T {
    let v8_ref = source.get_internal_field(scope, index).unwrap();
    let stored_item = unsafe { v8::Local::<v8::External>::cast_unchecked(v8_ref) };
    let stored_item = stored_item.value() as *mut T;
    unsafe { &mut *stored_item }
}

pub fn throw_exception(scope: &mut HandleScope, message: &str) {
    let message = v8::String::new(scope, message).unwrap();
    let exception = v8::Exception::error(scope, message);
    scope.throw_exception(exception);
}

pub fn throw_type_error(scope: &mut v8::HandleScope, message: &str) {
    let message = v8::String::new(scope, message).unwrap();
    let exception = v8::Exception::type_error(scope, message);
    scope.throw_exception(exception);
}
