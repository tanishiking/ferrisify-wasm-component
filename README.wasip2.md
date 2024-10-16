In Oct 2024, when building with the target `wasm32-wasip1`, it appears that WASI preview2 is being used, but in reality, an adapter for WASI preview1 is also generated.
This example creates a Wasm Component that uses the WASI preview2 API to print "hello" to standard output.

## Define an interface
```sh
$ mkdir -p hello-wasip2-wit/wit
$ cd hello-wasip2-wit
$ cat wit/world.wit
package tanishiking:hello-wasip2@0.1.0;

world run {
    import wasi:cli/stdout@0.2.0;
    export wasi:cli/run@0.2.0;
}

$ wkg wit build
$ wkg publish tanishiking:hello-wasip2@0.1.0.wasm
```

We might not need to publish the world to the registry, but I don't know a way to run `cargo component new` targets the local wit file.

## Implement
```sh
$ cd ..
$ ls
hello-wasip2-wit/
$ cargo component new --lib --namespace tanishiking --target tanishiking:hello-wasip2@0.1.0 hello-wasip2

$ cargo component build
// to check src/bindings.rs is generated
// implement
$ cat src/lib.rs
#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use bindings::wasi::cli::stdout::get_stdout;

struct Component;

impl Guest for Component {
    fn run() -> Result<(), ()> {
        let stdout = get_stdout();
        let s = "hello";
        stdout.write(s.as_bytes()).unwrap();
        stdout.flush().unwrap();
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);

$ cargo component build --target wasm32-unknown-unknown -r
// do not use wasm32-wasip1 to avoid using wasi preview1 adapter

$ wasmtime target/wasm32-unknown-unknown/release/hello_wasip2.wasm
hello%
```
