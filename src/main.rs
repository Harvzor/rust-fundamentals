fn main() {
    // For 1 to 11, excluding 11, do...
    for x in 1..11 {
        println!("x = {}", x);
    }

    for (pos, x) in (30..41).enumerate() {
        println!("pos = {}, x = {}", pos, x);
    }

    // This doesn't seem to work, says e is (usize, i32)
    // for e in (30..41).enumerate() {
    //     println!("e", e);
    // }
}
