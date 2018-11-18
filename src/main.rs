use std::io;

fn main() {
    println!("Welcome to temperature converter!");
    println!();
    println!("Would you like to convert to Celsius (C) or Fahrenheit (F)?");

    let mut ans = String::new();

    io::stdin().read_line(&mut ans)
        .expect("Failed to read line");

    let mut unit: char = match ans.trim().parse() {
        Ok(unit) => unit,
        Err(_) => {
            eprintln!("Invalid answer!");
            return
        }
    };

    for chr in unit.to_uppercase() {
        unit = chr;
    }

    let unit: char = unit;

    let other_unit = if unit == 'C' {
        'F'
    } else if unit == 'F' {
        'C'
    } else {
        eprintln!("Invalid answer!");
        return
    };

    println!("Enter temperature ({}): ", other_unit);

    ans = String::new(); // reset ans to empty string

    io::stdin().read_line(&mut ans)
        .expect("Failed to read line");

    let temp: f64 = match ans.trim().parse() {
        Ok(temp) => temp,
        Err(_) => {
            eprintln!("Invalid answer!");
            return
        }
    };

    let converted = if unit == 'C' {
        celsius_to_fahrenheit(temp)
    } else {
        fahrenheit_to_celsius(temp)
    };

    println!("Converted temperature: {} ({})", converted, unit);
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp * (9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) / (9.0 / 5.0)
}
