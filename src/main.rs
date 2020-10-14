fn scope_and_shadowing() {
    let a = 123;

    {
        // This inner a shadows the outer a
        let a = 456;

        println!("a = {}", a);
    }

    println!("a = {}", a);
}

fn main() {
    scope_and_shadowing();
}
