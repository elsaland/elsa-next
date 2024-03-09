use crate::modules;

pub trait AbstractRuntime {
  type Value<'s>;
  type Context<'s>;

  fn init() -> Self;
  fn setup_bindings<'s>(&mut self);
  fn eval(&mut self, source: &str);
}

cfg_v8! {
  pub struct Runtime {
    isolate: v8::OwnedIsolate,
    // module_map: Rc<RefCell<ModuleMap>>,
  }

  impl AbstractRuntime for Runtime {
    type Value<'s> = v8::Local<'s, v8::Value>;
    type Context<'s> = v8::HandleScope<'s, ()>;

    fn init() -> Self {
      setup();
      let isolate = v8::Isolate::new(Default::default());
      Self { isolate }
    }

    fn setup_bindings<'s>(&mut self) {
      let scope = &mut v8::HandleScope::new(&mut self.isolate);
      modules::setup_bindings(scope);
    }

    fn eval(&mut self, source: &str) {
      self.eval(source)
    }
  }

  fn setup() {
    v8::V8::set_flags_from_string("--turbo_fast_api_calls");
    v8::V8::initialize_platform(v8::new_default_platform(0, false).make_shared());
    v8::V8::initialize();
  }

  pub fn module_resolve_callback<'s>(
    context: v8::Local<'s, v8::Context>,
    specifier: v8::Local<'s, v8::String>,
    _import_assertions: v8::Local<'s, v8::FixedArray>,
    _referrer: v8::Local<'s, v8::Module>,
  ) -> Option<v8::Local<'s, v8::Module>> {
    // SAFETY: `CallbackScope` can be safely constructed from `Local<Context>`
    let scope = &mut unsafe { v8::CallbackScope::new(context) };

    let specifier = specifier.to_rust_string_lossy(scope);

    // WIP!!!
    let source = std::fs::read_to_string(&specifier).unwrap();
    let source = v8::String::new(scope, &source).unwrap();

    let name_str = v8::String::new(scope, &specifier).unwrap();
    let origin = module_origin(scope, name_str);

    let source = v8::script_compiler::Source::new(source, Some(&origin));
    let module = v8::script_compiler::compile_module(scope, source).unwrap();
    Some(module)
  }

  pub fn module_origin<'a>(
    s: &mut v8::HandleScope<'a>,
    resource_name: v8::Local<'a, v8::String>,
  ) -> v8::ScriptOrigin<'a> {
    let source_map_url = v8::String::new(s, "").unwrap();
    v8::ScriptOrigin::new(
      s,
      resource_name.into(),
      0,
      0,
      false,
      123,
      source_map_url.into(),
      true,
      false,
      true,
    )
  }

  impl Runtime {
    pub fn eval(&mut self, source: &str) {
      let scope = &mut v8::HandleScope::new(&mut self.isolate);

      let global = modules::setup_bindings(scope);
      let context = v8::Context::new_from_template(scope, global);

      let scope = &mut v8::ContextScope::new(scope, context);
      let source_str = v8::String::new(scope, source).unwrap();

      let name_str = v8::String::new(scope, "main.ts").unwrap();
      let origin = module_origin(scope, name_str);

      let source = v8::script_compiler::Source::new(source_str, Some(&origin));
      let try_catch = &mut v8::TryCatch::new(scope);

      let module = v8::script_compiler::compile_module(try_catch, source);

      if try_catch.has_caught() {
        let exception = try_catch.exception().unwrap();
        let exception_string = exception
          .to_string(try_catch)
          .unwrap()
          .to_rust_string_lossy(try_catch);

        panic!("{}", exception_string);
      }

      let module = module.unwrap();
      module.instantiate_module(try_catch, module_resolve_callback).unwrap();
      module.evaluate(try_catch).unwrap();

      if module.get_status() == v8::ModuleStatus::Errored {
        let exception = module.get_exception().to_rust_string_lossy(try_catch);

        panic!("{}", exception);
      }
    }
  }
}

cfg_jsc! {
  use rusty_jsc_sys::*;
  use std::ffi::CString;

  pub struct Runtime {
    vm: JSContextGroupRef,
    context: JSContextRef,
  }

  impl AbstractRuntime for Runtime {
    type Value<'s> = JSValueRef;
    type Context<'s> = JSContextRef;

    fn init() -> Self {
      let vm = unsafe { JSContextGroupCreate() };
      let context =
          unsafe { JSGlobalContextCreateInGroup(vm, std::ptr::null_mut()) };

      Self { context, vm }
    }

    fn setup_bindings<'s>(&mut self) {
      modules::setup_bindings(self.context);
    }

    fn eval(&mut self, source: &str) {
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

  impl Drop for Runtime {
    fn drop(&mut self) {
      unsafe {
        JSGlobalContextRelease(self.context as _);
        JSContextGroupRelease(self.vm);
      }
    }
  }
}

cfg_mozjs! {
  use mozjs::rust::{JSEngine, Runtime};
  use mozjs::jsapi::{CallArgs, JSAutoRealm, JSContext, OnNewGlobalHookOption, Value};

  pub struct Runtime {
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
  use quickjs_ng_sys as q;
  use std::ffi::CString;

  pub struct Runtime {
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

  impl AbstractRuntime for Runtime {
    type Value<'s> = q::JSValue;
    type Context<'s> = q::JSContext;

    fn init() -> Self {
      let runtime = unsafe { q::JS_NewRuntime() };
      let context = unsafe { q::JS_NewContext(runtime) };
      Self { context, runtime }
    }

    fn setup_bindings<'s>(&mut self) {
      modules::setup_bindings(self.context);
    }

    fn eval(&mut self, source: &str) {
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

cfg_hermes! {
  use libhermesabi_sys::*;

  pub struct Runtime {
    runtime: *mut HermesABIRuntime
  }

  impl AbstractRuntime for Runtime {
    type Value<'s> = ();
    type Context<'s> = ();

    fn init() -> Self {
        unsafe {
          let vtable = &*get_hermes_abi_vtable();

          let config = std::ptr::null_mut();
          let runtime = (vtable.make_hermes_runtime.unwrap())(config);
            Self { runtime }
        }
    }

    fn setup_bindings<'s>(&mut self) {
    }

    fn eval(&mut self, source: &str) {}
  }
}
