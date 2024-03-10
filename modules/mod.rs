// This file is auto-generated!!
crate::cfg_v8!(
  mod core;
  mod io;

  pub fn setup_bindings<'a, 's>(
    scope: &'a mut v8::HandleScope<'s, ()>,
  ) -> v8::Local<'s, v8::ObjectTemplate> {
    let global = v8::ObjectTemplate::new(scope);
    core::init(scope, global);
    io::init(scope, global);
    global
  }
);

crate::cfg_quickjs!(
  mod core;
  mod io;

  pub fn setup_bindings(context: *mut quickjs_ng_sys::JSContext) {
    core::init(context);
    io::init(context);
  }
);

crate::cfg_jsc!(
  mod core;
  mod io;

  pub fn setup_bindings(context: rusty_jsc_sys::JSContextRef) {
    let global = unsafe { rusty_jsc_sys::JSContextGetGlobalObject(context) };
    core::init(global, context);
    io::init(global, context);
  }
);
