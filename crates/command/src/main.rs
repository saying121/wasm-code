use std::fmt::Display;

use clap::Parser;

use self::bindings::works::{calculator, calculator::calculate::Op};

#[allow(warnings)]
mod bindings;

fn parser_operator(op: &str) -> anyhow::Result<Op> {
    match op {
        "add" => Ok(Op::Add),
        _ => anyhow::bail!("Unknown operation: {}", op),
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
        }
    }
}

#[derive(Parser)]
pub struct Command {
    x:  u32,
    y:  u32,
    #[clap(value_parser = parser_operator)]
    op: Op,
}

impl Command {
    fn run(self) {
        let res = calculator::calculate::eval_expression(self.op, self.x, self.y);
        println!("{} {} {} = {} ", self.x, self.op, self.y, res);
    }
}

fn main() {
    Command::parse().run()
}
