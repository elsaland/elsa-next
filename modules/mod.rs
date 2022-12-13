// Copyright (c) 2022 Divy Srivastava.
//
// This file is part of elsaland/elsa.
// See https://github.com/elsaland/elsa-next for further info.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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

  pub fn setup_bindings(context: *mut libquickjs_sys::JSContext) {
    r#loop::init(context);
  }
);

crate::cfg_jsc!(
  mod r#loop;

  pub fn setup_bindings(context: rusty_jsc_sys::JSContextRef) {
    let global = unsafe { rusty_jsc_sys::JSContextGetGlobalObject(context) };
    r#loop::init(global, context);
  }
);
