// It's basically inserted into the rest of the application.
// There's no fixed address.
const MEANING_OF_LIFE: u8 = 42;

// Actually has an address.
// Isn't normally mutable because any part of the application could read/write the var and break memory safety.
static mut Z: i32 = 123;

fn main() {
    println!("meaning of life =  {}", MEANING_OF_LIFE);

    // I promise to not mess up Z
    unsafe
    {
        Z = 777;
        println!("Z =  {}", Z);
    }
}
