use crate::{bindings::set_function_to, fs::open_sync};
use calloop::{EventLoop, LoopHandle, LoopSignal};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

pub trait JsFuture {
    fn run(&mut self, scope: &mut v8::HandleScope);
}

pub struct JsRuntime {
    isolate: v8::OwnedIsolate,
    pub handle: Arc<calloop::EventLoop<'static, LoopSignal>>,
}

pub struct JsRuntimeState {
    pub global_context: v8::Global<v8::Context>,
    pub handle: LoopHandle<'static, LoopSignal>,
    pub pending_futures: Vec<Box<dyn JsFuture>>,
}

impl JsRuntime {
    /// Create a new instance of the JavaScript runtime.
    pub fn new() -> Self {
        // Create a new V8 isolate
        let mut isolate = v8::Isolate::new(v8::CreateParams::default());

        let context = {
            let scope = &mut v8::HandleScope::new(&mut *isolate);
            let context = Self::context_init(scope);
            v8::Global::new(scope, context)
        };
        let handle = Arc::new(EventLoop::try_new().expect("Failed to initialize the event loop!"));
        let state = Rc::new(RefCell::new(JsRuntimeState {
            global_context: context,
            handle: handle.handle().clone(),
            pending_futures: vec![],
        }));
        isolate.set_slot(state);

        Self { isolate, handle }
    }

    /// Initialize the global context.
    fn context_init<'s>(scope: &mut v8::HandleScope<'s, ()>) -> v8::Global<v8::Context> {
        let scope = &mut v8::EscapableHandleScope::new(scope);
        let context = v8::Context::new(scope, Default::default());
        let global = context.global(scope);
        let scope = &mut v8::ContextScope::new(scope, context);

        set_function_to(
            scope,
            global,
            "print",
            |scope: &mut v8::HandleScope,
             args: v8::FunctionCallbackArguments,
             mut _rv: v8::ReturnValue| {
                let value = args.get(0).to_rust_string_lossy(scope);
                println!("{}", value);
            },
        );
        set_function_to(scope, global, "openSync", open_sync);
        v8::Global::new(scope, context)
    }

    pub async fn run_script(&mut self, file_path: &str) {
        // Read JavaScript code
        let js_code = tokio::fs::read_to_string(file_path)
            .await
            .expect("Failed to read the JavaScript file");

        let state = self
            .isolate
            .get_slot::<Rc<RefCell<JsRuntimeState>>>()
            .expect("Failed to retrieve runtime state")
            .clone();

        let global_context = state.borrow().global_context.clone();

        let handle_scope = &mut v8::HandleScope::with_context(&mut self.isolate, global_context);

        let code = v8::String::new(handle_scope, &js_code).unwrap();
        let script = v8::Script::compile(handle_scope, code, None).unwrap();
        let result = script.run(handle_scope).unwrap();

        let result_str = result.to_string(handle_scope).unwrap();
        println!(
            "Script result: {}",
            result_str.to_rust_string_lossy(handle_scope)
        );
    }
}
