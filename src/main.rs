use gccjit::Context;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let c = Context::default();
}
