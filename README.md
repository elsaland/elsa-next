## Elsa

Small, baseline performant JavaScript runtime. If you want to answer, _how fast
can JavaScript runtimes go?_. This is the place.

We use Elsa for benchmarking Deno internally.

Elsa is engine-agnostic. It has engine backends for:

- V8
- JavaScriptCore
- SpiderMonkey
- QuickJS

No module system, single file scripts only. I might implement ESM behind a
feature flag in the future.

TypeScript stripping support available behind `typescript` feature flag using
`swc`. Typechecking done using `stc`.

## Why was this revived?

When it first started out, Elsa was a fun project. I eventually moved on to work
on Deno.

We were using just-js/spindle/crimson for baseline perf comparison. I wanted to
benchmark against many many possibilities. Elsa is an attempt to write a
cross-platform engine-agnostic fast JavaScript runtime.

Elsa is designed for:

- Speed. TODO microbenchmarks.
- Size. Smallest configuration is 0.3MB.
- Tons of compile time feature flags. Don't include things in the binary you
  don't need!

  Ex: "I want to run a script _in V8_ that only uses _FS apis_, also don't want
  to _type check_ but strip types":

  `cargo build --no-default-features --features "use_v8,fs,typescript"`

- Easy to embed. Offers a Rust crate and C API (with Go and Zig bindings).

In time, most Elsa speed optimizations will make it to Deno.

## Why not use spindle?

It only works on Linux. Unfortunately, making it work on macOS / windows is
non-trivial as compiling requires a previous version of spindle.

## Why not use crimson?

This is in a way, an extension of crimson. Crison _was_ an internal research
project to figure out a fast way to run Rust futures with a JS-first event loop
like in just-js, spindle.

Unlike crimson, Elsa cannot run Rust futures. This is however still a JS-first
event loop.

Elsa copies the code-generation from Crimson which greately improves
maintainaiblity and unlike spindle, it is cross-platform.

Key difference from crimson:

- No common state
- Not rust futures executor

## Building

To build Elsa, you need:

- Deno (>=1.28.3)
- Rust (>=1.66.0)

```shell
./build.ts debug # build debug. V8 without typescript support.
./build.ts release # build release.
./build.ts release-all # build release with all engine backends.

./build.ts release --features "use_quickjs,typescript,typecheck" # build release with quickjs and full TS support.
# ...
```

You need python 3.9 (not 3.10) to build the spidermonkey backend.

```shell
# Warning: this may be destructive!
brew install python@3.9
cd $(dirname $(which python3.9))
rm -f python3 pip3
ln -s python3.9 python3
ln -s pip3.9 pip3
```

## Size

- Note: `jsc` is not bundled but linked dynamically. This will change in the
  future and included it in the below table.

Without typescript support:

```
┌───────┬───────────┬───────────┐
│ (idx) │ feature   │ size      │
├───────┼───────────┼───────────┤
│     0 │ "v8"      │ "26.94MB" │
│     2 │ "quickjs" │ "1.28MB"  │
└───────┴───────────┴───────────┘
```

With typescript support:

```
┌───────┬───────────┬───────────┐
│ (idx) │ feature   │ size      │
├───────┼───────────┼───────────┤
│     0 │ "v8"      │ "31.96MB" │
│     2 │ "quickjs" │ "6.30MB"  │
└───────┴───────────┴───────────┘
```

## Guidelines for contributors

- No serde. It's a performance killer. Serializing objects is discouraged. If
  you need to pass untyped complex data, use the engine's `Value` type.
- Typed data must always have inlined conversions using codegen in place.
- Everything should be behind a compile time feature flag.

## Authors

[littledivy](http://github.com/littledivy) - Divy Srivastava

## License

MIT
