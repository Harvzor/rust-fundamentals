// x goes on the stack
fn print_value(x: i32) {
    println!("value = {}", x);
}

// & = reference
// mut = mutable
fn increase(x: &mut i32) {
    // *x is derefencing the reference.
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    print_value(33);

    let mut z = 1;

    // & = I'm letting them borrow
    increase(&mut z);

    println!("z = {}", z);

    let p = product(3, 5);

    println!("a = {}, b = {}, p = {}", 3, 5, p);
}
