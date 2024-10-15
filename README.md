## Publish interface component to wa.dev
```sh
$ mkdir ferris-says-interface
$ cd ferris-says-interface

$ cat wit/world.wit
package tanishiking:ferris-says;

interface ferris-says {
    say: func(content: string, width: u32) -> string;
}

world ferrisify {
    export ferris-says;
}

world ferrisify-cli {
    export ferris-says;
}

$ wkg wit build
WIT package written to tanishiking:ferris-says.wasm

$ wkg publish tanishiking:ferris-says.wasm
Error: malformed component: component package version not found

Caused by:
    component package version not found
```

Okay, add a package version to the WIT.

```diff
diff --git a/ferris-says-interface/wit/world.wit b/ferris-says-interface/wit/world.wit
index f812e2a..e248d3b 100644
--- a/ferris-says-interface/wit/world.wit
+++ b/ferris-says-interface/wit/world.wit
@@ -1,4 +1,4 @@
-package tanishiking:ferris-says;
+package tanishiking:ferris-says@1.0.0;

 interface ferris-says {
     say: func(content: string, width: u32) -> string;
```

```sh
$ wkg wit build
WIT package written to tanishiking:ferris-says@1.0.0.wasm

$ wkg publish tanishiking:ferris-says@1.0.0.wasm
Published tanishiking:ferris-says@1.0.0
```

https://wa.dev/tanishiking:ferris-says

## Implement interface

```sh
$ cargo component new --lib --target tanishiking:ferris-says/ferrisify --namespace tanishiking ferrisify

$ cd ferrisify
$ cat Cargo.toml
[package]
name = "ferrisify"
version = "1.0.0"
edition = "2021"

[dependencies]
wit-bindgen-rt = { version = "0.34.0", features = ["bitflags"] }

[lib]
crate-type = ["cdylib"]

[profile.release]
codegen-units = 1
opt-level = "s"
debug = false
strip = true
lto = true

[package.metadata.component]
package = "tanishiking:ferrisify"
target = "tanishiking:ferris-says/ferrisify@1.0.0"

[package.metadata.component.dependencies]

$ cat src/lib.rs
#[allow(warnings)]
mod bindings;

use bindings::exports::tanishiking::ferris_says::ferris_says::Guest;

struct Component;

impl Guest for Component {
    fn say(content: String, width: u32) -> String {
        unimplemented!()
    }
}

bindings::export!(Component with_types_in bindings);

```

```sh
$ cargo add ferris-says
// and edit src/lib.rs to implement
$ cargo component build -r
```

## Publish Wasm Component

```sh
$ cargo component publish
```

https://wa.dev/tanishiking:ferrisify

## Implement CLI interface
```sh
$ cargo component new --target tanishiking:ferris-says/ferrisify-cli --namespace tanishiking ferrisify-cli
$ ferrisify-cli
$ cargo add clap --features derive
$ cargo add anyhow

$ cargo component build -r
```

## Plug components

```sh
$ wac plug target/wasm32-wasip1/release/ferrisify-cli.wasm --plug tanishiking:ferrisify -o result.wasm

$ wasmtime --wasm component-model result.wasm
Error: failed to run main module `result.wasm`

Caused by:
    0: import `wasi:cli/environment@0.2.0` has the wrong type
    1: instance export `get-environment` has the wrong type
    2: expected func found nothing
```

It seems like I was using too old wasmtime (v15.0.0)
[cargo-component と wasmtime の互換性が壊れた時の対処](https://zenn.dev/chikoski/articles/wasm-component-compatiblity-issue)

Install the latest version (v25.0.2)
https://github.com/bytecodealliance/wasmtime?tab=readme-ov-file#installation

```sh
$ wasmtime result.wasm hello 30
 _______
< hello >
 -------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \

```
