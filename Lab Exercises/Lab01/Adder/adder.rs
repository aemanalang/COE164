use std::io;

fn main() {
    println!("Please input two \
        space-separated integers");

    let mut str_in = String::new();
    io::stdin()
        .read_line(&mut str_in)
        .expect("Failed to read input");

    let str_in_split: Vec<&str> = str_in
        .split(' ')
        .collect ();

    if str_in_split.len() != 2{
        panic!("Input does not contain two integers!");
    }

    let a: u64 = str_in_split[0]
        .trim()
        .parse()
        .expect("Input is not an integer!");

    let b: u64 = str_in_split[1]
    .trim()
    .parse()
    .expect("Input is not an integer!");

    let c = a + b;
    println!("Calculation: {a} + {b} = {c}");
}