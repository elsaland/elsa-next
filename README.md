## Why?

Small, baseline performant JavaScript runtime. If you want to answer, _how fast
can JavaScript runtimes go?_. This is the place.

We use deno-lite for benchmarking Deno internally.

deno-lite is engine-agnostic. It has engine backends for:

- V8
- JavaScriptCore
- QuickJS

## Why not use spindle?

It only works on Linux. Unfortunately, making it work on macOS / windows is
non-trivial as compiling requires a previous version of spindle.

## Why not use crimson?

This is in a way, an extension of crimson. Crison _was_ an internal research
project to figure out a fast way to run Rust futures with a JS-first event loop
like in just-js, spindle.

Unlike crimson, deno-lite cannot run Rust futures. This is however still a
JS-first event loop.

deno-lite copies the code-generation from Crimson which greately improves
maintainaiblity and unlike spindle, it is cross-platform.

Key difference from crimson:

- No common state
- Not rust futures executor

## Authors

[littledivy](http://github.com/littledivy) - Divy Srivastava

## License

MIT
