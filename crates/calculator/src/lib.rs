#[allow(warnings)]
mod bindings;

use bindings::{
    exports::works::calculator::calculate::{Guest, Op},
    works::adder::add_it,
};

struct Component;

impl Guest for Component {
    fn eval_expression(op: Op, x: u32, y: u32) -> u32 {
        match op {
            Op::Add => add_it::add(x, y),
        }
    }
}

bindings::export!(Component with_types_in bindings);
