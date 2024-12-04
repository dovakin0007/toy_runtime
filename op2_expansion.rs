#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_core::error::AnyError;
use deno_core::extension;
use deno_core::op2;
use deno_core::ModuleLoadResponse;
use deno_core::ModuleSourceCode;
use std::env;
use std::rc::Rc;
#[allow(non_camel_case_types)]
const fn op_read_file() -> ::deno_core::_ops::OpDecl {
    #[allow(non_camel_case_types)]
    struct op_read_file {
        _unconstructable: ::std::marker::PhantomData<()>,
    }
    impl ::deno_core::_ops::Op for op_read_file {
        const NAME: &'static str = "op_read_file";
        const DECL: ::deno_core::_ops::OpDecl = ::deno_core::_ops::OpDecl::new_internal_op2(
            {
                const LITERAL: &'static [u8] = "op_read_file".as_bytes();
                const STR: ::deno_core::v8::OneByteConst =
                    ::deno_core::FastStaticString::create_external_onebyte_const(LITERAL);
                let s: &'static ::deno_core::v8::OneByteConst = &STR;
                ("op_read_file", ::deno_core::FastStaticString::new(s))
            },
            true,
            false,
            2usize as u8,
            false,
            Self::v8_fn_ptr as _,
            Self::v8_fn_ptr_metrics as _,
            None,
            None,
            ::deno_core::OpMetadata {
                ..::deno_core::OpMetadata::default()
            },
        );
    }
    impl op_read_file {
        pub const fn name() -> &'static str {
            <Self as deno_core::_ops::Op>::NAME
        }
        #[inline(always)]
        fn slow_function_impl<'s>(info: &'s deno_core::v8::FunctionCallbackInfo) -> usize {
            #[cfg(debug_assertions)]
            let _reentrancy_check_guard =
                deno_core::_ops::reentrancy_check(&<Self as deno_core::_ops::Op>::DECL);
            let mut scope = unsafe { deno_core::v8::CallbackScope::new(info) };
            let mut rv = deno_core::v8::ReturnValue::from_function_callback_info(info);
            let args = deno_core::v8::FunctionCallbackArguments::from_function_callback_info(info);
            let opctx: &'s _ = unsafe {
                &*(deno_core::v8::Local::<deno_core::v8::External>::cast_unchecked(args.data())
                    .value() as *const deno_core::_ops::OpCtx)
            };
            let result = {
                let arg0 = args.get(1usize as i32);
                let arg0 = deno_core::_ops::to_string(&mut scope, &arg0);
                Self::call(arg0)
            };
            let promise_id = deno_core::_ops::to_i32_option(&args.get(0)).unwrap_or_default();
            if let Some(result) = deno_core::_ops::map_async_op_fallible(
                opctx,
                false,
                false,
                promise_id,
                result,
                |scope, result| deno_core::_ops::RustToV8Fallible::to_v8_fallible(result, scope),
            ) {
                match result {
                    Ok(result) => {
                        match deno_core::_ops::RustToV8Fallible::to_v8_fallible(result, &mut scope)
                        {
                            Ok(v) => rv.set(v),
                            Err(rv_err) => {
                                let msg = deno_core::v8::String::new(
                                    &mut scope,
                                    &::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(format_args!(
                                            "{0}",
                                            deno_core::anyhow::Error::from(rv_err)
                                        ));
                                        res
                                    }),
                                )
                                .unwrap();
                                let exc = deno_core::v8::Exception::type_error(&mut scope, msg);
                                scope.throw_exception(exc);
                                return 1;
                            }
                        }
                    }
                    Err(err) => {
                        let err = err.into();
                        let exception = deno_core::error::to_v8_error(
                            &mut scope,
                            opctx.get_error_class_fn,
                            &err,
                        );
                        scope.throw_exception(exception);
                        return 1;
                    }
                };
                return 0;
            }
            return 2;
        }
        extern "C" fn v8_fn_ptr<'s>(info: *const deno_core::v8::FunctionCallbackInfo) {
            let info: &'s _ = unsafe { &*info };
            Self::slow_function_impl(info);
        }
        extern "C" fn v8_fn_ptr_metrics<'s>(info: *const deno_core::v8::FunctionCallbackInfo) {
            let info: &'s _ = unsafe { &*info };
            let args = deno_core::v8::FunctionCallbackArguments::from_function_callback_info(info);
            let opctx: &'s _ = unsafe {
                &*(deno_core::v8::Local::<deno_core::v8::External>::cast_unchecked(args.data())
                    .value() as *const deno_core::_ops::OpCtx)
            };
            deno_core::_ops::dispatch_metrics_async(
                opctx,
                deno_core::_ops::OpMetricsEvent::Dispatched,
            );
            let res = Self::slow_function_impl(info);
            if res == 0 {
                deno_core::_ops::dispatch_metrics_async(
                    opctx,
                    deno_core::_ops::OpMetricsEvent::Completed,
                );
            } else if res == 1 {
                deno_core::_ops::dispatch_metrics_async(
                    opctx,
                    deno_core::_ops::OpMetricsEvent::Error,
                );
            }
        }
    }
    impl op_read_file {
        #[inline(always)]
        async fn call(path: String) -> Result<String, AnyError> {
            let contents = tokio::fs::read_to_string(path).await?;
            Ok(contents)
        }
    }
    <op_read_file as ::deno_core::_ops::Op>::DECL
}
#[allow(non_camel_case_types)]
const fn op_write_file() -> ::deno_core::_ops::OpDecl {
    #[allow(non_camel_case_types)]
    struct op_write_file {
        _unconstructable: ::std::marker::PhantomData<()>,
    }
    impl ::deno_core::_ops::Op for op_write_file {
        const NAME: &'static str = "op_write_file";
        const DECL: ::deno_core::_ops::OpDecl = ::deno_core::_ops::OpDecl::new_internal_op2(
            {
                const LITERAL: &'static [u8] = "op_write_file".as_bytes();
                const STR: ::deno_core::v8::OneByteConst =
                    ::deno_core::FastStaticString::create_external_onebyte_const(LITERAL);
                let s: &'static ::deno_core::v8::OneByteConst = &STR;
                ("op_write_file", ::deno_core::FastStaticString::new(s))
            },
            true,
            false,
            3usize as u8,
            false,
            Self::v8_fn_ptr as _,
            Self::v8_fn_ptr_metrics as _,
            None,
            None,
            ::deno_core::OpMetadata {
                ..::deno_core::OpMetadata::default()
            },
        );
    }
    impl op_write_file {
        pub const fn name() -> &'static str {
            <Self as deno_core::_ops::Op>::NAME
        }
        #[inline(always)]
        fn slow_function_impl<'s>(info: &'s deno_core::v8::FunctionCallbackInfo) -> usize {
            #[cfg(debug_assertions)]
            let _reentrancy_check_guard =
                deno_core::_ops::reentrancy_check(&<Self as deno_core::_ops::Op>::DECL);
            let mut scope = unsafe { deno_core::v8::CallbackScope::new(info) };
            let mut rv = deno_core::v8::ReturnValue::from_function_callback_info(info);
            let args = deno_core::v8::FunctionCallbackArguments::from_function_callback_info(info);
            let opctx: &'s _ = unsafe {
                &*(deno_core::v8::Local::<deno_core::v8::External>::cast_unchecked(args.data())
                    .value() as *const deno_core::_ops::OpCtx)
            };
            let result = {
                let arg0 = args.get(1usize as i32);
                let arg0 = deno_core::_ops::to_string(&mut scope, &arg0);
                let arg1 = args.get(2usize as i32);
                let arg1 = deno_core::_ops::to_string(&mut scope, &arg1);
                Self::call(arg0, arg1)
            };
            let promise_id = deno_core::_ops::to_i32_option(&args.get(0)).unwrap_or_default();
            if let Some(result) = deno_core::_ops::map_async_op_fallible(
                opctx,
                false,
                false,
                promise_id,
                result,
                |scope, result| Ok(deno_core::_ops::RustToV8::to_v8(result, scope)),
            ) {
                match result {
                    Ok(result) => deno_core::_ops::RustToV8RetVal::to_v8_rv(result, &mut rv),
                    Err(err) => {
                        let err = err.into();
                        let exception = deno_core::error::to_v8_error(
                            &mut scope,
                            opctx.get_error_class_fn,
                            &err,
                        );
                        scope.throw_exception(exception);
                        return 1;
                    }
                };
                return 0;
            }
            return 2;
        }
        extern "C" fn v8_fn_ptr<'s>(info: *const deno_core::v8::FunctionCallbackInfo) {
            let info: &'s _ = unsafe { &*info };
            Self::slow_function_impl(info);
        }
        extern "C" fn v8_fn_ptr_metrics<'s>(info: *const deno_core::v8::FunctionCallbackInfo) {
            let info: &'s _ = unsafe { &*info };
            let args = deno_core::v8::FunctionCallbackArguments::from_function_callback_info(info);
            let opctx: &'s _ = unsafe {
                &*(deno_core::v8::Local::<deno_core::v8::External>::cast_unchecked(args.data())
                    .value() as *const deno_core::_ops::OpCtx)
            };
            deno_core::_ops::dispatch_metrics_async(
                opctx,
                deno_core::_ops::OpMetricsEvent::Dispatched,
            );
            let res = Self::slow_function_impl(info);
            if res == 0 {
                deno_core::_ops::dispatch_metrics_async(
                    opctx,
                    deno_core::_ops::OpMetricsEvent::Completed,
                );
            } else if res == 1 {
                deno_core::_ops::dispatch_metrics_async(
                    opctx,
                    deno_core::_ops::OpMetricsEvent::Error,
                );
            }
        }
    }
    impl op_write_file {
        #[inline(always)]
        async fn call(path: String, contents: String) -> Result<(), AnyError> {
            tokio::fs::write(path, contents).await?;
            Ok(())
        }
    }
    <op_write_file as ::deno_core::_ops::Op>::DECL
}
#[allow(non_camel_case_types)]
const fn op_fetch() -> ::deno_core::_ops::OpDecl {
    #[allow(non_camel_case_types)]
    struct op_fetch {
        _unconstructable: ::std::marker::PhantomData<()>,
    }
    impl ::deno_core::_ops::Op for op_fetch {
        const NAME: &'static str = "op_fetch";
        const DECL: ::deno_core::_ops::OpDecl = ::deno_core::_ops::OpDecl::new_internal_op2(
            {
                const LITERAL: &'static [u8] = "op_fetch".as_bytes();
                const STR: ::deno_core::v8::OneByteConst =
                    ::deno_core::FastStaticString::create_external_onebyte_const(LITERAL);
                let s: &'static ::deno_core::v8::OneByteConst = &STR;
                ("op_fetch", ::deno_core::FastStaticString::new(s))
            },
            true,
            false,
            2usize as u8,
            false,
            Self::v8_fn_ptr as _,
            Self::v8_fn_ptr_metrics as _,
            None,
            None,
            ::deno_core::OpMetadata {
                ..::deno_core::OpMetadata::default()
            },
        );
    }
    impl op_fetch {
        pub const fn name() -> &'static str {
            <Self as deno_core::_ops::Op>::NAME
        }
        #[inline(always)]
        fn  <'s>(info: &'s deno_core::v8::FunctionCallbackInfo) -> usize {
            #[cfg(debug_assertions)]
            let _reentrancy_check_guard =
                deno_core::_ops::reentrancy_check(&<Self as deno_core::_ops::Op>::DECL);
            let mut scope = unsafe { deno_core::v8::CallbackScope::new(info) };
            let mut rv = deno_core::v8::ReturnValue::from_function_callback_info(info);
            let args = deno_core::v8::FunctionCallbackArguments::from_function_callback_info(info);
            let opctx: &'s _ = unsafe {
                &*(deno_core::v8::Local::<deno_core::v8::External>::cast_unchecked(args.data())
                    .value() as *const deno_core::_ops::OpCtx)
            };
            let result = {
                let arg0 = args.get(1usize as i32);
                let arg0 = deno_core::_ops::to_string(&mut scope, &arg0);
                Self::call(arg0)
            };
            let promise_id = deno_core::_ops::to_i32_option(&args.get(0)).unwrap_or_default();
            if let Some(result) = deno_core::_ops::map_async_op_fallible(
                opctx,
                false,
                false,
                promise_id,
                result,
                |scope, result| deno_core::_ops::RustToV8Fallible::to_v8_fallible(result, scope),
            ) {
                match result {
                    Ok(result) => {
                        match deno_core::_ops::RustToV8Fallible::to_v8_fallible(result, &mut scope)
                        {
                            Ok(v) => rv.set(v),
                            Err(rv_err) => {
                                let msg = deno_core::v8::String::new(
                                    &mut scope,
                                    &::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(format_args!(
                                            "{0}",
                                            deno_core::anyhow::Error::from(rv_err)
                                        ));
                                        res
                                    }),
                                )
                                .unwrap();
                                let exc = deno_core::v8::Exception::type_error(&mut scope, msg);
                                scope.throw_exception(exc);
                                return 1;
                            }
                        }
                    }
                    Err(err) => {
                        let err = err.into();
                        let exception = deno_core::error::to_v8_error(
                            &mut scope,
                            opctx.get_error_class_fn,
                            &err,
                        );
                        scope.throw_exception(exception);
                        return 1;
                    }
                };
                return 0;
            }
            return 2;
        }
        extern "C" fn v8_fn_ptr<'s>(info: *const deno_core::v8::FunctionCallbackInfo) {
            let info: &'s _ = unsafe { &*info };
            Self::slow_function_impl(info);
        }
        extern "C" fn v8_fn_ptr_metrics<'s>(info: *const deno_core::v8::FunctionCallbackInfo) {
            let info: &'s _ = unsafe { &*info };
            let args = deno_core::v8::FunctionCallbackArguments::from_function_callback_info(info);
            let opctx: &'s _ = unsafe {
                &*(deno_core::v8::Local::<deno_core::v8::External>::cast_unchecked(args.data())
                    .value() as *const deno_core::_ops::OpCtx)
            };
            deno_core::_ops::dispatch_metrics_async(
                opctx,
                deno_core::_ops::OpMetricsEvent::Dispatched,
            );
            let res = Self::slow_function_impl(info);
            if res == 0 {
                deno_core::_ops::dispatch_metrics_async(
                    opctx,
                    deno_core::_ops::OpMetricsEvent::Completed,
                );
            } else if res == 1 {
                deno_core::_ops::dispatch_metrics_async(
                    opctx,
                    deno_core::_ops::OpMetricsEvent::Error,
                );
            }
        }
    }
    impl op_fetch {
        #[inline(always)]
        async fn call(url: String) -> Result<String, AnyError> {
            let body = reqwest::get(url).await?.text().await?;
            Ok(body)
        }
    }
    <op_fetch as ::deno_core::_ops::Op>::DECL
}
#[allow(non_camel_case_types)]
const fn op_set_timeout() -> ::deno_core::_ops::OpDecl {
    #[allow(non_camel_case_types)]
    struct op_set_timeout {
        _unconstructable: ::std::marker::PhantomData<()>,
    }
    impl ::deno_core::_ops::Op for op_set_timeout {
        const NAME: &'static str = "op_set_timeout";
        const DECL: ::deno_core::_ops::OpDecl = ::deno_core::_ops::OpDecl::new_internal_op2(
            {
                const LITERAL: &'static [u8] = "op_set_timeout".as_bytes();
                const STR: ::deno_core::v8::OneByteConst =
                    ::deno_core::FastStaticString::create_external_onebyte_const(LITERAL);
                let s: &'static ::deno_core::v8::OneByteConst = &STR;
                ("op_set_timeout", ::deno_core::FastStaticString::new(s))
            },
            true,
            false,
            2usize as u8,
            false,
            Self::v8_fn_ptr as _,
            Self::v8_fn_ptr_metrics as _,
            None,
            None,
            ::deno_core::OpMetadata {
                ..::deno_core::OpMetadata::default()
            },
        );
    }
    impl op_set_timeout {
        pub const fn name() -> &'static str {
            <Self as deno_core::_ops::Op>::NAME
        }
        #[inline(always)]
        fn slow_function_impl<'s>(info: &'s deno_core::v8::FunctionCallbackInfo) -> usize {
            #[cfg(debug_assertions)]
            let _reentrancy_check_guard =
                deno_core::_ops::reentrancy_check(&<Self as deno_core::_ops::Op>::DECL);
            let mut rv = deno_core::v8::ReturnValue::from_function_callback_info(info);
            let args = deno_core::v8::FunctionCallbackArguments::from_function_callback_info(info);
            let opctx: &'s _ = unsafe {
                &*(deno_core::v8::Local::<deno_core::v8::External>::cast_unchecked(args.data())
                    .value() as *const deno_core::_ops::OpCtx)
            };
            let result = {
                let arg0 = args.get(1usize as i32);
                let Some(arg0) = deno_core::_ops::to_f64_option(&arg0) else {
                    let mut scope = unsafe { deno_core::v8::CallbackScope::new(info) };
                    let msg = deno_core::v8::String::new_from_one_byte(
                        &mut scope,
                        "expected f64".as_bytes(),
                        deno_core::v8::NewStringType::Normal,
                    )
                    .unwrap();
                    let exc = deno_core::v8::Exception::type_error(&mut scope, msg);
                    scope.throw_exception(exc);
                    return 1;
                };
                let arg0 = arg0 as _;
                Self::call(arg0)
            };
            let promise_id = deno_core::_ops::to_i32_option(&args.get(0)).unwrap_or_default();
            if let Some(result) = deno_core::_ops::map_async_op_fallible(
                opctx,
                false,
                false,
                promise_id,
                result,
                |scope, result| Ok(deno_core::_ops::RustToV8::to_v8(result, scope)),
            ) {
                match result {
                    Ok(result) => deno_core::_ops::RustToV8RetVal::to_v8_rv(result, &mut rv),
                    Err(err) => {
                        let mut scope = unsafe { deno_core::v8::CallbackScope::new(info) };
                        let err = err.into();
                        let exception = deno_core::error::to_v8_error(
                            &mut scope,
                            opctx.get_error_class_fn,
                            &err,
                        );
                        scope.throw_exception(exception);
                        return 1;
                    }
                };
                return 0;
            }
            return 2;
        }
        extern "C" fn v8_fn_ptr<'s>(info: *const deno_core::v8::FunctionCallbackInfo) {
            let info: &'s _ = unsafe { &*info };
            Self::slow_function_impl(info);
        }
        extern "C" fn v8_fn_ptr_metrics<'s>(info: *const deno_core::v8::FunctionCallbackInfo) {
            let info: &'s _ = unsafe { &*info };
            let args = deno_core::v8::FunctionCallbackArguments::from_function_callback_info(info);
            let opctx: &'s _ = unsafe {
                &*(deno_core::v8::Local::<deno_core::v8::External>::cast_unchecked(args.data())
                    .value() as *const deno_core::_ops::OpCtx)
            };
            deno_core::_ops::dispatch_metrics_async(
                opctx,
                deno_core::_ops::OpMetricsEvent::Dispatched,
            );
            let res = Self::slow_function_impl(info);
            if res == 0 {
                deno_core::_ops::dispatch_metrics_async(
                    opctx,
                    deno_core::_ops::OpMetricsEvent::Completed,
                );
            } else if res == 1 {
                deno_core::_ops::dispatch_metrics_async(
                    opctx,
                    deno_core::_ops::OpMetricsEvent::Error,
                );
            }
        }
    }
    impl op_set_timeout {
        #[inline(always)]
        async fn call(delay: f64) -> Result<(), AnyError> {
            tokio::time::sleep(std::time::Duration::from_millis(delay as u64)).await;
            Ok(())
        }
    }
    <op_set_timeout as ::deno_core::_ops::Op>::DECL
}
