use std::io::{self, Write};
fn main() {
    user_input();
}

// user interface
fn user_input() {
    println!("Main Menu - Press the number of the option you want to select: ");
    println!("1. Celcius \n2. Farhenite \n3. Kelvin");
    print!("Enter your choice: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let scale = ["Celcius", "Farhenite", "Kelvin"];
    match choice.trim().parse::<i32>() {
        Ok(choice) => match choice {
            1 | 2 | 3 => println!("You selected: {}", scale[choice as usize - 1]),
            _ => {
                println!("No such option exists in menu! bye bye.");
                return;
            }
        },
        Err(_) => {
            println!("Erm that ain't a number! bye bye.");
            return;
        }
    }

    let choice: i32 = choice.trim().parse().unwrap();

    print!(
        "\nEnter the temperature in {}: ",
        scale[choice as usize - 1]
    );
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(_) => break,
            Err(_) => {
                println!("Erm that ain't a number! Try again.");
                print!(
                    "\nEnter the temperature in {}: ",
                    scale[choice as usize - 1]
                );
                io::stdout().flush().expect("Failed to flush stdout");
            }
        }
    }

    let input: f64 = input.trim().parse::<f64>().unwrap();
    let unit = scale[choice as usize - 1];
    println!("You entered: {}{}", input, unit);

    println!("What do you want to conver {} temp into?", unit);
    println!("1. Celcius \n2. Farhenite \n3. Kelvin");
    print!("Enter your choice: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut to_convert_to = String::new();
    loop {
        to_convert_to.clear();
        io::stdin()
            .read_line(&mut to_convert_to)
            .expect("Failed to read line");

        match to_convert_to.trim().parse::<i32>() {
            Ok(1) | Ok(2) | Ok(3) => break,
            _ => {
                println!("Broda which option is that? Try again.");
                print!("Enter your choice: ");
                io::stdout().flush().expect("Failed to flush stdout");
            }
        }
    }
    let to_convert_to: i32 = to_convert_to.trim().parse::<i32>().unwrap();

    if choice == to_convert_to {
        println!(
            "\nAfter converting {}{} to {}{} you get {}{}",
            input, unit, input, unit, input, unit
        );
        println!("---i aint no dumb dumb, bye bye.---");
        return;
    }
    match to_convert_to {
        1 => println!(
            "\n{}{} in Celcius is: {:.2}C",
            input,
            unit,
            convert_to_celcius(input, unit)
        ),
        2 => println!(
            "\n{}{} in Farhenite is: {:.2}F",
            input,
            unit,
            convert_to_farhenite(input, unit)
        ),
        3 => println!(
            "\n{}{} in Kelvin is: {:.2}K",
            input,
            unit,
            convert_to_kelvin(input, unit)
        ),
        _ => println!("Invalid selection"),
    }
}

// conversion functions
fn convert_to_celcius(temp: f64, unit: &str) -> f64 {
    match unit {
        "Farhenite" => (temp - 32.00) * 5.00 / 9.00,
        "Kelvin" => temp - 273.15,
        _ => 0.00,
    }
}

fn convert_to_farhenite(temp: f64, unit: &str) -> f64 {
    match unit {
        "Celcius" => (temp * 9.00 / 5.00) + 32.00,
        "Kelvin" => (temp - 273.15) * 9.00 / 5.00 + 32.00,
        _ => 0.00,
    }
}

fn convert_to_kelvin(temp: f64, unit: &str) -> f64 {
    match unit {
        "Celcius" => temp + 273.15,
        "Farhenite" => (temp - 32.00) * 5.00 / 9.00 + 273.15,
        _ => 0.00,
    }
}
