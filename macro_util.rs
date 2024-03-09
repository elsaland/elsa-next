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

#[macro_export]
macro_rules! cfg_v8 {
  ($($item:item)*) => {
      $(
          #[cfg(feature = "use_v8")]
          #[cfg_attr(docsrs, doc(cfg(feature = "use_v8")))]
          $item
      )*
  }
}

#[macro_export]
macro_rules! cfg_jsc {
  ($($item:item)*) => {
      $(
          #[cfg(feature = "use_jsc")]
          #[cfg_attr(docsrs, doc(cfg(feature = "use_jsc")))]
          $item
      )*
  }
}

#[macro_export]
macro_rules! cfg_mozjs {
  ($($item:item)*) => {
      $(
          #[cfg(feature = "use_spidermonkey")]
          #[cfg_attr(docsrs, doc(cfg(feature = "use_spidermonkey")))]
          $item
      )*
  }
}

#[macro_export]
macro_rules! cfg_quickjs {
  ($($item:item)*) => {
      $(
          #[cfg(feature = "use_quickjs")]
          #[cfg_attr(docsrs, doc(cfg(feature = "use_quickjs")))]
          $item
      )*
  }
}

#[macro_export]
macro_rules! cfg_hermes {
  ($($item:item)*) => {
      $(
          #[cfg(feature = "use_hermes")]
          #[cfg_attr(docsrs, doc(cfg(feature = "use_hermes")))]
          $item
      )*
  }
}
