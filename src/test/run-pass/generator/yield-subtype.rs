// run-pass
#![allow(dead_code)]
#![allow(dead_code)]

// revisions:lexical nll
//[nll]compile-flags: -Z disable-nll-user-type-assert
#![cfg_attr(nll, feature(nll))]

#![feature(generators)]

fn bar<'a>() {
    let a: &'static str = "hi";
    let b: &'a str = a;

    || {
        yield a;
        yield b;
    };
}

fn main() {}
