use std:: io;

fn main() {

    let mut value = String::new() ;
    let mut fahrenheit_or_celcius = String::new();

    println!("Value you want to convert");
    io::stdin().read_line(&mut value).expect("Need to enter a value");

    println!("Is this temp in f or c? (enter f or c)");
    io::stdin().read_line(&mut fahrenheit_or_celcius).expect("Need to enter a value: f or c");

    let value: i32 = value.trim().parse().expect("Not a number");
    let fahrenheit_or_celcius: char = fahrenheit_or_celcius.trim().parse().expect("Not a character");


    if fahrenheit_or_celcius == 'c' {
        println!("{}", (value * 9/5) + 32);
    } else if fahrenheit_or_celcius == 'f' {
        println!("{}", (value - 32) * 5/9);
    } else {
        println!("Unexpected error try again");
    }
}
