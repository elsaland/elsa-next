// This file is auto-generated!!

#![allow(non_camel_case_types)]
#![allow(unused_variables)]

crate::cfg_v8! (
  use v8::MapFnTo;
  use v8::fast_api;
  mod r#impl;

  pub fn init<'a, 's>(
    scope: &'a mut v8::HandleScope<'s, ()>,
    global: v8::Local<'s, v8::ObjectTemplate>,
  ) {
    global.set(
      v8::String::new(scope, "add").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_add_.map_fn_to())
        .build_fast(scope, &add_, None)
        .into(),
    );
  }

  pub struct add_;
  impl fast_api::FastFunction for add_ {
    fn function(&self) -> *const std::ffi::c_void  {
      fast_add_ as *const _
    }
    fn args(&self) -> &'static [fast_api::Type] {
      &[ fast_api::Type::V8Value, fast_api::Type::Int32, fast_api::Type::Int32 ]
    }
    fn return_type(&self) -> fast_api::CType {
      fast_api::CType::Int32
    }
  }

  fn fast_add_(
    _: v8::Local<v8::Object>,
    p0: i32, p1: i32
  ) -> i32 {
    r#impl::add(p0 as _, p1 as _) as _
  }

  fn slow_add_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = serde_v8::from_v8(scope, args.get(0)).unwrap();
    let p1 = serde_v8::from_v8(scope, args.get(1)).unwrap();
    let result = r#impl::add(p0, p1);
    rv.set(serde_v8::to_v8(scope, result).unwrap());
  }
);

crate::cfg_quickjs!(
  mod r#impl;

  pub fn init(context: &quick_js::Context) {
    context.add_callback("add", &add_).unwrap();
  }
  fn add_(p0: i32, p1: i32) -> i32 {
    r#impl::add(p0 as _, p1 as _) as _
  }
);
