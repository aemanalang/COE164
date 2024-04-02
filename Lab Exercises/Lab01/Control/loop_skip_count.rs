fn main() {
    // infinite loop
    println!("infinite loop");
    println!(" t 2t 5t");
    let mut t = 1;

    loop {
        println!("{:>2} {:>2} {:>2}", t, t * 2, t * 5);

        t += 1;

        if t > 10 {
            break;
        }
    }

    // While loop
    println!("\nwhile loop");
    println!(" t 2t 5t");
    let mut t = 1;

    while t <= 10 {
        println!("{:>2} {:>2} {:>2}", t, t * 2, t * 5);

        t += 1;
    }

    // For loop
    println!("\nfor loop");
    println!(" t 2t 5t");

    for t in 1..=10 {
        println!("{:>2} {:>2} {:>2}", t, t * 2, t * 5);
    }
}