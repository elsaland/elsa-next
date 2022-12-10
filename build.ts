#!/usr/bin/env deno run -A --unstable

import $ from "https://deno.land/x/dax/mod.ts";

const mode = Deno.args[0] || "debug";

async function allRelease() {
  await $`./build/main.ts && cargo fmt`;
  const sizeData = [];
  for (const feature of ["use_v8", "use_jsc", "use_quickjs"]) {
    await $`cargo build --release --features ${feature},typescript --no-default-features`;
    sizeData.push({ feature: feature.slice(4), size: `${size()}MB` });
  }
  console.table(sizeData);
}

function size() {
  const s = Deno.statSync("./target/release/deno-lite").size;
  return (s / 1024 / 1024).toFixed(2);
}

async function release() {
  await $`./build/main.ts && cargo fmt && cargo build --release`;
  console.log(`Size: ${size()}MB`);
}

const actions: Record<string, () => Promise<void>> = {
  "debug": () => $`./build/main.ts && cargo fmt && cargo build`,
  "release": () => release(),
  "release-all": () => allRelease(),
};

if (mode in actions) {
  await actions[mode]();
} else {
  console.error(
    `Unknown mode: ${mode}. Available modes: ${
      Object.keys(actions).join(", ")
    }`,
  );
  Deno.exit(1);
}
