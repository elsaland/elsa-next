export type Module = {
  symbols: Definition[];
  name: string;
  output: string;
  imports?: string[];
};

export type Definition = {
  parameters?: Parameter[];
  result?: Result;
  name: string;
  description?: string;
};

type NativeType = "i32" | "i64" | "f32" | "f64" | "u32" | "u64" | "bool";
type NativeTypeSequence = "*u8" | "*u32" | "*char";

export type Parameter = NativeType | NativeTypeSequence;
export type Result = Exclude<NativeType, "i64" | "u64" | "f64"> | "void";

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
        let i: u64 = serde_v8::from_v8(scope, args.get(${idx})).unwrap();
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
        let i = serde_v8::from_v8::<u64>(scope, args.get(${idx})).unwrap();
        i as *const u8 as _
      }
    };`;
  }

  return `  let p${idx} = serde_v8::from_v8(scope, args.get(${idx})).unwrap();`;
}

export function generateBinding(
  { name, parameters = [], result = "void" }: Definition,
) {
  return `pub struct ${name}_;
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
  crate::syscall!(${name}(${parameters.map(fastValues).join(", ")})) as _
}
fn slow_${name}_(
  scope: &mut v8::HandleScope,
  args: v8::FunctionCallbackArguments,
  mut rv: v8::ReturnValue,
) {
  ${parameters.map(serdeV8Decl).join("\n")}
  let result = crate::syscall!(${name}(${
    parameters.map((_, i) => `p${i}`).join(", ")
  }));
  rv.set(serde_v8::to_v8(scope, result).unwrap());
}`;
}

const preludeComment = `// This file is generated!!\n`;
const reservedRust = ["loop", "type", "match", "async"];

function ident(name: string) {
  if (reservedRust.includes(name)) {
    return `r#${name}`;
  }
  return name;
}

export async function main(modules: Module[]) {
  const p = new Array<Promise<void>>(modules.length + 1);
  let mod = preludeComment;
  let initRest = `
fn setup_bindings<'a, 's>(
  scope: &'a mut v8::HandleScope<'s, ()>,
) -> v8::Local<'s, v8::ObjectTemplate> {
  let global = v8::ObjectTemplate::new(scope);
  `;

  for (let { name: moduleName, output, symbols, imports = [] } of modules) {
    moduleName = ident(moduleName);
    mod += `mod ${moduleName};\n`;
    initRest += `  ${moduleName}::init(scope, global);\n`;

    let code = `${preludeComment}
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use v8::MapFnTo;
use v8::fast_api;
${imports.map((i) => `use ${i};`).join("\n")}

pub fn init<'a, 's>(
  scope: &'a mut v8::HandleScope<'s, ()>,
  global: v8::Local<'s, v8::ObjectTemplate>,
) {
    `;
    let rest = "";
    for (const symbol of symbols) {
      rest += generateBinding(symbol);
      const name = symbol.name;
      code += `\n    global.set(
        v8::String::new(scope, "${name}").unwrap().into(),
        v8::FunctionTemplate::builder_raw(slow_${name}_.map_fn_to())
          .build_fast(scope, &${name}_, None)
          .into(), 
      );`;
    }

    code += "}\n";
    code += rest;
    p.push(
      Deno.writeTextFile(output, code).then(() =>
        console.log(`Compiled ${moduleName} ✅`)
      ),
    );
  }

  initRest += "  global\n}\n";
  mod += initRest;
  p.push(
    Deno.writeTextFile(
      new URL("../modules/mod.rs", import.meta.url).pathname,
      mod,
    ).then(
      () => {
        console.log("Compiled mod.rs ✅");
      },
    ),
  );

  await Promise.all(p);
}
