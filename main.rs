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

#[macro_use]
mod macro_util;
// mod module_map;
mod modules;
mod runtime;
#[cfg(feature = "typescript")]
mod strip;

use crate::runtime::AbstractRuntime;

fn main() {
  let filename = std::env::args()
    .nth(1)
    .expect("Invalid invocation. Usage: elsa <filename>");

  #[cfg(feature = "typescript")]
  let source = strip::strip(&filename);

  #[cfg(not(feature = "typescript"))]
  let source = std::fs::read_to_string(&filename).expect("failed to read file");

  let mut rt = runtime::Runtime::init();
  rt.setup_bindings();
  rt.eval(&source);
}
