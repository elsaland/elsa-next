## Why?

Small, baseline performant JavaScript runtime. If you want to answer, _how fast
can JavaScript runtimes go?_. This is the place.

We use deno-lite for benchmarking Deno internally.

deno-lite is engine-agnostic. It has engine backends for:

- V8
- JavaScriptCore
- SpiderMonkey
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

## Building

You need python 3.9 (not 3.10) to build the spidermonkey backend.

```shell
# Warning: this may be destructive!
brew install python@3.9
cd $(dirname $(which python3.9))
rm -f python3 pip3
ln -s python3.9 python3
ln -s pip3.9 pip3
```

## Authors

[littledivy](http://github.com/littledivy) - Divy Srivastava

## License

MIT
