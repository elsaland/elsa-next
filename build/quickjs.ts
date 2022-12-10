import { Definition, Generator } from "./codegen.ts";

export class QuickJsGenerator implements Generator {
  modulePrelude(): string {
    return `
  pub fn setup_bindings(
    context: &quick_js::Context,
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
  ${imports.map((i) => `mod ${i};`).join("\n")}
  pub fn init(
    context: &quick_js::Context,
  ) {\n`;
  }

  symbolsSet(name: string): string {
    return `\n    context.add_callback(
              "${name}",
              &${name}_,
            ).unwrap();`;
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
  return `fn ${name}_(
  ${parameters.map((p, i) => `p${i}: ${parameterValues(p)}`).join(", ")}
) -> ${result} {
  r#impl::${name}(${parameters.map((_, idx) => `p${idx} as _`).join(", ")}) as _
}
  `;
}
