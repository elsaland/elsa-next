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

import type { Definition } from "../../build/codegen.ts";

const symbols: Definition[] = [
  {
    name: "print",
    parameters: ["string"],
    result: "void",
  },
  // // kqueue
  // {
  //   name: "kqueue",
  //   parameters: [],
  //   result: "i32",
  // },
  // {
  //   name: "kevent",
  //   "parameters": ["i32", "pointer", "i32", "pointer", "i32", "pointer"],
  //   "result": "i32",
  // },
  // {
  //   name: "socket",
  //   "parameters": ["i32", "i32", "i32"],
  //   "result": "i32",
  // },
  // {
  //   name: "setsockopt",
  //   "parameters": ["i32", "i32", "i32", "pointer", "i32"],
  //   "result": "i32",
  // },
  // {
  //   name: "bind",
  //   "parameters": ["i32", "pointer", "i32"],
  //   "result": "i32",
  // },
  // {
  //   name: "listen",
  //   "parameters": ["i32", "i32"],
  //   "result": "i32",
  // },
  // {
  //   name: "close",
  //   "parameters": ["i32"],
  //   "result": "i32",
  // },
  // {
  //   name: "accept",
  //   "parameters": ["i32", "pointer", "pointer"],
  //   "result": "i32",
  // },
  // {
  //   name: "send",
  //   "parameters": ["i32", "buffer", "i32", "i32"],
  //   "result": "i32",
  // },
  // {
  //   name: "recv",
  //   "parameters": ["i32", "buffer", "i32", "i32"],
  //   "result": "i32",
  // },
];

export default {
  symbols,
  name: "loop",
  output: new URL("mod.rs", import.meta.url).pathname,
};
