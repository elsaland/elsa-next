#!/usr/bin/env deno run -A --unstable

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

import $ from "https://deno.land/x/dax/mod.ts";
import { checkLicense } from "https://deno.land/x/license_checker@v3.1.4/lib.ts";

const mode = Deno.args[0] || "debug";

$.setPrintCommand(true);

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

function maybeArgs() {
  return Deno.args.slice(1);
}

async function debug() {
  const maybeFlags = maybeArgs();
  if (maybeFlags.length > 0) {
    await $`./build/main.ts && cargo fmt && cargo build --no-default-features --features ${
      maybeFlags.join(",")
    }`;
  } else {
    await $`./build/main.ts && cargo fmt && cargo build`;
  }
}

async function release() {
  const maybeFlags = maybeArgs();
  if (maybeFlags.length > 0) {
    await $`./build/main.ts && cargo fmt && cargo build --release --no-default-features --features ${
      maybeFlags.join(",")
    }`;
  } else {
    await $`./build/main.ts && cargo fmt && cargo build`;
  }
}

const years = "2022";
const owner = "Divy Srivastava";
const projectname = "elsaland/elsa";
const projecturl = "https://github.com/elsaland/elsa-next";

async function checkApache2(fix: boolean) {
  const licenses = await checkLicense(
    [
      {
        ignore: [
          ".git",
          "target",
        ],
        config: [
          [
            "**/*.{js,ts,jsx,tsx,rs}",
            `// Copyright (c) ${years} ${owner}.
//
// This file is part of ${projectname}.
// See ${projecturl} for further info.
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
`,
          ],
        ],
      },
    ],
    { cwd: "./", inject: fix },
  );
  if (!licenses) {
    console.log("%cLicenses not correct.", "color:red");
    Deno.exit(1);
  }
}

async function check(fix: boolean) {
  await checkApache2(fix);
}

const actions: Record<string, () => Promise<void>> = {
  "debug": debug,
  "release": release,
  "release-all": allRelease,
  "check-fix": () => check(true),
  "check": () => check(false),
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
