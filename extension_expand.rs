fn main() {
    ///
    /// An extension for use with the Deno JS runtime.
    /// To use it, provide it as an argument when instantiating your runtime:
    ///
    /// ```rust,ignore
    /// use deno_core::{ JsRuntime, RuntimeOptions };
    ///
    ///let mut extensions = vec![runjs::init_ops_and_esm()];
    /// let mut js_runtime = JsRuntime::new(RuntimeOptions {
    ///   extensions,
    ///   ..Default::default()
    /// });
    /// ```
    ///
    #[allow(non_camel_case_types)]
    pub struct runjs {}
    impl runjs {
        fn ext() -> ::deno_core::Extension {
            #[allow(unused_imports)]
            use ::deno_core::Op;
            ::deno_core::Extension {
                name: "runjs",
                deps: &[],
                js_files: {
                    const JS: &'static [::deno_core::ExtensionFileSource] = &[];
                    ::std::borrow::Cow::Borrowed(JS)
                },
                esm_files: {
                    const JS: &'static [::deno_core::ExtensionFileSource] = &[
                        ::deno_core::ExtensionFileSource::new("ext:runjs/src/runtime.js", {
                            const STR: ::deno_core::v8::OneByteConst = ::deno_core::FastStaticString::create_external_onebyte_const(
                                    "const { core } = Deno;\r\n\r\nfunction argsToMessage(...args) {\r\n  return args.map((arg) => JSON.stringify(arg)).join(\" \");\r\n}\r\n\r\nglobalThis.console = {\r\n  log: (...args) => {\r\n    core.print(`[out]: ${argsToMessage(...args)}\\n`, false);\r\n  },\r\n  error: (...args) => {\r\n    core.print(`[err]: ${argsToMessage(...args)}\\n`, true);\r\n  },\r\n};\r\n\r\nglobalThis.runjs = {\r\n  readFile: (path) => {\r\n    return core.ops.op_read_file(path);\r\n  },\r\n  writeFile: (path, contents) => {\r\n    return core.ops.op_write_file(path, contents);\r\n  },\r\n  removeFile: (path) => {\r\n    return core.ops.op_remove_file(path);\r\n  },\r\n  fetch: async (url) => {\r\n    return core.ops.op_fetch(url);\r\n  },\r\n};\r\n\r\nglobalThis.setTimeout = async (callback, delay) => {\r\n  core.ops.op_set_timeout(delay).thean(callback);\r\n};\r\n"
                                        .as_bytes(),
                                );
                            let s: &'static ::deno_core::v8::OneByteConst = &STR;
                            ::deno_core::FastStaticString::new(s)
                        }),
                    ];
                    ::std::borrow::Cow::Borrowed(JS)
                },
                lazy_loaded_esm_files: {
                    const JS: &'static [::deno_core::ExtensionFileSource] = &[];
                    ::std::borrow::Cow::Borrowed(JS)
                },
                esm_entry_point: {
                    const V: ::std::option::Option<&'static ::std::primitive::str> =
                        ::std::option::Option::Some("ext:runjs/src/runtime.js");
                    V
                },
                ops: ::std::borrow::Cow::Owned(::alloc::vec::Vec::new()),
                external_references: ::std::borrow::Cow::Borrowed(&[]),
                global_template_middleware: ::std::option::Option::None,
                global_object_middleware: ::std::option::Option::None,
                op_state_fn: ::std::option::Option::None,
                middleware_fn: ::std::option::Option::None,
                enabled: true,
            }
        }
        #[inline(always)]
        #[allow(unused_variables)]
        fn with_ops_fn(ext: &mut ::deno_core::Extension) {}
        #[inline(always)]
        #[allow(unused_variables)]
        fn with_state_and_middleware(ext: &mut ::deno_core::Extension) {}
        #[inline(always)]
        #[allow(unused_variables)]
        #[allow(clippy::redundant_closure_call)]
        fn with_customizer(ext: &mut ::deno_core::Extension) {}
        #[allow(dead_code)]
        /// Initialize this extension for runtime or snapshot creation. Use this
        /// function if the runtime or snapshot is not created from a (separate)
        /// snapshot, or that snapshot does not contain this extension. Otherwise
        /// use `init_ops()` instead.
        ///
        /// # Returns
        /// an Extension object that can be used during instantiation of a JsRuntime
        pub fn init_ops_and_esm() -> ::deno_core::Extension {
            let mut ext = Self::ext();
            Self::with_ops_fn(&mut ext);
            Self::with_state_and_middleware(&mut ext);
            Self::with_customizer(&mut ext);
            ext
        }
        #[allow(dead_code)]
        /// Initialize this extension for runtime or snapshot creation, excluding
        /// its JavaScript sources and evaluation. This is used when the runtime
        /// or snapshot is created from a (separate) snapshot which includes this
        /// extension in order to avoid evaluating the JavaScript twice.
        ///
        /// # Returns
        /// an Extension object that can be used during instantiation of a JsRuntime
        pub fn init_ops() -> ::deno_core::Extension {
            let mut ext = Self::ext();
            Self::with_ops_fn(&mut ext);
            Self::with_state_and_middleware(&mut ext);
            Self::with_customizer(&mut ext);
            ext.js_files = ::std::borrow::Cow::Borrowed(&[]);
            ext.esm_files = ::std::borrow::Cow::Borrowed(&[]);
            ext.esm_entry_point = ::std::option::Option::None;
            ext
        }
    }
    let out_dir = std::path::PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let snapshot_path = out_dir.join("RUNJS_SNAPSHOT.bin");
    let snapshot = deno_core::snapshot::create_snapshot(
        deno_core::snapshot::CreateSnapshotOptions {
            cargo_manifest_dir: "C:\\roll-your-own-javascript-runtime",
            startup_snapshot: None,
            skip_op_registration: false,
            extensions: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([runjs::init_ops_and_esm()]),
            ),
            with_runtime_cb: None,
            extension_transpiler: None,
        },
        None,
    )
    .unwrap();
    std::fs::write(snapshot_path, snapshot.output).unwrap();
}
