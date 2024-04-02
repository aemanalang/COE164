use rand::Rng;

fn main() {
    // Create a random number generator
    let mut rng = rand::thread_rng();
    
    let length = rng.gen_range(1..10);
    let max = 10u32.pow(length) - 1;
    let length = length as usize;
    
    // Format the random number as a string with leading zeros
    let command = format!("{} {:0>length$} {:0>length$}", length, rng.gen_range(0..=max), rng.gen_range(0..=max));

    // Print the result
    println!("{}", command);
}
