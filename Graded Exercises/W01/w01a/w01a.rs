use std::io;

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let n_resistors: usize = str_in.trim()
        .parse()
        .expect("Invalid number!");

    let mut r_series: f64 = 0.0;
    let mut r_parallel: f64 = 0.0;
    let mut r_input: f64 = 0.0; //container for the resistor input
    let mut c: usize = 1; //counter

    while c <= n_resistors {
        let mut str_in = String::new();

        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");
    
        let r_input: f64 = str_in.trim()
            .parse()
            .expect("Invalid number!");

        r_series = r_series + r_input;
        r_parallel = r_parallel + 1.0/r_input;

        c += 1;
    }
        r_parallel = 1.0/r_parallel;
        
        println!("{:.4}",   r_series);
        println!("{:.4}",   r_parallel);





    // TODO: Loop through each resistor and compute the series
    //       and parallel equivalent resistances
}