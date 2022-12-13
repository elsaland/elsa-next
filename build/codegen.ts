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

import { QuickJsGenerator } from "./quickjs.ts";
import { JscGenerator } from "./jsc.ts";
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
  cfg_jsc: new JscGenerator(),
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
    mod += `);\n\n`;
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
