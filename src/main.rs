fn main() {
    let print_vector = |x: &Vec<i32>| {
        println!("{:?}", x);
    };

    let v = vec![3, 2, 1];

    // & - instead of giving control of v over to the closure,
    // let it borrow my vector for a while
    print_vector(&v);

    // This now works fine.
    println!("v[0] = {}", v[0]);

    let mut a = 40;

    // The tutorial says that the braces must be necessary so that b stops borrowing a but it works fine without too.
    {
        // b becomes a mutabale reference to a.
        let b = &mut a;

        // * - follow the reference
        *b += 2;
    }

    println!("a = {}", a);

    let mut z = vec![3, 2, 1];

    for i in &z {
        println!("i = {}", i);

        // Doesn't work because it would overload the vector.
        //z.push(5);
    }
}
