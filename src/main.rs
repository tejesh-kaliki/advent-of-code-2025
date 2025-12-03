mod days;

fn main() {
    let day = std::env::args().nth(1).unwrap_or_else(|| "1".into());
    match day.as_str() {
        "1" => days::day01::run(),
        "2" => days::day02::run(),
        "3" => days::day03::run(),
        _ => println!("Unknown day"),
    }
}
