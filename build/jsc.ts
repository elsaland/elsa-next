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

export class JscGenerator implements Generator {
  modulePrelude(): string {
    return `
  pub fn setup_bindings(
    context: rusty_jsc_sys::JSContextRef,
  ) {
    let global = unsafe { rusty_jsc_sys::JSContextGetGlobalObject(context) };\n`;
  }

  moduleBody(moduleName: string): string {
    return `    ${moduleName}::init(global, context);\n`;
  }

  moduleEnd(): string {
    return "  }\n";
  }

  symbolsPrelude(imports: string[]): string {
    return `  mod r#impl;
    use rusty_jsc_sys::*;
  ${imports.map((i) => `mod ${i};`).join("\n")}
  pub fn init(
    obj: JSObjectRef,
    context: JSContextRef,
  ) {`;
  }

  symbolsSet(name: string): string {
    return `\n  let func = unsafe { JSObjectMakeFunctionWithCallback(context, std::ptr::null_mut() as _, Some(${name}_)) };
  let name = unsafe { JSStringCreateWithUTF8CString("${name}\0".as_ptr() as _) };
  let mut exception: JSValueRef = std::ptr::null_mut();
  unsafe {
    JSObjectSetProperty(
        context,
        obj,
        name,
        func,
        0,
        &mut exception,
    )
  }
`;
  }

  generateBinding(def: Definition): string {
    return generateBinding(def);
  }
}

const todo = 'compile_error!("TODO: implement")';
function parameterValues(ty: string, i: number) {
  if (ty === "buffer") {
    return todo;
  }
  if (ty === "pointer") {
    return "u64";
  }
  // skip scalar type validation, compiler will catch it
  return `JSValueToNumber(ctx, *(arguments.offset(${i} as _)), exception)`;
}

function makeReturn(type: string): string {
  if (type === "void") {
    return "JSValueMakeUndefined(ctx)";
  }
  if (type === "buffer") {
    return todo;
  }
  if (type === "pointer") {
    return todo;
  }
  return `JSValueMakeNumber(ctx, result as _)`;
}

export function generateBinding(
  { name, parameters = [], result = "void" }: Definition,
): string {
  return ` unsafe extern "C" fn ${name}_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    argument_count: size_t,
    arguments: *const JSValueRef,
    exception: *mut JSValueRef,
) -> JSValueRef {
  assert!(argument_count == ${parameters.length});
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
