#[cfg(test)]
mod tests {
    extern crate phrases;

    #[test]
    #[should_panic]
    // #[ignore] If I don't want the test to run.
    fn english_greeting_correct() {
        assert_eq!("not hello", phrases::greetings::english::hello());
    }
}
