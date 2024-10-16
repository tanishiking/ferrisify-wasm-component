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
