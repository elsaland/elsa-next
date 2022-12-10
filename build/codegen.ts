import { QuickJsGenerator } from "./quickjs.ts";
import { V8Generator } from "./v8.ts";

export interface Generator {
  modulePrelude(): string;
  moduleBody(module: string): string;
  moduleEnd(): string;

  symbolsPrelude(imports: string[]): string;
  symbolsSet(name: string): string;
  generateBinding(def: Definition): string;
}

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

export const preludeComment = `// This file is auto-generated!!\n`;
const reservedRust = ["loop", "type", "match", "async"];

function ident(name: string) {
  if (reservedRust.includes(name)) {
    return `r#${name}`;
  }
  return name;
}

const cfg: Record<string, Generator> = {
  cfg_v8: new V8Generator(),
  cfg_quickjs: new QuickJsGenerator(),
};

async function writeAndFormat(cfg: string, path: string, content: string) {
  await Deno.writeTextFile(path, content);
  await Deno.run({
    cmd: ["rustfmt", path],
  }).status();
  console.log(`[${cfg}] ${path} ✅`);
}

export async function main(modules: Module[]) {
  const p = new Array<Promise<void>>(modules.length + 1);

  let mod = preludeComment;
  for (const [cfgIf, generator] of Object.entries(cfg)) {
    mod += `crate::${cfgIf}!(\n`;
    let initRest = generator.modulePrelude();

    for (const { name, output, symbols, imports = [] } of modules) {
      let code = `${preludeComment}
#![allow(non_camel_case_types)]
#![allow(unused_variables)]\n\n`;

      const moduleName = ident(name);
      mod += `  mod ${moduleName};\n`;

      initRest += generator.moduleBody(moduleName);
      for (const [cfgIf, generator] of Object.entries(cfg)) {
        code += `crate::${cfgIf}! (\n`;

        code += `${generator.symbolsPrelude(imports)}`;
        let rest = "";
        for (const symbol of symbols) {
          rest += generator.generateBinding(symbol);
          code += generator.symbolsSet(symbol.name);
        }

        code += "  }\n";
        code += rest;
        code += ");\n\n";
      }
      p.push(
        writeAndFormat(cfgIf, output, code),
      );
    }

    initRest += generator.moduleEnd();
    mod += initRest;
    mod += `);\n`;
  }

  p.push(
    writeAndFormat(
      "all",
      new URL("../modules/mod.rs", import.meta.url).pathname,
      mod,
    ),
  );

  await Promise.all(p);
}
