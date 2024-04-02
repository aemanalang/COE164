use std::io;

fn solve_factorial(number : u128) -> u128{

    if number<=1 {
        return 1;
    }

    return number * solve_factorial(number-1);
}

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let n_testcases: u64 = str_in.trim().parse().expect("Not an integer!"); 

    for a in 1..n_testcases + 1{
        str_in.clear();
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

        let split_in: Vec <&str> = str_in.splitn(3, ' ').collect();

        let cmd = split_in[0];
        let r_desired: f64 = split_in[1].trim().parse().expect("Not a float!");
        let x: f64 = split_in[2].trim().parse().expect("Not a number!");
        
        let mut temp: f64 = 0.0;
        let mut factorial: f64 = 0.0; 
        let mut n: i32 = 0;
        
        while n < 21{
            
            factorial = solve_factorial(n as u128) as f64;

            if x.powi(n)/factorial <= r_desired {break}

            if cmd == "e"{
                temp = temp + x.powi(n)/solve_factorial(n as u128) as f64;
        
            } 

            else if cmd == "s"{
                temp = temp + (if n == 0{1.0} else if n%2 == 0 {1.0} else {-1.0})/     
                    ((solve_factorial((2 * n + 1) as u128)) as f64) * x.powi(2 * n + 1);
                
  
            }

            else if cmd == "c"{
                temp = temp + (if n == 0{1.0} else if n%2 == 0 {1.0} else {-1.0}) /
                    ((solve_factorial((2 * n) as u128)) as f64) * x.powi(2 * n);
            }
            
            n += 1; 
            
        }
            
        println!("Function #{}: {:.2}", a, temp);
    }
}