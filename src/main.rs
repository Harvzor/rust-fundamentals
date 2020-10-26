fn is_even(x: u32) -> bool {
    x % 2 == 0
}

fn main() {
    let limit = 500;
    let mut sum = 0;

    for i in 0..
    {
        let isq = i*i;

        if isq > limit {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }

    println!("loop sum = {}", sum);

    let sum2 = (0..)
        .map(|x| x*x)
        .take_while(|&x| x < limit)
        // x is a reference, so we need to dereference
        .filter(|x| is_even(*x))
        // fold takes the initial value and the folding function
        // if we wanted to sum, we should start at 0 for the starter value
        // if we wanted to multiply, we should start at 1
        // sum is the value of each fold foreached over, so sum is 0 initially
        .fold(0, |sum, x| sum + x);
        
    println!("higher order function sum = {}", sum2);
}
