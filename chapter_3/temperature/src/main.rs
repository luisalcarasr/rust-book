fn main() {
    let mut fahrenheit = String::new();

    println!("Enter a temperature in Fahrenheit: ");

    std::io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please enter a number"),
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("{}°F is {}°C", fahrenheit, celsius);
}
