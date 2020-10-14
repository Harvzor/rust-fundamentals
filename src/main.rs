use std::mem;

fn main() {
    // Const
    let a: u8 = 123;

    println!("a = {}", a);

    // Mutable
    let mut b: i8 = 0;

    println!("b = {}", b);

    b = 42;

    println!("b = {}", b);

    // Auto typed as i32
    let mut c = 123456789;

    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    c = -1;

    println!("c = {} after mod", c);

    // isize or usize, will be maximum of the system
    let z:isize = 123;

    let size_of_z = mem::size_of_val(&z);

    println!("z = {}, takes up {} bytes, {}-bit os", z, size_of_z, size_of_z * 8);

    // 4 bytes because it accepts any ascii char
    let d: char = 'x';

    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    // Double-precisition, f64
    let e = 2.5;

    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // Boolean
    let g = false;

    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}
