fn main() {
    let temp = 23;

    // Like: `let day = temp > 20 ? "sunny" : "cloudy";`
    let day = if temp > 20 { "sunny" } else { "cloudy" };

    println!("day is {}", day);
}
