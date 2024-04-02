use std::io;

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let n_resistors: usize = str_in.trim()
        .parse()
        .expect("Invalid number!");

    let mut r_series = 0.0;
    let mut r_parallel = 0.0;
    let mut c = 1;
    let mut r_array = [0u64; 5];


    while c <= n_resistors {
        str_in = String::new();
        io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

        r_array[c-1] = str_in.trim()
        .parse()
        .expect("Invalid number!");
    
        println!("{}", r_array[c]);
        c += 1;
    }
    println!("{:?}", r_array);

    // TODO: Loop through each resistor and compute the series
    //       and parallel equivalent resistances
}