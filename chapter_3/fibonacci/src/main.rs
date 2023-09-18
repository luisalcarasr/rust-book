fn main() {
    let mut number = String::new();

    println!("Enter a number: ");

    std::io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please enter a number"),
    };

    println!("The {}th fibonacci number is {}", number, fibonacci(number));
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
