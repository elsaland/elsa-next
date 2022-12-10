#!/usr/bin/env deno run -A --unstable

import { main } from "./codegen.ts";
import modules from "../modules/mod.ts";

await main(modules);
