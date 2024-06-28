#[allow(warnings)]
mod bindings;

use bindings::exports::works::adder::add_it::Guest;

struct Component;

impl Guest for Component {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}

bindings::export!(Component with_types_in bindings);
