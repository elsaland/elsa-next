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

import { Definition, Generator } from "./codegen.ts";

export class QuickJsGenerator implements Generator {
  argcount = 0;
  modulePrelude(): string {
    return `
  pub fn setup_bindings(
    context: *mut quickjs_ng_sys::JSContext,
  ) {\n`;
  }

  moduleBody(moduleName: string): string {
    return `    ${moduleName}::init(context);\n`;
  }

  moduleEnd(): string {
    return "  }\n";
  }

  symbolsPrelude(imports: string[]): string {
    return `  mod r#impl;
  use quickjs_ng_sys as q;
  ${imports.map((i) => `mod ${i};`).join("\n")}
  pub fn init(
    context: *mut q::JSContext,
  ) {
    let global_raw = unsafe { q::JS_GetGlobalObject(context) };
    let data = Box::into_raw(Box::new(q::JSValue {
      u: q::JSValueUnion {
        int32: 0,
      },
      tag: q::JS_TAG_NULL as _,
    }));\n`;
  }

  symbolsSet(name: string): string {
    return `\n     let f = unsafe { q::JS_NewCFunctionData(context, Some(${name}_), ${this.argcount}, 0, 1, data) };
    if unsafe { q::JS_SetPropertyStr(
      context,
      global_raw,
      "${name}\0".as_ptr() as _,
      f,
    ) } < 0 { panic!("failed to set property") }`;
  }

  generateBinding(def: Definition): string {
    this.argcount = def.parameters?.length || 0;
    return generateBinding(def);
  }
}

const todo = 'compile_error!("TODO: implement")';
function parameterValues(ty: string, i: number) {
  if (ty === "buffer") {
    return todo;
  }
  if (ty === "pointer") {
    return todo;
  }
  return `(*(argv.offset(${i} as _))).u.int32`;
}

function makeReturn(type: string): string {
  if (type === "void") {
    return `q::JSValue { u: q::JSValueUnion { int32: 0 }, tag: q::JS_TAG_UNDEFINED as _ }`;
  }
  if (type === "buffer") {
    return todo;
  }
  if (type === "pointer") {
    return todo;
  }
  return `q::JSValue { u: q::JSValueUnion { int32: result as _ }, tag: q::JS_TAG_INT as _ }`;
}

export function generateBinding(
  { name, parameters = [], result = "void" }: Definition,
): string {
  return `unsafe extern "C" fn ${name}_(
    _ctx: *mut q::JSContext,
    _this: q::JSValue,
    argc: i32,
    argv: *mut q::JSValue,
    _magic: i32,
    data: *mut q::JSValue,
) -> q::JSValue {
  assert!(argc == ${parameters.length});
  ${
    parameters.map((p, i) => `  let p${i} = ${parameterValues(p, i)};`).join(
      ";\n",
    )
  }
  let result = r#impl::${name}(${
    parameters.map((_, idx) => `p${idx} as _`).join(", ")
  });
  ${makeReturn(result)}
}
  `;
}
