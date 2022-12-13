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

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Specifier = String;

crate::cfg_v8! {
  type Module = v8::Global<v8::Module>;
}

#[derive(Default)]
struct ModuleMap {
  inner: HashMap<Specifier, Module>,
}

impl ModuleMap {
  pub fn new() -> Self {
    Self {
      inner: HashMap::new(),
    }
  }
}
