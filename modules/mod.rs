// This file is auto-generated!!
crate::cfg_v8!(
  mod r#loop;

  pub fn setup_bindings<'a, 's>(
    scope: &'a mut v8::HandleScope<'s, ()>,
  ) -> v8::Local<'s, v8::ObjectTemplate> {
    let global = v8::ObjectTemplate::new(scope);
    r#loop::init(scope, global);
    global
  }
);
crate::cfg_quickjs!(
  mod r#loop;

  pub fn setup_bindings(context: &quick_js::Context) {
    r#loop::init(context);
  }
);
