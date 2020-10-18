use std::mem;

// String and str types are not the same!

fn main() {
    // Looks like the default type is &str.
    // &str = string slice.
    // let s: &str = "hello there!";
 
    // But the tutorial says it's `*'static str`
    // static = it's hardcoded into the program rather than being in memory.
    // Must be utf-8?
    let s: &'static str = "hello there!";

    // Won't work with the static type.
    // s = [abc];

    // Won't work with any kind of string?
    // Because we'd be actually referencing the bytes, rather than the char?
    // let h = s[0];

    // Chan use chars though...
    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    // heap
    // String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        // letters.push(',');
        letters.push_str(", ");
        a += 1;
    }

    println!("{}", letters);

    // &str conver to String
    // De-ref conversion.
    let u: &str = &letters;

    // Concatenation
    // String + str
    // let _z = letters + "abc";

    // Tutorial says this should work because it will cause a de-ref convert, but it doesn't seem to.
    // let _z = letters + &letters;

    // Make a string from a string slice.
    // let mut abc = String::from("hello world");
    let mut abc = "hello world!".to_string();

    abc.remove(0);
    abc.push_str("!!!");

    println!("{}", abc.replace("ello", "goodbye"));
}
