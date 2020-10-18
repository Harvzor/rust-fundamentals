use std::mem;

fn main() {
    // Array, 5 i32s.
    // Array is where I know how many elements I have.
    // Not variable size, use vector for that?
    let mut a: [i32;5] = [1, 2, 3, 4, 5];

    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;

    println!("a[0] s {}", a[0]);

    // Output debug information with `{:?}`.
    println!("a s {:?}", a);

    if a == [321, 2, 3, 4, 5] {
        println!("does match");
    }

    // 10 elements, all are 1.
    let b = [1u16; 10];

    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    // Each u16 takes up 2 bytes, so 10*2:
    println!("b took up {} bytes", mem::size_of_val(&b));

    let matrix: [[f32;3];2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
    ];

    println!("{:?}", matrix);

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if i == j {
                println!("matrix[{}][{}] is {}", i, j, matrix[i][j]);
            }
        }
    }
}
