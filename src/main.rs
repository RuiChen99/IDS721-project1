use std::io;
use std::io::Write;

fn main() {
    println!("Happy Chinese New Year!  Do you know what your Chinese Zodiac is?");

    let mut input = String::new();
    let x: i64;

    print!("Enter your birth year: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    x = input.trim().parse::<i64>().unwrap();

    let n = (x - 4) % 12;
    let mut s = String::new();

    if n == 0 {s.push_str("rat ");}
    if n == 1 {s.push_str("ox ");}
    if n == 2 {s.push_str("tiger ");}
    if n == 3 {s.push_str("rabbit ");}
    if n == 4 {s.push_str("dragon ");}
    if n == 5 {s.push_str("snake ");}
    if n == 6 {s.push_str("horse ");}
    if n == 7 {s.push_str("goat ");}
    if n == 8 {s.push_str("monkey ");}
    if n == 9 {s.push_str("rooster ");}
    if n == 10 {s.push_str("dog ");}
    if n == 11 {s.push_str("pig ");}

    let result = s;


    println!("Your Chinese Zodiac is: {}", result);
}