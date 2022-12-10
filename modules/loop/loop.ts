import type { Definition } from "../../build/codegen.ts";

const symbols: Definition[] = [];

export default {
  symbols,
  name: "loop",
  output: new URL("mod.rs", import.meta.url).pathname,
};
