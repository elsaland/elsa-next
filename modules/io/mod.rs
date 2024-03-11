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
      v8::String::new(scope, "io_uring_check_version")
        .unwrap()
        .into(),
      v8::FunctionTemplate::builder_raw(
        slow_io_uring_check_version_.map_fn_to(),
      )
      .build_fast(scope, &FAST_io_uring_check_version, None, None, None)
      // .build(scope)
      .into(),
    );
    global.set(
      v8::String::new(scope, "io_uring_queue_init")
        .unwrap()
        .into(),
      v8::FunctionTemplate::builder_raw(slow_io_uring_queue_init_.map_fn_to())
        .build_fast(scope, &FAST_io_uring_queue_init, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "size_of_io_uring").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_size_of_io_uring_.map_fn_to())
        .build_fast(scope, &FAST_size_of_io_uring, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "size_of_io_uring_cqe")
        .unwrap()
        .into(),
      v8::FunctionTemplate::builder_raw(slow_size_of_io_uring_cqe_.map_fn_to())
        .build_fast(scope, &FAST_size_of_io_uring_cqe, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "io_uring_get_sqe").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_io_uring_get_sqe_.map_fn_to())
        .build_fast(scope, &FAST_io_uring_get_sqe, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "io_uring_sqe_set_data")
        .unwrap()
        .into(),
      v8::FunctionTemplate::builder_raw(
        slow_io_uring_sqe_set_data_.map_fn_to(),
      )
      .build_fast(scope, &FAST_io_uring_sqe_set_data, None, None, None)
      // .build(scope)
      .into(),
    );
    global.set(
      v8::String::new(scope, "io_uring_cqe_get_data")
        .unwrap()
        .into(),
      v8::FunctionTemplate::builder_raw(
        slow_io_uring_cqe_get_data_.map_fn_to(),
      )
      .build_fast(scope, &FAST_io_uring_cqe_get_data, None, None, None)
      // .build(scope)
      .into(),
    );
    global.set(
      v8::String::new(scope, "io_uring_submit").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_io_uring_submit_.map_fn_to())
        .build_fast(scope, &FAST_io_uring_submit, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "io_uring_prep_accept")
        .unwrap()
        .into(),
      v8::FunctionTemplate::builder_raw(slow_io_uring_prep_accept_.map_fn_to())
        .build_fast(scope, &FAST_io_uring_prep_accept, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "io_uring_prep_readv")
        .unwrap()
        .into(),
      v8::FunctionTemplate::builder_raw(slow_io_uring_prep_readv_.map_fn_to())
        .build_fast(scope, &FAST_io_uring_prep_readv, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "io_uring_prep_writev")
        .unwrap()
        .into(),
      v8::FunctionTemplate::builder_raw(slow_io_uring_prep_writev_.map_fn_to())
        .build_fast(scope, &FAST_io_uring_prep_writev, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "io_uring_prep_send_zc")
        .unwrap()
        .into(),
      v8::FunctionTemplate::builder_raw(
        slow_io_uring_prep_send_zc_.map_fn_to(),
      )
      .build_fast(scope, &FAST_io_uring_prep_send_zc, None, None, None)
      // .build(scope)
      .into(),
    );
    global.set(
      v8::String::new(scope, "io_uring_wait_cqe").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_io_uring_wait_cqe_.map_fn_to())
        .build_fast(scope, &FAST_io_uring_wait_cqe, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "io_uring_cqe_seen").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_io_uring_cqe_seen_.map_fn_to())
        .build_fast(scope, &FAST_io_uring_cqe_seen, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "io_uring_wait_cqe2").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_io_uring_wait_cqe2_.map_fn_to())
        .build_fast(scope, &FAST_io_uring_wait_cqe2, None, None, None)
        // .build(scope)
        .into(),
    );
    global.set(
      v8::String::new(scope, "io_uring_cqe_create_data")
        .unwrap()
        .into(),
      v8::FunctionTemplate::builder_raw(
        slow_io_uring_cqe_create_data_.map_fn_to(),
      )
      .build_fast(scope, &FAST_io_uring_cqe_create_data, None, None, None)
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
  }

  pub struct io_uring_check_version_;
  const FAST_io_uring_check_version: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[
        fast_api::Type::V8Value,
        fast_api::Type::Int32,
        fast_api::Type::Int32,
      ],
      fast_api::CType::Int32,
      fast_io_uring_check_version_ as *const _,
    );

  fn fast_io_uring_check_version_(
    _: v8::Local<v8::Object>,
    p0: i32,
    p1: i32,
  ) -> i32 {
    unsafe { r#impl::io_uring_check_version(p0 as _, p1 as _) as _ }
  }

  fn slow_io_uring_check_version_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = args.get(0).uint32_value(scope).unwrap() as _;
    let p1 = args.get(1).uint32_value(scope).unwrap() as _;
    let result = unsafe { r#impl::io_uring_check_version(p0, p1) };
    rv.set(v8::Number::new(scope, result as u64 as _).into());
  }

  pub struct io_uring_queue_init_;
  const FAST_io_uring_queue_init: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[
        fast_api::Type::V8Value,
        fast_api::Type::Uint32,
        fast_api::Type::Uint64,
        fast_api::Type::Uint32,
      ],
      fast_api::CType::Int32,
      fast_io_uring_queue_init_ as *const _,
    );

  fn fast_io_uring_queue_init_(
    _: v8::Local<v8::Object>,
    p0: u32,
    p1: u64,
    p2: u32,
  ) -> i32 {
    unsafe {
      r#impl::io_uring_queue_init(p0 as _, p1 as *const u8 as _, p2 as _) as _
    }
  }

  fn slow_io_uring_queue_init_(
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
    let result = unsafe { r#impl::io_uring_queue_init(p0, p1, p2) };
    rv.set(v8::Number::new(scope, result as u64 as _).into());
  }

  pub struct size_of_io_uring_;
  const FAST_size_of_io_uring: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[fast_api::Type::V8Value],
      fast_api::CType::Uint64,
      fast_size_of_io_uring_ as *const _,
    );

  fn fast_size_of_io_uring_(_: v8::Local<v8::Object>) -> usize {
    unsafe { r#impl::size_of_io_uring() as _ }
  }

  fn slow_size_of_io_uring_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let result = unsafe { r#impl::size_of_io_uring() };
    rv.set(v8::Number::new(scope, result as u64 as _).into());
  }

  pub struct size_of_io_uring_cqe_;
  const FAST_size_of_io_uring_cqe: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[fast_api::Type::V8Value],
      fast_api::CType::Uint64,
      fast_size_of_io_uring_cqe_ as *const _,
    );

  fn fast_size_of_io_uring_cqe_(_: v8::Local<v8::Object>) -> usize {
    unsafe { r#impl::size_of_io_uring_cqe() as _ }
  }

  fn slow_size_of_io_uring_cqe_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let result = unsafe { r#impl::size_of_io_uring_cqe() };
    rv.set(v8::Number::new(scope, result as u64 as _).into());
  }

  pub struct io_uring_get_sqe_;
  const FAST_io_uring_get_sqe: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[fast_api::Type::V8Value, fast_api::Type::Uint64],
      fast_api::CType::Uint64,
      fast_io_uring_get_sqe_ as *const _,
    );

  fn fast_io_uring_get_sqe_(_: v8::Local<v8::Object>, p0: u64) -> u64 {
    unsafe { r#impl::io_uring_get_sqe(p0 as *const u8 as _) as _ }
  }

  fn slow_io_uring_get_sqe_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = {
      let v = args.get(0);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(0)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(0).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let result = unsafe { r#impl::io_uring_get_sqe(p0) };
    rv.set(v8::Number::new(scope, result as u64 as _).into());
  }

  pub struct io_uring_sqe_set_data_;
  const FAST_io_uring_sqe_set_data: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[
        fast_api::Type::V8Value,
        fast_api::Type::Uint64,
        fast_api::Type::Uint64,
      ],
      fast_api::CType::Void,
      fast_io_uring_sqe_set_data_ as *const _,
    );

  fn fast_io_uring_sqe_set_data_(
    _: v8::Local<v8::Object>,
    p0: u64,
    p1: u64,
  ) -> () {
    unsafe {
      r#impl::io_uring_sqe_set_data(p0 as *const u8 as _, p1 as *const u8 as _)
        as _
    }
  }

  fn slow_io_uring_sqe_set_data_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = {
      let v = args.get(0);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(0)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(0).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
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
    let result = unsafe { r#impl::io_uring_sqe_set_data(p0, p1) };
    rv.set(v8::undefined(scope).into());
  }

  pub struct io_uring_cqe_get_data_;
  const FAST_io_uring_cqe_get_data: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[fast_api::Type::V8Value, fast_api::Type::Uint64],
      fast_api::CType::Uint64,
      fast_io_uring_cqe_get_data_ as *const _,
    );

  fn fast_io_uring_cqe_get_data_(_: v8::Local<v8::Object>, p0: u64) -> usize {
    unsafe { r#impl::io_uring_cqe_get_data(p0 as *const u8 as _) as _ }
  }

  fn slow_io_uring_cqe_get_data_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = {
      let v = args.get(0);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(0)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(0).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let result = unsafe { r#impl::io_uring_cqe_get_data(p0) };
    rv.set(v8::Number::new(scope, result as u64 as _).into());
  }

  pub struct io_uring_submit_;
  const FAST_io_uring_submit: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[fast_api::Type::V8Value, fast_api::Type::Uint64],
      fast_api::CType::Int32,
      fast_io_uring_submit_ as *const _,
    );

  fn fast_io_uring_submit_(_: v8::Local<v8::Object>, p0: u64) -> i32 {
    unsafe { r#impl::io_uring_submit(p0 as *const u8 as _) as _ }
  }

  fn slow_io_uring_submit_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = {
      let v = args.get(0);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(0)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(0).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let result = unsafe { r#impl::io_uring_submit(p0) };
    rv.set(v8::Number::new(scope, result as u64 as _).into());
  }

  pub struct io_uring_prep_accept_;
  const FAST_io_uring_prep_accept: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[
        fast_api::Type::V8Value,
        fast_api::Type::Uint64,
        fast_api::Type::Int32,
        fast_api::Type::Uint64,
        fast_api::Type::Uint64,
        fast_api::Type::Int32,
      ],
      fast_api::CType::Void,
      fast_io_uring_prep_accept_ as *const _,
    );

  fn fast_io_uring_prep_accept_(
    _: v8::Local<v8::Object>,
    p0: u64,
    p1: i32,
    p2: u64,
    p3: u64,
    p4: i32,
  ) -> () {
    unsafe {
      r#impl::io_uring_prep_accept(
        p0 as *const u8 as _,
        p1 as _,
        p2 as *const u8 as _,
        p3 as *const u8 as _,
        p4 as _,
      ) as _
    }
  }

  fn slow_io_uring_prep_accept_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = {
      let v = args.get(0);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(0)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(0).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let p1 = args.get(1).uint32_value(scope).unwrap() as _;
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
    let result = unsafe { r#impl::io_uring_prep_accept(p0, p1, p2, p3, p4) };
    rv.set(v8::undefined(scope).into());
  }

  pub struct io_uring_prep_readv_;
  const FAST_io_uring_prep_readv: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[
        fast_api::Type::V8Value,
        fast_api::Type::Uint64,
        fast_api::Type::Int32,
        fast_api::Type::Uint64,
        fast_api::Type::Int32,
        fast_api::Type::Int64,
      ],
      fast_api::CType::Void,
      fast_io_uring_prep_readv_ as *const _,
    );

  fn fast_io_uring_prep_readv_(
    _: v8::Local<v8::Object>,
    p0: u64,
    p1: i32,
    p2: u64,
    p3: i32,
    p4: i64,
  ) -> () {
    unsafe {
      r#impl::io_uring_prep_readv(
        p0 as *const u8 as _,
        p1 as _,
        p2 as *const u8 as _,
        p3 as _,
        p4 as _,
      ) as _
    }
  }

  fn slow_io_uring_prep_readv_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = {
      let v = args.get(0);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(0)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(0).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let p1 = args.get(1).uint32_value(scope).unwrap() as _;
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
    let p3 = args.get(3).uint32_value(scope).unwrap() as _;
    let p4 = args.get(4).uint32_value(scope).unwrap() as _;
    let result = unsafe { r#impl::io_uring_prep_readv(p0, p1, p2, p3, p4) };
    rv.set(v8::undefined(scope).into());
  }

  pub struct io_uring_prep_writev_;
  const FAST_io_uring_prep_writev: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[
        fast_api::Type::V8Value,
        fast_api::Type::Uint64,
        fast_api::Type::Int32,
        fast_api::Type::Uint64,
        fast_api::Type::Int32,
        fast_api::Type::Int64,
      ],
      fast_api::CType::Void,
      fast_io_uring_prep_writev_ as *const _,
    );

  fn fast_io_uring_prep_writev_(
    _: v8::Local<v8::Object>,
    p0: u64,
    p1: i32,
    p2: u64,
    p3: i32,
    p4: i64,
  ) -> () {
    unsafe {
      r#impl::io_uring_prep_writev(
        p0 as *const u8 as _,
        p1 as _,
        p2 as *const u8 as _,
        p3 as _,
        p4 as _,
      ) as _
    }
  }

  fn slow_io_uring_prep_writev_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = {
      let v = args.get(0);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(0)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(0).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let p1 = args.get(1).uint32_value(scope).unwrap() as _;
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
    let p3 = args.get(3).uint32_value(scope).unwrap() as _;
    let p4 = args.get(4).uint32_value(scope).unwrap() as _;
    let result = unsafe { r#impl::io_uring_prep_writev(p0, p1, p2, p3, p4) };
    rv.set(v8::undefined(scope).into());
  }

  pub struct io_uring_prep_send_zc_;
  const FAST_io_uring_prep_send_zc: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[
        fast_api::Type::V8Value,
        fast_api::Type::Uint64,
        fast_api::Type::Int32,
        fast_api::Type::Uint64,
        fast_api::Type::Int32,
        fast_api::Type::Int32,
        fast_api::Type::Int32,
      ],
      fast_api::CType::Void,
      fast_io_uring_prep_send_zc_ as *const _,
    );

  fn fast_io_uring_prep_send_zc_(
    _: v8::Local<v8::Object>,
    p0: u64,
    p1: i32,
    p2: u64,
    p3: i32,
    p4: i32,
    p5: i32,
  ) -> () {
    unsafe {
      r#impl::io_uring_prep_send_zc(
        p0 as *const u8 as _,
        p1 as _,
        p2 as *const u8 as _,
        p3 as _,
        p4 as _,
        p5 as _,
      ) as _
    }
  }

  fn slow_io_uring_prep_send_zc_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = {
      let v = args.get(0);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(0)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(0).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
    let p1 = args.get(1).uint32_value(scope).unwrap() as _;
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
    let p3 = args.get(3).uint32_value(scope).unwrap() as _;
    let p4 = args.get(4).uint32_value(scope).unwrap() as _;
    let p5 = args.get(5).uint32_value(scope).unwrap() as _;
    let result =
      unsafe { r#impl::io_uring_prep_send_zc(p0, p1, p2, p3, p4, p5) };
    rv.set(v8::undefined(scope).into());
  }

  pub struct io_uring_wait_cqe_;
  const FAST_io_uring_wait_cqe: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[
        fast_api::Type::V8Value,
        fast_api::Type::Uint64,
        fast_api::Type::Uint64,
      ],
      fast_api::CType::Int32,
      fast_io_uring_wait_cqe_ as *const _,
    );

  fn fast_io_uring_wait_cqe_(
    _: v8::Local<v8::Object>,
    p0: u64,
    p1: u64,
  ) -> i32 {
    unsafe {
      r#impl::io_uring_wait_cqe(p0 as *const u8 as _, p1 as *const u8 as _) as _
    }
  }

  fn slow_io_uring_wait_cqe_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = {
      let v = args.get(0);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(0)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(0).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
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
    let result = unsafe { r#impl::io_uring_wait_cqe(p0, p1) };
    rv.set(v8::Number::new(scope, result as u64 as _).into());
  }

  pub struct io_uring_cqe_seen_;
  const FAST_io_uring_cqe_seen: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[
        fast_api::Type::V8Value,
        fast_api::Type::Uint64,
        fast_api::Type::Uint64,
      ],
      fast_api::CType::Void,
      fast_io_uring_cqe_seen_ as *const _,
    );

  fn fast_io_uring_cqe_seen_(_: v8::Local<v8::Object>, p0: u64, p1: u64) -> () {
    unsafe {
      r#impl::io_uring_cqe_seen(p0 as *const u8 as _, p1 as *const u8 as _) as _
    }
  }

  fn slow_io_uring_cqe_seen_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = {
      let v = args.get(0);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(0)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(0).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
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
    let result = unsafe { r#impl::io_uring_cqe_seen(p0, p1) };
    rv.set(v8::undefined(scope).into());
  }

  pub struct io_uring_wait_cqe2_;
  const FAST_io_uring_wait_cqe2: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[
        fast_api::Type::V8Value,
        fast_api::Type::Uint64,
        fast_api::Type::Uint64,
      ],
      fast_api::CType::Int32,
      fast_io_uring_wait_cqe2_ as *const _,
    );

  fn fast_io_uring_wait_cqe2_(
    _: v8::Local<v8::Object>,
    p0: u64,
    p1: u64,
  ) -> i32 {
    unsafe {
      r#impl::io_uring_wait_cqe2(p0 as *const u8 as _, p1 as *const u8 as _)
        as _
    }
  }

  fn slow_io_uring_wait_cqe2_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = {
      let v = args.get(0);
      if v.is_array_buffer_view() {
        let view =
          v8::Local::<v8::ArrayBufferView>::try_from(args.get(0)).unwrap();
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data().unwrap().as_ptr() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }
      } else {
        let i = args.get(0).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };
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
    let result = unsafe { r#impl::io_uring_wait_cqe2(p0, p1) };
    rv.set(v8::Number::new(scope, result as u64 as _).into());
  }

  pub struct io_uring_cqe_create_data_;
  const FAST_io_uring_cqe_create_data: fast_api::FastFunction =
    fast_api::FastFunction::new(
      &[
        fast_api::Type::V8Value,
        fast_api::Type::Uint32,
        fast_api::Type::Uint32,
      ],
      fast_api::CType::Uint64,
      fast_io_uring_cqe_create_data_ as *const _,
    );

  fn fast_io_uring_cqe_create_data_(
    _: v8::Local<v8::Object>,
    p0: u32,
    p1: u32,
  ) -> u64 {
    unsafe { r#impl::io_uring_cqe_create_data(p0 as _, p1 as _) as _ }
  }

  fn slow_io_uring_cqe_create_data_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
    let p0 = args.get(0).uint32_value(scope).unwrap() as _;
    let p1 = args.get(1).uint32_value(scope).unwrap() as _;
    let result = unsafe { r#impl::io_uring_cqe_create_data(p0, p1) };
    rv.set(v8::Number::new(scope, result as u64 as _).into());
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
    rv.set(v8::Number::new(scope, result as u64 as _).into());
  }

  pub struct bind_;
  const FAST_bind: fast_api::FastFunction = fast_api::FastFunction::new(
    &[
      fast_api::Type::V8Value,
      fast_api::Type::Int32,
      fast_api::Type::Uint64,
      fast_api::Type::Uint32,
    ],
    fast_api::CType::Int32,
    fast_bind_ as *const _,
  );

  fn fast_bind_(_: v8::Local<v8::Object>, p0: i32, p1: u64, p2: u32) -> i32 {
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
    rv.set(v8::Number::new(scope, result as u64 as _).into());
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
    rv.set(v8::Number::new(scope, result as u64 as _).into());
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
    rv.set(v8::Number::new(scope, result as u64 as _).into());
  }
);

crate::cfg_quickjs!(
  mod r#impl;
  use quickjs_ng_sys as q;

  pub fn init(context: *mut q::JSContext) {
    let global_raw = unsafe { q::JS_GetGlobalObject(context) };
    let data = Box::into_raw(Box::new(q::JSValue {
      u: q::JSValueUnion { int32: 0 },
      tag: q::JS_TAG_NULL as _,
    }));

    let f = unsafe {
      q::JS_NewCFunctionData(
        context,
        Some(io_uring_check_version_),
        2,
        0,
        1,
        data,
      )
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "io_uring_check_version ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(context, Some(io_uring_queue_init_), 3, 0, 1, data)
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "io_uring_queue_init ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(context, Some(size_of_io_uring_), 0, 0, 1, data)
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "size_of_io_uring ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(
        context,
        Some(size_of_io_uring_cqe_),
        0,
        0,
        1,
        data,
      )
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "size_of_io_uring_cqe ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(context, Some(io_uring_get_sqe_), 1, 0, 1, data)
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "io_uring_get_sqe ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(
        context,
        Some(io_uring_sqe_set_data_),
        2,
        0,
        1,
        data,
      )
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "io_uring_sqe_set_data ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(
        context,
        Some(io_uring_cqe_get_data_),
        1,
        0,
        1,
        data,
      )
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "io_uring_cqe_get_data ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(context, Some(io_uring_submit_), 1, 0, 1, data)
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "io_uring_submit ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(
        context,
        Some(io_uring_prep_accept_),
        5,
        0,
        1,
        data,
      )
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "io_uring_prep_accept ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(context, Some(io_uring_prep_readv_), 5, 0, 1, data)
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "io_uring_prep_readv ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(
        context,
        Some(io_uring_prep_writev_),
        5,
        0,
        1,
        data,
      )
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "io_uring_prep_writev ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(
        context,
        Some(io_uring_prep_send_zc_),
        6,
        0,
        1,
        data,
      )
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "io_uring_prep_send_zc ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(context, Some(io_uring_wait_cqe_), 2, 0, 1, data)
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "io_uring_wait_cqe ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(context, Some(io_uring_cqe_seen_), 2, 0, 1, data)
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "io_uring_cqe_seen ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(context, Some(io_uring_wait_cqe2_), 2, 0, 1, data)
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "io_uring_wait_cqe2 ".as_ptr() as _,
        f,
      )
    } < 0
    {
      panic!("failed to set property")
    }
    let f = unsafe {
      q::JS_NewCFunctionData(
        context,
        Some(io_uring_cqe_create_data_),
        2,
        0,
        1,
        data,
      )
    };
    if unsafe {
      q::JS_SetPropertyStr(
        context,
        global_raw,
        "io_uring_cqe_create_data ".as_ptr() as _,
        f,
      )
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
  }
  unsafe extern "C" fn io_uring_check_version_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 2, "in io_uring_check_version");
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let p1 = (*(argv.offset(1 as _))).u.int32;
    let result = r#impl::io_uring_check_version(p0 as _, p1 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn io_uring_queue_init_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 3, "in io_uring_queue_init");
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let p1 = {
      if q::JS_IsObject(*(argv.offset(1 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(1 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(1 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p2 = (*(argv.offset(2 as _))).u.int32;
    let result = r#impl::io_uring_queue_init(p0 as _, p1 as _, p2 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn size_of_io_uring_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 0, "in size_of_io_uring");

    let result = r#impl::size_of_io_uring();
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn size_of_io_uring_cqe_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 0, "in size_of_io_uring_cqe");

    let result = r#impl::size_of_io_uring_cqe();
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn io_uring_get_sqe_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 1, "in io_uring_get_sqe");
    let p0 = {
      if q::JS_IsObject(*(argv.offset(0 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(0 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(0 as _))).u.float64 as u64 as *mut u8
      }
    };
    let result = r#impl::io_uring_get_sqe(p0 as _);
    compile_error!("TODO: implement");
  }
  unsafe extern "C" fn io_uring_sqe_set_data_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 2, "in io_uring_sqe_set_data");
    let p0 = {
      if q::JS_IsObject(*(argv.offset(0 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(0 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(0 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p1 = {
      if q::JS_IsObject(*(argv.offset(1 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(1 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(1 as _))).u.float64 as u64 as *mut u8
      }
    };
    let result = r#impl::io_uring_sqe_set_data(p0 as _, p1 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: 0 },
      tag: q::JS_TAG_UNDEFINED as _,
    }
  }
  unsafe extern "C" fn io_uring_cqe_get_data_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 1, "in io_uring_cqe_get_data");
    let p0 = {
      if q::JS_IsObject(*(argv.offset(0 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(0 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(0 as _))).u.float64 as u64 as *mut u8
      }
    };
    let result = r#impl::io_uring_cqe_get_data(p0 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn io_uring_submit_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 1, "in io_uring_submit");
    let p0 = {
      if q::JS_IsObject(*(argv.offset(0 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(0 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(0 as _))).u.float64 as u64 as *mut u8
      }
    };
    let result = r#impl::io_uring_submit(p0 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn io_uring_prep_accept_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 5, "in io_uring_prep_accept");
    let p0 = {
      if q::JS_IsObject(*(argv.offset(0 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(0 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(0 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p1 = (*(argv.offset(1 as _))).u.int32;
    let p2 = {
      if q::JS_IsObject(*(argv.offset(2 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(2 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(2 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p3 = {
      if q::JS_IsObject(*(argv.offset(3 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(3 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(3 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p4 = (*(argv.offset(4 as _))).u.int32;
    let result =
      r#impl::io_uring_prep_accept(p0 as _, p1 as _, p2 as _, p3 as _, p4 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: 0 },
      tag: q::JS_TAG_UNDEFINED as _,
    }
  }
  unsafe extern "C" fn io_uring_prep_readv_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 5, "in io_uring_prep_readv");
    let p0 = {
      if q::JS_IsObject(*(argv.offset(0 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(0 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(0 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p1 = (*(argv.offset(1 as _))).u.int32;
    let p2 = {
      if q::JS_IsObject(*(argv.offset(2 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(2 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(2 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p3 = (*(argv.offset(3 as _))).u.int32;
    let p4 = (*(argv.offset(4 as _))).u.int32;
    let result =
      r#impl::io_uring_prep_readv(p0 as _, p1 as _, p2 as _, p3 as _, p4 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: 0 },
      tag: q::JS_TAG_UNDEFINED as _,
    }
  }
  unsafe extern "C" fn io_uring_prep_writev_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 5, "in io_uring_prep_writev");
    let p0 = {
      if q::JS_IsObject(*(argv.offset(0 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(0 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(0 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p1 = (*(argv.offset(1 as _))).u.int32;
    let p2 = {
      if q::JS_IsObject(*(argv.offset(2 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(2 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(2 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p3 = (*(argv.offset(3 as _))).u.int32;
    let p4 = (*(argv.offset(4 as _))).u.int32;
    let result =
      r#impl::io_uring_prep_writev(p0 as _, p1 as _, p2 as _, p3 as _, p4 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: 0 },
      tag: q::JS_TAG_UNDEFINED as _,
    }
  }
  unsafe extern "C" fn io_uring_prep_send_zc_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 6, "in io_uring_prep_send_zc");
    let p0 = {
      if q::JS_IsObject(*(argv.offset(0 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(0 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(0 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p1 = (*(argv.offset(1 as _))).u.int32;
    let p2 = {
      if q::JS_IsObject(*(argv.offset(2 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(2 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(2 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p3 = (*(argv.offset(3 as _))).u.int32;
    let p4 = (*(argv.offset(4 as _))).u.int32;
    let p5 = (*(argv.offset(5 as _))).u.int32;
    let result = r#impl::io_uring_prep_send_zc(
      p0 as _, p1 as _, p2 as _, p3 as _, p4 as _, p5 as _,
    );
    q::JSValue {
      u: q::JSValueUnion { int32: 0 },
      tag: q::JS_TAG_UNDEFINED as _,
    }
  }
  unsafe extern "C" fn io_uring_wait_cqe_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 2, "in io_uring_wait_cqe");
    let p0 = {
      if q::JS_IsObject(*(argv.offset(0 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(0 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(0 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p1 = {
      if q::JS_IsObject(*(argv.offset(1 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(1 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(1 as _))).u.float64 as u64 as *mut u8
      }
    };
    let result = r#impl::io_uring_wait_cqe(p0 as _, p1 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn io_uring_cqe_seen_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 2, "in io_uring_cqe_seen");
    let p0 = {
      if q::JS_IsObject(*(argv.offset(0 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(0 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(0 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p1 = {
      if q::JS_IsObject(*(argv.offset(1 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(1 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(1 as _))).u.float64 as u64 as *mut u8
      }
    };
    let result = r#impl::io_uring_cqe_seen(p0 as _, p1 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: 0 },
      tag: q::JS_TAG_UNDEFINED as _,
    }
  }
  unsafe extern "C" fn io_uring_wait_cqe2_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 2, "in io_uring_wait_cqe2");
    let p0 = {
      if q::JS_IsObject(*(argv.offset(0 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(0 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(0 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p1 = {
      if q::JS_IsObject(*(argv.offset(1 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(1 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(1 as _))).u.float64 as u64 as *mut u8
      }
    };
    let result = r#impl::io_uring_wait_cqe2(p0 as _, p1 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn io_uring_cqe_create_data_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 2, "in io_uring_cqe_create_data");
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let p1 = (*(argv.offset(1 as _))).u.int32;
    let result = r#impl::io_uring_cqe_create_data(p0 as _, p1 as _);
    compile_error!("TODO: implement");
  }
  unsafe extern "C" fn socket_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 3, "in socket");
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let p1 = (*(argv.offset(1 as _))).u.int32;
    let p2 = (*(argv.offset(2 as _))).u.int32;
    let result = r#impl::socket(p0 as _, p1 as _, p2 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn bind_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 3, "in bind");
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let p1 = {
      if q::JS_IsObject(*(argv.offset(1 as _))) != 0 {
        let mut len = 0;
        let val = *(argv.offset(1 as _));
        q::JS_GetUint8Array(ctx, &mut len, val)
      } else {
        // Get number
        (*(argv.offset(1 as _))).u.float64 as u64 as *mut u8
      }
    };
    let p2 = (*(argv.offset(2 as _))).u.int32;
    let result = r#impl::bind(p0 as _, p1 as _, p2 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn listen_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 2, "in listen");
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let p1 = (*(argv.offset(1 as _))).u.int32;
    let result = r#impl::listen(p0 as _, p1 as _);
    q::JSValue {
      u: q::JSValueUnion { int32: result as _ },
      tag: q::JS_TAG_INT as _,
    }
  }
  unsafe extern "C" fn close_(
    ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
  ) -> q::JSValue {
    // assert!(argc == 1, "in close");
    let p0 = (*(argv.offset(0 as _))).u.int32;
    let result = r#impl::close(p0 as _);
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
        Some(io_uring_check_version_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("io_uring_check_version ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(io_uring_queue_init_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("io_uring_queue_init ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(size_of_io_uring_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("size_of_io_uring ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(size_of_io_uring_cqe_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("size_of_io_uring_cqe ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(io_uring_get_sqe_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("io_uring_get_sqe ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(io_uring_sqe_set_data_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("io_uring_sqe_set_data ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(io_uring_cqe_get_data_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("io_uring_cqe_get_data ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(io_uring_submit_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("io_uring_submit ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(io_uring_prep_accept_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("io_uring_prep_accept ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(io_uring_prep_readv_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("io_uring_prep_readv ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(io_uring_prep_writev_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("io_uring_prep_writev ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(io_uring_prep_send_zc_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("io_uring_prep_send_zc ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(io_uring_wait_cqe_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("io_uring_wait_cqe ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(io_uring_cqe_seen_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("io_uring_cqe_seen ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(io_uring_wait_cqe2_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("io_uring_wait_cqe2 ".as_ptr() as _)
    };
    let mut exception: JSValueRef = std::ptr::null_mut();
    unsafe { JSObjectSetProperty(context, obj, name, func, 0, &mut exception) }

    let func = unsafe {
      JSObjectMakeFunctionWithCallback(
        context,
        std::ptr::null_mut() as _,
        Some(io_uring_cqe_create_data_),
      )
    };
    let name = unsafe {
      JSStringCreateWithUTF8CString("io_uring_cqe_create_data ".as_ptr() as _)
    };
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
  }
  unsafe extern "C" fn io_uring_check_version_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 2, "io_uring_check_version expects atleast 2 arguments");
    let p0 = JSValueToNumber(ctx, *(arguments.offset(0 as _)), exception);
    let p1 = JSValueToNumber(ctx, *(arguments.offset(1 as _)), exception);
    let result = r#impl::io_uring_check_version(p0 as _, p1 as _);
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn io_uring_queue_init_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 3, "io_uring_queue_init expects atleast 3 arguments");
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
    let result = r#impl::io_uring_queue_init(p0 as _, p1 as _, p2 as _);
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn size_of_io_uring_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 0, "size_of_io_uring expects atleast 0 arguments");

    let result = r#impl::size_of_io_uring();
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn size_of_io_uring_cqe_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 0, "size_of_io_uring_cqe expects atleast 0 arguments");

    let result = r#impl::size_of_io_uring_cqe();
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn io_uring_get_sqe_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 1, "io_uring_get_sqe expects atleast 1 arguments");
    let p0 = if JSValueIsObject(ctx, *(arguments.offset(0) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(0) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(0) as *mut _), exception) as u64
        as *mut ()
    };
    let result = r#impl::io_uring_get_sqe(p0 as _);
    compile_error!("TODO: implement")
  }
  unsafe extern "C" fn io_uring_sqe_set_data_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 2, "io_uring_sqe_set_data expects atleast 2 arguments");
    let p0 = if JSValueIsObject(ctx, *(arguments.offset(0) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(0) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(0) as *mut _), exception) as u64
        as *mut ()
    };
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
    let result = r#impl::io_uring_sqe_set_data(p0 as _, p1 as _);
    JSValueMakeUndefined(ctx)
  }
  unsafe extern "C" fn io_uring_cqe_get_data_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 1, "io_uring_cqe_get_data expects atleast 1 arguments");
    let p0 = if JSValueIsObject(ctx, *(arguments.offset(0) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(0) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(0) as *mut _), exception) as u64
        as *mut ()
    };
    let result = r#impl::io_uring_cqe_get_data(p0 as _);
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn io_uring_submit_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 1, "io_uring_submit expects atleast 1 arguments");
    let p0 = if JSValueIsObject(ctx, *(arguments.offset(0) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(0) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(0) as *mut _), exception) as u64
        as *mut ()
    };
    let result = r#impl::io_uring_submit(p0 as _);
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn io_uring_prep_accept_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 5, "io_uring_prep_accept expects atleast 5 arguments");
    let p0 = if JSValueIsObject(ctx, *(arguments.offset(0) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(0) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(0) as *mut _), exception) as u64
        as *mut ()
    };
    let p1 = JSValueToNumber(ctx, *(arguments.offset(1 as _)), exception);
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
      r#impl::io_uring_prep_accept(p0 as _, p1 as _, p2 as _, p3 as _, p4 as _);
    JSValueMakeUndefined(ctx)
  }
  unsafe extern "C" fn io_uring_prep_readv_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 5, "io_uring_prep_readv expects atleast 5 arguments");
    let p0 = if JSValueIsObject(ctx, *(arguments.offset(0) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(0) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(0) as *mut _), exception) as u64
        as *mut ()
    };
    let p1 = JSValueToNumber(ctx, *(arguments.offset(1 as _)), exception);
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
    let p3 = JSValueToNumber(ctx, *(arguments.offset(3 as _)), exception);
    let p4 = JSValueToNumber(ctx, *(arguments.offset(4 as _)), exception);
    let result =
      r#impl::io_uring_prep_readv(p0 as _, p1 as _, p2 as _, p3 as _, p4 as _);
    JSValueMakeUndefined(ctx)
  }
  unsafe extern "C" fn io_uring_prep_writev_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 5, "io_uring_prep_writev expects atleast 5 arguments");
    let p0 = if JSValueIsObject(ctx, *(arguments.offset(0) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(0) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(0) as *mut _), exception) as u64
        as *mut ()
    };
    let p1 = JSValueToNumber(ctx, *(arguments.offset(1 as _)), exception);
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
    let p3 = JSValueToNumber(ctx, *(arguments.offset(3 as _)), exception);
    let p4 = JSValueToNumber(ctx, *(arguments.offset(4 as _)), exception);
    let result =
      r#impl::io_uring_prep_writev(p0 as _, p1 as _, p2 as _, p3 as _, p4 as _);
    JSValueMakeUndefined(ctx)
  }
  unsafe extern "C" fn io_uring_prep_send_zc_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 6, "io_uring_prep_send_zc expects atleast 6 arguments");
    let p0 = if JSValueIsObject(ctx, *(arguments.offset(0) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(0) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(0) as *mut _), exception) as u64
        as *mut ()
    };
    let p1 = JSValueToNumber(ctx, *(arguments.offset(1 as _)), exception);
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
    let p3 = JSValueToNumber(ctx, *(arguments.offset(3 as _)), exception);
    let p4 = JSValueToNumber(ctx, *(arguments.offset(4 as _)), exception);
    let p5 = JSValueToNumber(ctx, *(arguments.offset(5 as _)), exception);
    let result = r#impl::io_uring_prep_send_zc(
      p0 as _, p1 as _, p2 as _, p3 as _, p4 as _, p5 as _,
    );
    JSValueMakeUndefined(ctx)
  }
  unsafe extern "C" fn io_uring_wait_cqe_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 2, "io_uring_wait_cqe expects atleast 2 arguments");
    let p0 = if JSValueIsObject(ctx, *(arguments.offset(0) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(0) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(0) as *mut _), exception) as u64
        as *mut ()
    };
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
    let result = r#impl::io_uring_wait_cqe(p0 as _, p1 as _);
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn io_uring_cqe_seen_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 2, "io_uring_cqe_seen expects atleast 2 arguments");
    let p0 = if JSValueIsObject(ctx, *(arguments.offset(0) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(0) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(0) as *mut _), exception) as u64
        as *mut ()
    };
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
    let result = r#impl::io_uring_cqe_seen(p0 as _, p1 as _);
    JSValueMakeUndefined(ctx)
  }
  unsafe extern "C" fn io_uring_wait_cqe2_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 2, "io_uring_wait_cqe2 expects atleast 2 arguments");
    let p0 = if JSValueIsObject(ctx, *(arguments.offset(0) as *mut _)) {
      JSObjectGetTypedArrayBytesPtr(
        ctx,
        *(arguments.offset(0) as *mut _),
        exception,
      ) as *mut ()
    } else {
      JSValueToNumber(ctx, *(arguments.offset(0) as *mut _), exception) as u64
        as *mut ()
    };
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
    let result = r#impl::io_uring_wait_cqe2(p0 as _, p1 as _);
    JSValueMakeNumber(ctx, result as _)
  }
  unsafe extern "C" fn io_uring_cqe_create_data_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
  ) -> JSValueRef {
    // assert!(argument_count <= 2, "io_uring_cqe_create_data expects atleast 2 arguments");
    let p0 = JSValueToNumber(ctx, *(arguments.offset(0 as _)), exception);
    let p1 = JSValueToNumber(ctx, *(arguments.offset(1 as _)), exception);
    let result = r#impl::io_uring_cqe_create_data(p0 as _, p1 as _);
    compile_error!("TODO: implement")
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
);
