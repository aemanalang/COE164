use std::io;

fn lock_solve(vec1: &Vec<u32>, vec2: &Vec<u32>, length: u32) -> u32 {
    let mut steps: u32 = 0;
    let mut diff: u32;
    for x in 0..length as usize
        {
            if vec1[x] <= vec2[x]{diff = vec2[x] - vec1[x];}
            
            else {diff = vec1[x] - vec2[x];}

            if diff > 5 {diff = 10 - diff;}
            
            steps += diff;
        }

        return steps;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let split_in: Vec <&str> = input.split(' ').collect();

    let length: u32 = split_in[0].trim().parse().expect("Not a number!");
    let lock_init: &str = split_in[1].trim();
    let lock_unlock: &str = split_in[2].trim();
    
    let lock_init_digits: Vec<u32> = lock_init
        .chars()
        .map(|c| c.to_digit(10).expect("Invalid digit"))
        .collect();

    let lock_unlock_digits: Vec<u32> = lock_unlock
        .chars()
        .map(|c| c.to_digit(10).expect("Invalid digit"))
        .collect();

    println!("{}", lock_solve(&lock_init_digits, &lock_unlock_digits, length));
}