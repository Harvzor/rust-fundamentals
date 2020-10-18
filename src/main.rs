use std::mem;

fn main() {
    // Like a List in C#, grows dynamically.
    // Must be mutable, otherwise you can't change it...
    let mut a =  Vec::new();

    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(44);

    println!("a = {:?}", a);

    // Cannot use signed values.
    // Must be the same size at the machine.
    // let index: usize = 111;

    // a[index] = 321;

    // Panics.
    // println!("a[0] = {}", a[index]);

    // Returns Option type.
    match a.get(3) {
        Some(x) => println!("a[3] = {}", x),
        None => println!("error, no such element"),
    }

    // No idea why the & is there, doesn't seem to change anything.
    for x in &a {
        println!("{}", x);
    }

    // Also returns an Option since we can't be sure there are any elements to pop.
    let last_elem = a.pop();

    println!("last elem is {:?}, a = {:?}", last_elem, a);

    // Could be a none so this doesn't work.
    // let Some(_last_value) = a.pop();

    // While `a.pop()` yields a Some, do...
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}
