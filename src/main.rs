use std::mem;

// &[i32] = we're taking a slice = we're borrowing a part of an array of i32s
fn use_slice(slice: &mut[i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());

    slice[0] = 4321;
}

fn main() {
    let mut data = [1, 2, 3, 4, 5];

    // I'm letting the function borrow the 2nd, 3rd and 4th elements.
    // use_slice(&mut data[1..4]);

    use_slice(&mut data);

    println!("{:?}", data);
}
