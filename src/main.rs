fn main() {
    // v owns the memory that is stored
    // v itself is on the stack, likely a pointer?
    // v (the data) is on the heap
    let v = vec![1, 2, 3];

    // v2 is copying the pointer
    //let v2 = v;

    // Does not compile because v2 now has the pointer,
    // and rust ensures safety by making sure only one
    // variable points to the data.
    //println!("{:?}", v);

    // foo(v) also would borrow the value, not letting us use v anymore.
    //let foo = |v: Vec<i32>| ();
    //foo(v);

    let u = 1;
    let _u2 = u;

    // However this works because with primitive types, the value is copied.
    // Copying such simple types on the stack is fine because they take up such little memory.
    println!("u = {}", u);

    // Putting something deliverately on the heap means we must have a pointer here:
    let u3 = Box::new(1);
    // Which means a borrow occurs here:
    let _u4 = u3;

    // So this cannot use a borrowed value:
    //println!("u3 = {}", *u3);

    // This is an inconvenient way of controller ownership:
    let print_vector = |x: Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);

        // Return control of the vector...
        x
    };

    let vv = print_vector(v);
}
