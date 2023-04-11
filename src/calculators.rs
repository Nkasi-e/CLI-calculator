// The rust standard library std comes with an env module which allows to command line argument parsed when calling the program
// Note Some is a variant on the option enum

use std::env::{args, Args}; //name of fn (args), name of struct (Args)

pub fn calc() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    // parssing the values into number/float to be able to perform basic arithmetic, because arithmetic operations can't be performed on strings
    let first_number = first.parse::<f32>().unwrap();
    let second_number: f32 = second.parse().unwrap();
    let result = operate(operator, first_number, second_number);

    println!(
        "{:?}",
        output(first_number, operator, second_number, result)
    );
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    // instead of else if statement we use the match method
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operator used."),
    }
}
// fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
//     if operator == '+' {
//         first_number + second_number
//     } else if operator == '-' {
//         first_number - second_number
//     } else if operator == '*' {
//         first_number * second_number
//     } else if operator == '/' {
//         first_number / second_number
//     } else {
//         return 0.0;
//     }
// }

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    )
}
