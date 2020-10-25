fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a doxen",
        // I could use z.
        _z @ 9..=11 => "a lot",
        _ if (x % 2 == 0) => "some",
        _ => "a few",
    }
}

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{} I have {} oranges", x, how_many(x));
    }

    let point = (0, 1);

    match point {
        (0, 0) => println!("origin"),
        // y could be a reference or mutable, rather than just a value.
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (_, y) => println!("(?, {})", y),
    }
}
