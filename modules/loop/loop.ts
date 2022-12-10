import type { Definition } from "../../build/codegen.ts";

const symbols: Definition[] = [
  {
    name: "add",
    parameters: ["i32", "i32"],
    result: "i32",
    description: "Add two numbers",
  },
];

export default {
  symbols,
  name: "loop",
  output: new URL("mod.rs", import.meta.url).pathname,
};
