use ferris_says::say;
use std::io;
use std::io::{stdout, BufWriter};

#[derive(strum_macros::Display)]
enum Temperature {
    Celsius,
    Fahrenheit,
}

#[derive(strum_macros::Display)]
enum TemperatureInitials {
    C,
    F,
}

fn main() {
    print!("\x1B[2J\x1B[1;1H");

    println!("Please input the value");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let mut temperature_type = get_temperature_type(&mut user_input);
    let mut temperature_initial = get_temperature_initial(&mut temperature_type);
    let converted_initial = get_converted_initial(&mut temperature_type);
    let tempeature_number = get_number(user_input, &mut temperature_initial);
    let converted_number = convert_temperature(tempeature_number, &mut temperature_type);

    let message = String::from(format!(
        "{tempeature_number} °{temperature_initial} is equal to {converted_number} °{converted_initial}",
    ));
    print_message(message)
}

fn print_message(message: String) {
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn convert_temperature(tempeature_number: f32, temperature_type: &mut Temperature) -> f32 {
    match temperature_type {
        Temperature::Fahrenheit => (tempeature_number - 32.0) * 5.0 / 9.0,
        Temperature::Celsius => (tempeature_number * 9.0 / 5.0) + 32.0,
    }
}

fn get_number(user_input: String, temperature_initial: &mut String) -> f32 {
    user_input
        .replace(&*temperature_initial, "")
        .trim()
        .parse()
        .expect("Please type a number!")
}

fn get_converted_initial(temperature_type: &mut Temperature) -> String {
    match temperature_type {
        Temperature::Celsius => TemperatureInitials::F.to_string(),
        Temperature::Fahrenheit => TemperatureInitials::C.to_string(),
    }
}

fn get_temperature_initial(temperature_type: &mut Temperature) -> String {
    match temperature_type {
        Temperature::Celsius => TemperatureInitials::C.to_string(),
        Temperature::Fahrenheit => TemperatureInitials::F.to_string(),
    }
}

fn get_temperature_type(text_with_temperature: &mut String) -> Temperature {
    if text_with_temperature.contains("C") {
        return Temperature::Celsius;
    } else if text_with_temperature.contains("F") {
        return Temperature::Fahrenheit;
    } else {
        panic!("Unknown temperature type");
    }
}
