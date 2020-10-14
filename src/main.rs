#![allow(dead_code)]

use std::mem;

// Stack is fast but limited.
// There's plenty of stack.

struct Point
{
    x: f64,
    y: f64,
}

fn origin() -> Point
{
    Point
    {
        x: 0.0,
        y: 0.0,
    }
}

fn stack_and_heap() {
    // Stack allocated by default.
    let p1 = origin();

    // Pointer to location on the stack.
    let p2 = Box::new(origin());

    // Should be 16 bytes since there's 2 f64 vars.
    println!("p1 takes up {} bytes", mem::size_of_val(&p1));

    // Should be 8 since this is a 8 byte system.
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    // Follow the point, relocate back to the stack in a new var.
    let p3 = *p2;

    println!("{}", p3.x);
}

fn main() {
    stack_and_heap();
}
