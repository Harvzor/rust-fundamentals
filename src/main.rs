fn main() {
    let country_code = 1000;

    let country = match country_code
    {
        44 => "UK",
        46 => "Sweden",
        // ..= is inclusive, using .. is currently "experimental"
        1..=999 => "unknown",
        _ => "invalid"
    };

    println!("country = {}" , country);
}
