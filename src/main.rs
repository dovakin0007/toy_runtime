// mod bindings;
// mod fs;
// mod runtime;
// pub mod event_loop;
// use bindings::*;
// use fs::open_sync;
// use std::{cell::RefCell, rc::Rc, sync::Once};
// use v8;



// fn context_init<'s>(scope: &mut v8::HandleScope<'s, ()>) -> v8::Local<'s, v8::Context> {
//     // let version = v8::ObjectTemplate::new(context_scope);
//     let scope = &mut v8::EscapableHandleScope::new(scope);
//     let context = v8::Context::new(scope, Default::default());
//     let global = context.global(scope);
//     let scope = &mut v8::ContextScope::new(scope, context);
//     set_function_to(
//         scope,
//         global,
//         "print",
//         |scope: &mut v8::HandleScope,
//          args: v8::FunctionCallbackArguments,
//          mut _rv: v8::ReturnValue| {
//             let value = args.get(0).to_rust_string_lossy(scope);
//             println!("{}", value);
//         },
//     );
//     set_function_to(scope, global, "openSync", open_sync);

//     return scope.escape(context);
// }

// #[tokio::main]
// async fn main() {
//     static V8_INIT: Once = Once::new();
//     V8_INIT.call_once(|| {
//         let platform = v8::new_default_platform(0, false).make_shared();
//         v8::V8::initialize_platform(platform);
//         v8::V8::initialize();
//     });

//     {
//         let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

//         let context = {
//             let scope = &mut v8::HandleScope::new(&mut *isolate);
//             let context = context_init(scope);
//             v8::Global::new(scope, context)
//         };
//         isolate.set_slot(Rc::new(RefCell::new(runtime::RuntimeState { context })));

//         let handle_scope = &mut v8::HandleScope::with_context(
//             isolate,
//             isolate
//                 .get_slot::<Rc<RefCell<runtime::RuntimeState>>>()
//                 .unwrap()
//                 .clone()
//                 .borrow()
//                 .context
//                 .clone(),
//         );

//         let js_code =
//             std::fs::read_to_string("./test.js").expect("Failed to read the JavaScript file");

//         let code = v8::String::new(handle_scope, &js_code).unwrap();
//         let script = v8::Script::compile(handle_scope, code, None).unwrap();
//         let result = script.run(handle_scope).unwrap();

//         let result_str = result.to_string(handle_scope).unwrap();
//         println!(
//             "Script result: {}",
//             result_str.to_rust_string_lossy(handle_scope)
//         );
//     }

//     // Cleanup V8
//     unsafe {
//         v8::V8::dispose();
//     }
//     v8::V8::dispose_platform();
// }

use std::sync::Once;

use tokio;
use v8;
mod runtime;
mod fs;
mod bindings;
use runtime::*;

#[tokio::main]
async fn main() {
    // Initialize the V8 platform (required before using V8)
    static V8_INIT: Once = Once::new();
    V8_INIT.call_once(|| {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });

    // Create a new instance of JsRuntime
    let mut runtime = JsRuntime::new();

    // Path to the JavaScript file to execute
    let script_path = "test.js";

    // Run the JavaScript script
    runtime.run_script(script_path).await;

    unsafe {
        v8::V8::dispose();
    }
    v8::V8::dispose_platform();
}
