mod macro_util;
mod modules;

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

cfg_quickjs! {
  struct Runtime {
    context: quick_js::Context,
  }

  impl Runtime {
    pub fn new() -> Self {
      let context = quick_js::Context::new().expect("failed to create context");
      modules::setup_bindings(&context);
      Self { context }
    }

    pub fn eval(&mut self, source: &str) {
      self.context.eval(source).unwrap();
    }
  }
}

fn main() {
  let filename = std::env::args()
    .nth(1)
    .expect("Invalid invocation. Usage: crimson <filename>");

  let source = std::fs::read_to_string(filename).expect("Failed to read file");

  Runtime::new().eval(&source);
}
