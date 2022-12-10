// This file is generated!!
mod r#loop;

fn setup_bindings<'a, 's>(
  scope: &'a mut v8::HandleScope<'s, ()>,
) -> v8::Local<'s, v8::ObjectTemplate> {
  let global = v8::ObjectTemplate::new(scope);
  r#loop::init(scope, global);
  global
}
