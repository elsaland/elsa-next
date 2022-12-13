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

export class V8Generator implements Generator {
  modulePrelude(): string {
    return `
  pub fn setup_bindings<'a, 's>(
    scope: &'a mut v8::HandleScope<'s, ()>,
  ) -> v8::Local<'s, v8::ObjectTemplate> {
    let global = v8::ObjectTemplate::new(scope);\n`;
  }

  moduleBody(moduleName: string): string {
    return `    ${moduleName}::init(scope, global);\n`;
  }

  moduleEnd(): string {
    return "    global\n  }\n";
  }

  symbolsPrelude(imports: string[]): string {
    return `  use v8::MapFnTo;
  use v8::fast_api;
  mod r#impl;
  ${imports.map((i) => `mod ${i};`).join("\n")}
  pub fn init<'a, 's>(
    scope: &'a mut v8::HandleScope<'s, ()>,
    global: v8::Local<'s, v8::ObjectTemplate>,
  ) {
`;
  }

  symbolsSet(name: string): string {
    return `    global.set(
      v8::String::new(scope, "${name}").unwrap().into(),
      v8::FunctionTemplate::builder_raw(slow_${name}_.map_fn_to())
        .build_fast(scope, &${name}_, None)
        .into(), 
    );\n`;
  }

  generateBinding(def: Definition): string {
    return generateBinding(def);
  }
}

const fastAPITypeMapping: Record<string, string> = {
  "()": "fast_api::Type::Void",
  "bool": "fast_api::Type::Bool",
  "i32": "fast_api::Type::Int32",
  "i64": "fast_api::Type::Int64",
  "f32": "fast_api::Type::Float32",
  "f64": "fast_api::Type::Float64",
  "u32": "fast_api::Type::Uint32",
  "u64": "fast_api::Type::Uint64",
  "pointer": "fast_api::Type::Uint64",
  "usize": "fast_api::Type::Uint64",
  "isize": "fast_api::Type::Int64",
};

function fastParameters(ty: string) {
  if (fastAPITypeMapping[ty]) {
    return fastAPITypeMapping[ty];
  }
  if (ty === "buffer") {
    return "fast_api::Type::TypedArray(fast_api::CType::Uint8)";
  }
}

function fastValues(ty: string, idx: number) {
  if (ty === "buffer") {
    return `(&*p${idx}).get_storage_if_aligned().unwrap().as_mut_ptr() as _`;
  }
  if (ty === "pointer") {
    return `p${idx} as *const u8 as _`;
  }
  return `p${idx} as _`;
}

function fastParameterValue(ty: string) {
  if (ty === "buffer") {
    return "*const fast_api::FastApiTypedArray<u8>";
  }
  if (ty === "pointer") {
    return "u64";
  }
  // skip scalar type validation, compiler will catch it
  return ty;
}

function serdeV8Decl(ty: string, idx: number) {
  if (ty === "buffer") {
    return `let p${idx} = match v8::Local::<v8::ArrayBufferView>::try_from(args.get(${idx})) {
       Ok(view) => {
         let buffer = view.buffer(scope).unwrap();
         let store = buffer.data() as *mut u8;
         unsafe { store.add(view.byte_offset()) as _ }
       }
       Err(_) => {
        let i: u64 = args.get(${idx}).number_value(scope).unwrap() as u64;
        i as *const u8 as _
       }
     };`;
  }

  if (ty === "pointer") {
    return `let p${idx} = {
      let v = args.get(${idx});
      if v.is_array_buffer_view() {
        let view = v8::Local::<v8::ArrayBufferView>::try_from(args.get(${idx})).unwrap();  
        let buffer = view.buffer(scope).unwrap();
        let store = buffer.data() as *mut u8;
        unsafe { store.add(view.byte_offset()) as _ }        
      } else {
        let i = args.get(${idx}).number_value(scope).unwrap() as u64;
        i as *const u8 as _
      }
    };`;
  }

  return `    let p${idx} = args.get(${idx}).uint32_value(scope).unwrap() as _;`;
}

export function generateBinding(
  { name, parameters = [], result = "void" }: Definition,
) {
  return `
  pub struct ${name}_;
  impl fast_api::FastFunction for ${name}_ {
    fn function(&self) -> *const std::ffi::c_void  {
      fast_${name}_ as *const _
    }
    fn args(&self) -> &'static [fast_api::Type] {
      &[ fast_api::Type::V8Value, ${parameters.map(fastParameters).join(", ")} ]
    }
    fn return_type(&self) -> fast_api::CType {
      ${
    fastAPITypeMapping[result].replace(
      "fast_api::Type::",
      "fast_api::CType::",
    )
  }
    }
  }

  fn fast_${name}_(
    _: v8::Local<v8::Object>,
    ${parameters.map((p, i) => `p${i}: ${fastParameterValue(p)}`).join(", ")}
  ) -> ${result} {
    r#impl::${name}(${parameters.map(fastValues).join(", ")}) as _
  }

  fn slow_${name}_(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
  ) {
${parameters.map(serdeV8Decl).join("\n")}
    let result = r#impl::${name}(${
    parameters.map((_, i) => `p${i}`).join(", ")
  });
    rv.set(v8::Number::new(scope, result as _).into());
  }\n`;
}
