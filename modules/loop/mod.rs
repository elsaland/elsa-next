// This file is auto-generated!!

#![allow(non_camel_case_types)]
#![allow(unused_variables)]

crate::cfg_v8!(
  use v8::MapFnTo;
  use v8::fast_api;
  mod r#impl;

  pub fn init<'a, 's>(
    scope: &'a mut v8::HandleScope<'s, ()>,
    global: v8::Local<'s, v8::ObjectTemplate>,
  ) {
    global.set(
      v8::String::new(scope, "print").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_print_.map_fn_to())
        .build_fast(scope, &FAST_print, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "kqueue").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_kqueue_.map_fn_to())
        .build_fast(scope, &FAST_kqueue, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "kevent").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_kevent_.map_fn_to())
        .build_fast(scope, &FAST_kevent, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "socket").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_socket_.map_fn_to())
        .build_fast(scope, &FAST_socket, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "setsockopt").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_setsockopt_.map_fn_to())
        .build_fast(scope, &FAST_setsockopt, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "bind").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_bind_.map_fn_to())
        .build_fast(scope, &FAST_bind, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "listen").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_listen_.map_fn_to())
        .build_fast(scope, &FAST_listen, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "close").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_close_.map_fn_to())
        .build_fast(scope, &FAST_close, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "accept").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_accept_.map_fn_to())
        .build_fast(scope, &FAST_accept, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "send").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_send_.map_fn_to())
        .build_fast(scope, &FAST_send, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "recv").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_recv_.map_fn_to())
        .build_fast(scope, &FAST_recv, None, None, None)
        // .build(scope)
        .into(),
    );
  }

  pub struct print_;
  const FAST_print: fast_api::FastFunction = fast_api::FastFunction::new(
    &[fast_api::Type::V8Value, fast_api::Type::SeqOneByteString],
    fast_api::CType::Void,
    fast_print_ as *const _,
  );

  fn fast_print_(
    _: v8::Local<v8::Object>,
    p0: *const fast_api::FastApiOneByteString,
  ) -> () {
    unsafe {
      r#impl::print(unsafe { std::str::from_utf8_unchecked((*p0).as_bytes()) })
        as _
    }
  }

  fn slow_print_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0_tmp = args.get(0).to_rust_string_lossy(scope);
    let p0 = p0_tmp.as_ref();
    let result = unsafe { r#impl::print(p0) };
    rv.set(v8::undefined(scope).into());
  }

  pub struct kqueue_;
  const FAST_kqueue: fast_api::FastFunction = fast_api::FastFunction::new(
    &[fast_api::Type::V8Value],
    fast_api::CType::Int32,
    fast_kqueue_ as *const _,
  );

  fn fast_kqueue_(_: v8::Local<v8::Object>) -> i32 {
    unsafe { r#impl::kqueue() as _ }
  }

  fn slow_kqueue_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let result = unsafe { r#impl::kqueue() };
    rv.set(v8::Number::new(scope, result as _).into());
  }

  pub struct kevent_;
  const FAST_kevent: fast_api::FastFunction = fast_api::FastFunction::new(
    &[
      fast_api::Type::V8Value,
      fast_api::Type::Int32,
      fast_api::Type::Uint64,
      fast_api::Type::Int32,
      fast_api::Type::Uint64,
      fast_api::Type::Int32,
      fast_api::Type::Uint64,
    ],
    fast_api::CType::Int32,
    fast_kevent_ as *const _,
  );

  fn fast_kevent_(
    _: v8::Local<v8::Object>,
    p0: i32,
    p1: u64,
    p2: i32,
    p3: u64,
    p4: i32,
    p5: u64,
  ) -> i32 {
    unsafe {
      r#impl::kevent(
        p0 as _,
        p1 as *const u8 as _,
        p2 as _,
        p3 as *const u8 as _,
        p4 as _,
        p5 as *const u8 as _,
      ) as _
    }
  }

  fn slow_kevent_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = args.get(0).uint32_value(scope).unwrap() as _;
    let p1 = {
      let v = args.get(1);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(1)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(1).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let p2 = args.get(2).uint32_value(scope).unwrap() as _;
    let p3 = {
      let v = args.get(3);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(3)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(3).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let p4 = args.get(4).uint32_value(scope).unwrap() as _;
    let p5 = {
      let v = args.get(5);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(5)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(5).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let result = unsafe { r#impl::kevent(p0, p1, p2, p3, p4, p5) };
    rv.set(v8::Number::new(scope, result as _).into());
  }

  pub struct socket_;
  const FAST_socket: fast_api::FastFunction = fast_api::FastFunction::new(
    &[
      fast_api::Type::V8Value,
      fast_api::Type::Int32,
      fast_api::Type::Int32,
      fast_api::Type::Int32,
    ],
    fast_api::CType::Int32,
    fast_socket_ as *const _,
  );

  fn fast_socket_(_: v8::Local<v8::Object>, p0: i32, p1: i32, p2: i32) -> i32 {
    unsafe { r#impl::socket(p0 as _, p1 as _, p2 as _) as _ }
  }

  fn slow_socket_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = args.get(0).uint32_value(scope).unwrap() as _;
    let p1 = args.get(1).uint32_value(scope).unwrap() as _;
    let p2 = args.get(2).uint32_value(scope).unwrap() as _;
    let result = unsafe { r#impl::socket(p0, p1, p2) };
    rv.set(v8::Number::new(scope, result as _).into());
  }

  pub struct setsockopt_;
  const FAST_setsockopt: fast_api::FastFunction = fast_api::FastFunction::new(
    &[
      fast_api::Type::V8Value,
      fast_api::Type::Int32,
      fast_api::Type::Int32,
      fast_api::Type::Int32,
      fast_api::Type::Uint64,
      fast_api::Type::Int32,
    ],
    fast_api::CType::Int32,
    fast_setsockopt_ as *const _,
  );

  fn fast_setsockopt_(
    _: v8::Local<v8::Object>,
    p0: i32,
    p1: i32,
    p2: i32,
    p3: u64,
    p4: i32,
  ) -> i32 {
    unsafe {
      r#impl::setsockopt(
        p0 as _,
        p1 as _,
        p2 as _,
        p3 as *const u8 as _,
        p4 as _,
      ) as _
    }
  }

  fn slow_setsockopt_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = args.get(0).uint32_value(scope).unwrap() as _;
    let p1 = args.get(1).uint32_value(scope).unwrap() as _;
    let p2 = args.get(2).uint32_value(scope).unwrap() as _;
    let p3 = {
      let v = args.get(3);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(3)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(3).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let p4 = args.get(4).uint32_value(scope).unwrap() as _;
    let result = unsafe { r#impl::setsockopt(p0, p1, p2, p3, p4) };
    rv.set(v8::Number::new(scope, result as _).into());
  }

  pub struct bind_;
  const FAST_bind: fast_api::FastFunction = fast_api::FastFunction::new(
    &[
      fast_api::Type::V8Value,
      fast_api::Type::Int32,
      fast_api::Type::Uint64,
      fast_api::Type::Int32,
    ],
    fast_api::CType::Int32,
    fast_bind_ as *const _,
  );

  fn fast_bind_(_: v8::Local<v8::Object>, p0: i32, p1: u64, p2: i32) -> i32 {
    unsafe { r#impl::bind(p0 as _, p1 as *const u8 as _, p2 as _) as _ }
  }

  fn slow_bind_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = args.get(0).uint32_value(scope).unwrap() as _;
    let p1 = {
      let v = args.get(1);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(1)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(1).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let p2 = args.get(2).uint32_value(scope).unwrap() as _;
    let result = unsafe { r#impl::bind(p0, p1, p2) };
    rv.set(v8::Number::new(scope, result as _).into());
  }

  pub struct listen_;
  const FAST_listen: fast_api::FastFunction = fast_api::FastFunction::new(
    &[
      fast_api::Type::V8Value,
      fast_api::Type::Int32,
      fast_api::Type::Int32,
    ],
    fast_api::CType::Int32,
    fast_listen_ as *const _,
  );

  fn fast_listen_(_: v8::Local<v8::Object>, p0: i32, p1: i32) -> i32 {
    unsafe { r#impl::listen(p0 as _, p1 as _) as _ }
  }

  fn slow_listen_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = args.get(0).uint32_value(scope).unwrap() as _;
    let p1 = args.get(1).uint32_value(scope).unwrap() as _;
    let result = unsafe { r#impl::listen(p0, p1) };
    rv.set(v8::Number::new(scope, result as _).into());
  }

  pub struct close_;
  const FAST_close: fast_api::FastFunction = fast_api::FastFunction::new(
    &[fast_api::Type::V8Value, fast_api::Type::Int32],
    fast_api::CType::Int32,
    fast_close_ as *const _,
  );

  fn fast_close_(_: v8::Local<v8::Object>, p0: i32) -> i32 {
    unsafe { r#impl::close(p0 as _) as _ }
  }

  fn slow_close_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = args.get(0).uint32_value(scope).unwrap() as _;
    let result = unsafe { r#impl::close(p0) };
    rv.set(v8::Number::new(scope, result as _).into());
  }

  pub struct accept_;
  const FAST_accept: fast_api::FastFunction = fast_api::FastFunction::new(
    &[
      fast_api::Type::V8Value,
      fast_api::Type::Int32,
      fast_api::Type::Uint64,
      fast_api::Type::Uint64,
    ],
    fast_api::CType::Int32,
    fast_accept_ as *const _,
  );

  fn fast_accept_(_: v8::Local<v8::Object>, p0: i32, p1: u64, p2: u64) -> i32 {
    unsafe {
      r#impl::accept(p0 as _, p1 as *const u8 as _, p2 as *const u8 as _) as _
    }
  }

  fn slow_accept_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = args.get(0).uint32_value(scope).unwrap() as _;
    let p1 = {
      let v = args.get(1);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(1)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(1).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let p2 = {
      let v = args.get(2);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(2)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(2).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let result = unsafe { r#impl::accept(p0, p1, p2) };
    rv.set(v8::Number::new(scope, result as _).into());
  }

  pub struct send_;
  const FAST_send: fast_api::FastFunction = fast_api::FastFunction::new(
    &[
      fast_api::Type::V8Value,
      fast_api::Type::Int32,
      fast_api::Type::TypedArray(fast_api::CType::Uint8),
      fast_api::Type::Int32,
      fast_api::Type::Int32,
    ],
    fast_api::CType::Int32,
    fast_send_ as *const _,
  );

  fn fast_send_(
    _: v8::Local<v8::Object>,
    p0: i32,
    p1: *const fast_api::FastApiTypedArray<u8>,
    p2: i32,
    p3: i32,
  ) -> i32 {
    unsafe {
      r#impl::send(
        p0 as _,
        (&*p1).get_storage_if_aligned().unwrap().as_mut_ptr() as _,
        p2 as _,
        p3 as _,
      ) as _
    }
  }

  fn slow_send_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = args.get(0).uint32_value(scope).unwrap() as _;
    let p1 = match v8::Local::<v8::ArrayBufferView>::try_from(args.get(1)) {
      Ok(view) => {
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      }
      Err(_) => {
        let i: u64 = args.get(1).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let p2 = args.get(2).uint32_value(scope).unwrap() as _;
    let p3 = args.get(3).uint32_value(scope).unwrap() as _;
    let result = unsafe { r#impl::send(p0, p1, p2, p3) };
    rv.set(v8::Number::new(scope, result as _).into());
  }

  pub struct recv_;
  const FAST_recv: fast_api::FastFunction = fast_api::FastFunction::new(
    &[
      fast_api::Type::V8Value,
      fast_api::Type::Int32,
      fast_api::Type::TypedArray(fast_api::CType::Uint8),
      fast_api::Type::Int32,
      fast_api::Type::Int32,
    ],
    fast_api::CType::Int32,
    fast_recv_ as *const _,
  );

  fn fast_recv_(
    _: v8::Local<v8::Object>,
    p0: i32,
    p1: *const fast_api::FastApiTypedArray<u8>,
    p2: i32,
    p3: i32,
  ) -> i32 {
    unsafe {
      r#impl::recv(
        p0 as _,
        (&*p1).get_storage_if_aligned().unwrap().as_mut_ptr() as _,
        p2 as _,
        p3 as _,
      ) as _
    }
  }

  fn slow_recv_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = args.get(0).uint32_value(scope).unwrap() as _;
    let p1 = match v8::Local::<v8::ArrayBufferView>::try_from(args.get(1)) {
      Ok(view) => {
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      }
      Err(_) => {
        let i: u64 = args.get(1).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let p2 = args.get(2).uint32_value(scope).unwrap() as _;
    let p3 = args.get(3).uint32_value(scope).unwrap() as _;
    let result = unsafe { r#impl::recv(p0, p1, p2, p3) };
    rv.set(v8::Number::new(scope, result as _).into());
  }
);

crate::cfg_quickjs!(
  mod r#impl;
  use libquickjs_sys as q;

  pub fn init(context: *mut q::JSContext) {
    let global_raw = unsafe { q::JS_GetGlobalObject(context) };
    let data = Box::into_raw(Box::new(q::JSValue {
      u: q::JSValueUnion { int32: 0 },
      tag: q::JS_TAG_NULL as _,
    }));

    let f =
      unsafe { q::JS_NewCFunctionData(context, Some(print_), 1, 0, 1, data) };
    if unsafe {
      q::JS_SetPropertyStr(context, global_raw, "print ".as_ptr() as _, f)
    } < 0
    {
      panic!("failed to set property")
    }
    let f =
      unsafe { q::JS_NewCFunctionData(context, Some(kqueue_), 0, 0, 1, data) };
    if unsafe {
      q::JS_SetPropertyStr(context, global_raw, "kqueue ".as_ptr() as _, f)
    } < 0
    {
      panic!("failed to set property")
    }
    let f =
      unsafe { q::JS_NewCFunctionData(context, Some(kevent_), 6, 0, 1, data) };
    if unsafe {
      q::JS_SetPropertyStr(context, global_raw, "kevent ".as_ptr() as _, f)
    } < 0
    {
      panic!("failed to set property")
    }
    let f =
      unsafe { q::JS_NewCFunctionData(context, Some(socket_), 3, 0, 1, data) };
    if unsafe {
      q::JS_SetPropertyStr(context, global_raw, "socket ".as_ptr() as _, f)
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(context, Some(setsockopt_), 5, 0, 1, data)
    };
    if unsafe {
      q::JS_SetPropertyStr(context, global_raw, "setsockopt ".as_ptr() as _, f)
    } < 0
    {
      panic!("failed to set property")
    }
    let f =
      unsafe { q::JS_NewCFunctionData(context, Some(bind_), 3, 0, 1, data) };
    if unsafe {
      q::JS_SetPropertyStr(context, global_raw, "bind ".as_ptr() as _, f)
    } < 0
    {
      panic!("failed to set property")
    }
    let f =
      unsafe { q::JS_NewCFunctionData(context, Some(listen_), 2, 0, 1, data) };
    if unsafe {
      q::JS_SetPropertyStr(context, global_raw, "listen ".as_ptr() as _, f)
    } < 0
    {
      panic!("failed to set property")
    }
    let f =
      unsafe { q::JS_NewCFunctionData(context, Some(close_), 1, 0, 1, data) };
    if unsafe {
      q::JS_SetPropertyStr(context, global_raw, "close ".as_ptr() as _, f)
    } < 0
    {
      panic!("failed to set property")
    }
    let f =
      unsafe { q::JS_NewCFunctionData(context, Some(accept_), 3, 0, 1, data) };
    if unsafe {
      q::JS_SetPropertyStr(context, global_raw, "accept ".as_ptr() as _, f)
    } < 0
    {
      panic!("failed to set property")
    }
    let f =
      unsafe { q::JS_NewCFunctionData(context, Some(send_), 4, 0, 1, data) };
    if unsafe {
      q::JS_SetPropertyStr(context, global_raw, "send ".as_ptr() as _, f)
    } < 0
    {
      panic!("failed to set property")
    }
    let f =
      unsafe { q::JS_NewCFunctionData(context, Some(recv_), 4, 0, 1, data) };
    if unsafe {
      q::JS_SetPropertyStr(context, global_raw, "recv ".as_ptr() as _, f)
    } < 0
    {
      panic!("failed to set property")
    }
  }
  unsafe extern "C" fn print_(
    _ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    assert!(argc == 1);
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let result = r#impl::print(p0 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: 0 },
      tag: q::JS_TAG_UNDEFINED as _,
    }
  }
  unsafe extern "C" fn kqueue_(
    _ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    assert!(argc == 0);

    let result = r#impl::kqueue();
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn kevent_(
    _ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    assert!(argc == 6);
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let p1 = compile_error!("TODO: implement");
    let p2 = (*(argv.offset(2 as _))).u.int32;
    let p3 = compile_error!("TODO: implement");
    let p4 = (*(argv.offset(4 as _))).u.int32;
    let p5 = compile_error!("TODO: implement");
    let result =
      r#impl::kevent(p0 as _, p1 as _, p2 as _, p3 as _, p4 as _, p5 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn socket_(
    _ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    assert!(argc == 3);
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let p1 = (*(argv.offset(1 as _))).u.int32;
    let p2 = (*(argv.offset(2 as _))).u.int32;
    let result = r#impl::socket(p0 as _, p1 as _, p2 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn setsockopt_(
    _ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    assert!(argc == 5);
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let p1 = (*(argv.offset(1 as _))).u.int32;
    let p2 = (*(argv.offset(2 as _))).u.int32;
    let p3 = compile_error!("TODO: implement");
    let p4 = (*(argv.offset(4 as _))).u.int32;
    let result =
      r#impl::setsockopt(p0 as _, p1 as _, p2 as _, p3 as _, p4 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn bind_(
    _ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    assert!(argc == 3);
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let p1 = compile_error!("TODO: implement");
    let p2 = (*(argv.offset(2 as _))).u.int32;
    let result = r#impl::bind(p0 as _, p1 as _, p2 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn listen_(
    _ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    assert!(argc == 2);
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let p1 = (*(argv.offset(1 as _))).u.int32;
    let result = r#impl::listen(p0 as _, p1 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn close_(
    _ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    assert!(argc == 1);
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let result = r#impl::close(p0 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn accept_(
    _ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    assert!(argc == 3);
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let p1 = compile_error!("TODO: implement");
    let p2 = compile_error!("TODO: implement");
    let result = r#impl::accept(p0 as _, p1 as _, p2 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn send_(
    _ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    assert!(argc == 4);
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let p1 = compile_error!("TODO: implement");
    let p2 = (*(argv.offset(2 as _))).u.int32;
    let p3 = (*(argv.offset(3 as _))).u.int32;
    let result = r#impl::send(p0 as _, p1 as _, p2 as _, p3 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn recv_(
    _ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    assert!(argc == 4);
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let p1 = compile_error!("TODO: implement");
    let p2 = (*(argv.offset(2 as _))).u.int32;
    let p3 = (*(argv.offset(3 as _))).u.int32;
    let result = r#impl::recv(p0 as _, p1 as _, p2 as _, p3 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
);

crate::cfg_jsc!(
  mod r#impl;
  use rusty_jsc_sys::*;

  pub fn init(obj: JSObjectRef, context: JSContextRef) {
    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(print_),
      )
    };
    let name = unsafe { JSStringCreateWithUTF8CString("print ".as_ptr() as _) };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(kqueue_),
      )
    };
    let name =
      unsafe { JSStringCreateWithUTF8CString("kqueue ".as_ptr() as _) };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(kevent_),
      )
    };
    let name =
      unsafe { JSStringCreateWithUTF8CString("kevent ".as_ptr() as _) };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(socket_),
      )
    };
    let name =
      unsafe { JSStringCreateWithUTF8CString("socket ".as_ptr() as _) };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(setsockopt_),
      )
    };
    let name =
      unsafe { JSStringCreateWithUTF8CString("setsockopt ".as_ptr() as _) };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(bind_),
      )
    };
    let name = unsafe { JSStringCreateWithUTF8CString("bind ".as_ptr() as _) };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(listen_),
      )
    };
    let name =
      unsafe { JSStringCreateWithUTF8CString("listen ".as_ptr() as _) };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(close_),
      )
    };
    let name = unsafe { JSStringCreateWithUTF8CString("close ".as_ptr() as _) };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(accept_),
      )
    };
    let name =
      unsafe { JSStringCreateWithUTF8CString("accept ".as_ptr() as _) };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(send_),
      )
    };
    let name = unsafe { JSStringCreateWithUTF8CString("send ".as_ptr() as _) };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(recv_),
      )
    };
    let name = unsafe { JSStringCreateWithUTF8CString("recv ".as_ptr() as _) };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }
  }
  unsafe extern "C" fn print_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 1, "print expects atleast 1 arguments");
    let tmp_0 = {
      let string =
        JSValueToStringCopy(ctx, *(arguments.offset(0) as *mut _), exception);
      let size = JSStringGetMaximumUTF8CStringSize(string);
      let mut buffer = vec![0u8; size as _];

      let size =
        JSStringGetUTF8CString(string, buffer.as_mut_ptr() as _, size as _);
      buffer.set_len(size as _);
      String::from_utf8_unchecked(buffer)
    };

    let p0 = tmp_0.as_ref();
    let result = r#impl::print(p0 as _);
    JSValueMakeUndefined(ctx)
  }
  unsafe extern "C" fn kqueue_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 0, "kqueue expects atleast 0 arguments");

    let result = r#impl::kqueue();
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn kevent_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 6, "kevent expects atleast 6 arguments");
    let p0 = JSValueToNumber(ctx, *(arguments.offset(0 as _)), exception);
    let p1 = if JSValueIsObject(ctx, *(arguments.offset(1) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(1) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(1) as *mut _), exception) as u64
        as *mut ()
    };
    let p2 = JSValueToNumber(ctx, *(arguments.offset(2 as _)), exception);
    let p3 = if JSValueIsObject(ctx, *(arguments.offset(3) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(3) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(3) as *mut _), exception) as u64
        as *mut ()
    };
    let p4 = JSValueToNumber(ctx, *(arguments.offset(4 as _)), exception);
    let p5 = if JSValueIsObject(ctx, *(arguments.offset(5) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(5) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(5) as *mut _), exception) as u64
        as *mut ()
    };
    let result =
      r#impl::kevent(p0 as _, p1 as _, p2 as _, p3 as _, p4 as _, p5 as _);
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn socket_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 3, "socket expects atleast 3 arguments");
    let p0 = JSValueToNumber(ctx, *(arguments.offset(0 as _)), exception);
    let p1 = JSValueToNumber(ctx, *(arguments.offset(1 as _)), exception);
    let p2 = JSValueToNumber(ctx, *(arguments.offset(2 as _)), exception);
    let result = r#impl::socket(p0 as _, p1 as _, p2 as _);
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn setsockopt_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 5, "setsockopt expects atleast 5 arguments");
    let p0 = JSValueToNumber(ctx, *(arguments.offset(0 as _)), exception);
    let p1 = JSValueToNumber(ctx, *(arguments.offset(1 as _)), exception);
    let p2 = JSValueToNumber(ctx, *(arguments.offset(2 as _)), exception);
    let p3 = if JSValueIsObject(ctx, *(arguments.offset(3) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(3) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(3) as *mut _), exception) as u64
        as *mut ()
    };
    let p4 = JSValueToNumber(ctx, *(arguments.offset(4 as _)), exception);
    let result =
      r#impl::setsockopt(p0 as _, p1 as _, p2 as _, p3 as _, p4 as _);
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn bind_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 3, "bind expects atleast 3 arguments");
    let p0 = JSValueToNumber(ctx, *(arguments.offset(0 as _)), exception);
    let p1 = if JSValueIsObject(ctx, *(arguments.offset(1) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(1) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(1) as *mut _), exception) as u64
        as *mut ()
    };
    let p2 = JSValueToNumber(ctx, *(arguments.offset(2 as _)), exception);
    let result = r#impl::bind(p0 as _, p1 as _, p2 as _);
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn listen_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 2, "listen expects atleast 2 arguments");
    let p0 = JSValueToNumber(ctx, *(arguments.offset(0 as _)), exception);
    let p1 = JSValueToNumber(ctx, *(arguments.offset(1 as _)), exception);
    let result = r#impl::listen(p0 as _, p1 as _);
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn close_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 1, "close expects atleast 1 arguments");
    let p0 = JSValueToNumber(ctx, *(arguments.offset(0 as _)), exception);
    let result = r#impl::close(p0 as _);
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn accept_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 3, "accept expects atleast 3 arguments");
    let p0 = JSValueToNumber(ctx, *(arguments.offset(0 as _)), exception);
    let p1 = if JSValueIsObject(ctx, *(arguments.offset(1) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(1) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(1) as *mut _), exception) as u64
        as *mut ()
    };
    let p2 = if JSValueIsObject(ctx, *(arguments.offset(2) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(2) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(2) as *mut _), exception) as u64
        as *mut ()
    };
    let result = r#impl::accept(p0 as _, p1 as _, p2 as _);
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn send_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 4, "send expects atleast 4 arguments");
    let p0 = JSValueToNumber(ctx, *(arguments.offset(0 as _)), exception);
    let p1 = JSObjectGetTypedArrayBytesPtr(
      ctx,
      *(arguments.offset(1 as _) as *mut _),
      exception,
    );
    let p2 = JSValueToNumber(ctx, *(arguments.offset(2 as _)), exception);
    let p3 = JSValueToNumber(ctx, *(arguments.offset(3 as _)), exception);
    let result = r#impl::send(p0 as _, p1 as _, p2 as _, p3 as _);
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn recv_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 4, "recv expects atleast 4 arguments");
    let p0 = JSValueToNumber(ctx, *(arguments.offset(0 as _)), exception);
    let p1 = JSObjectGetTypedArrayBytesPtr(
      ctx,
      *(arguments.offset(1 as _) as *mut _),
      exception,
    );
    let p2 = JSValueToNumber(ctx, *(arguments.offset(2 as _)), exception);
    let p3 = JSValueToNumber(ctx, *(arguments.offset(3 as _)), exception);
    let result = r#impl::recv(p0 as _, p1 as _, p2 as _, p3 as _);
    JSValueMakeNumber(ctx, result as _)
  }
);
