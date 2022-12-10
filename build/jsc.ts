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
function parameterValues(ty: string) {
  if (ty === "buffer") {
    return todo;
  }
  if (ty === "pointer") {
    return "u64";
  }
  // skip scalar type validation, compiler will catch it
  return ty;
}

export function generateBinding(
  { name, parameters = [], result = "void" }: Definition,
): string {
  return ` unsafe extern "C" fn ${name}_(
    ctx: JSContextRef,
    _function: JSObjectRef,
    _this_object: JSObjectRef,
    _argument_count: size_t,
    _arguments: *const JSValueRef,
    _exception: *mut JSValueRef,
) -> JSValueRef {
  // ${parameters.map((p, i) => `let p${i} = ${parameterValues(p)}`).join(", ")}
  // r#impl::${name}(${
    parameters.map((_, idx) => `p${idx} as _`).join(", ")
  }) as _
  JSValueMakeUndefined(ctx)
}
  `;
}
