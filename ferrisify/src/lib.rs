#[allow(warnings)]
mod bindings;

use bindings::exports::tanishiking::ferris_says::ferris_says::Guest;

struct Component;

impl Guest for Component {
    fn say(content: String, width: u32) -> String {
        let mut buffer = Vec::new();
        ferris_says::say(content.as_str(), width.try_into().unwrap(), &mut buffer).unwrap();
        return String::from_utf8(buffer).unwrap();
    }
}

bindings::export!(Component with_types_in bindings);
