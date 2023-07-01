use std::io;
mod test;

fn main() {
    println!("Print value and 'F' to convert Fahrenheit to Celsius or 'C' Celsius to Fahrenheit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Incorrect input!");

    let conversion_type = define_conversion_type(&input);
    let value_to_convert = get_f64_from_input(&input);
    let result = convert(value_to_convert, conversion_type);

    println!("Result is {result:.2} °{conversion_type}",);
}

fn get_f64_from_input(input: &String) -> f64 {
    let input: String = input
        .chars()
        .filter(|c| c.is_numeric() || c.eq(&'.'))
        .collect();

    input.parse().expect("Cannot read number")
}

fn convert(value: f64, conversion_type: &str) -> f64 {
    match conversion_type {
        "F" | "f" => (value * 1.8) + 32.0,
        "C" | "c" => (value - 32.0) * 5.0 / 9.0,
        _ => {
            panic!("Incorrect value, couldn't convert")
        }
    }
}

fn define_conversion_type(input: &String) -> &str {
    let input = input.trim().to_uppercase();
    if input.ends_with('F') {
        "C"
    } else if input.ends_with('C') {
        "F"
    } else {
        panic!("Input must contain C or F at the end!")
    }
}
