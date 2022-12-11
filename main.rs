#[cfg(feature = "typecheck")]
mod check;
mod macro_util;
mod modules;
#[cfg(feature = "typescript")]
mod strip;

cfg_v8! {
  struct Runtime {
    isolate: v8::OwnedIsolate,
  }

  fn setup() {
    v8::V8::set_flags_from_string("--turbo_fast_api_calls");
    v8::V8::initialize_platform(v8::new_default_platform(0, false).make_shared());
    v8::V8::initialize();
  }

  impl Runtime {
    pub fn new() -> Self {
      setup();
      let isolate = v8::Isolate::new(Default::default());
      Self { isolate }
    }

    pub fn eval(&mut self, source: &str) {
      let scope = &mut v8::HandleScope::new(&mut self.isolate);

      let global = modules::setup_bindings(scope);
      let context = v8::Context::new_from_template(scope, global);

      let scope = &mut v8::ContextScope::new(scope, context);
      let source = v8::String::new(scope, source).unwrap();

      let try_catch = &mut v8::TryCatch::new(scope);

      let script = v8::Script::compile(try_catch, source, None)
        .expect("failed to compile script");

      if script.run(try_catch).is_none() {
        let exception = try_catch.exception().unwrap();
        let exception_string = exception
          .to_string(try_catch)
          .unwrap()
          .to_rust_string_lossy(try_catch);

        panic!("{}", exception_string);
      }
    }
  }
}

cfg_jsc! {
  use rusty_jsc_sys::*;
  use std::ffi::CString;

  struct Runtime {
    vm: JSContextGroupRef,
    context: JSContextRef,
  }

  impl Drop for Runtime {
    fn drop(&mut self) {
      unsafe {
        JSGlobalContextRelease(self.context as _);
        JSContextGroupRelease(self.vm);
      }
    }
  }

  impl Runtime {
    pub fn new() -> Self {
      let vm = unsafe { JSContextGroupCreate() };
      let context =
          unsafe { JSGlobalContextCreateInGroup(vm, std::ptr::null_mut()) };

      modules::setup_bindings(context);
      Self { context, vm }
    }

    pub fn eval(&mut self, source: &str) {
      let source = CString::new(source.as_bytes()).unwrap();
      let source = unsafe { JSStringCreateWithUTF8CString(source.as_ptr()) };

      let this_object = std::ptr::null_mut();
      let source_url = std::ptr::null_mut();
      let mut exception: JSValueRef = std::ptr::null_mut();
      let value = unsafe {
          JSEvaluateScript(
              self.context,
              source,
              this_object,
              source_url,
              1,
              &mut exception,
          )
      };

      if unsafe { JSValueIsNull(self.context, value) } {
        panic!("JS exception: {:?}", exception)
      }
    }
  }
}

cfg_mozjs! {
  use mozjs::rust::{JSEngine, Runtime};
  use mozjs::jsapi::{CallArgs, JSAutoRealm, JSContext, OnNewGlobalHookOption, Value};

  struct Runtime {
    engine: JSEngine,
    runtime: Runtime,
  }

  impl Runtime {
    pub fn new() -> Self {
      let engine = JSEngine::init().unwrap();
      let runtime = Runtime::new(engine.handle());
      let context = runtime.cx();
      Self { engine, runtime }
    }

    pub fn eval(&mut self, source: &str) {
      todo!()
    }
  }
}

cfg_quickjs! {
  use libquickjs_sys as q;
  use std::ffi::CString;

  struct Runtime {
    runtime: *mut q::JSRuntime,
    context: *mut q::JSContext,
  }

  impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            q::JS_FreeContext(self.context);
            q::JS_FreeRuntime(self.runtime);
        }
    }
  }

  impl Runtime {
    pub fn new() -> Self {
      let runtime = unsafe { q::JS_NewRuntime() };
      let context = unsafe { q::JS_NewContext(runtime) };
      modules::setup_bindings(context);
      Self { context, runtime }
    }

    pub fn eval(&mut self, source: &str) {
      let code = CString::new(source).unwrap();
      let filename = CString::new("<eval>").unwrap();
      let value_raw = unsafe {
        q::JS_Eval(
            self.context,
            code.as_ptr(),
            source.len() as _,
            filename.as_ptr(),
            q::JS_EVAL_TYPE_GLOBAL as i32,
        )
      };
      unsafe { q::JS_FreeValue(self.context, value_raw) };
    }
  }
}

fn main() {
  let filename = std::env::args()
    .nth(1)
    .expect("Invalid invocation. Usage: crimson <filename>");

  #[cfg(feature = "typecheck")]
  let j = {
    let f = filename.clone();
    std::thread::spawn(move || check::check(&f))
  };

  #[cfg(feature = "typescript")]
  let source = strip::strip(&filename);

  #[cfg(not(feature = "typescript"))]
  #[cfg(not(feature = "typecheck"))]
  let source = std::fs::read_to_string(&filename).expect("failed to read file");

  #[cfg(feature = "typecheck")]
  j.join().unwrap();

  Runtime::new().eval(&source);
}
