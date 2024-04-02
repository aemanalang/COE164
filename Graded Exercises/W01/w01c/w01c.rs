use std::io;

fn get_network_from_prefix(prefix: u64, last_digit: u64) -> u64 {

    // TODO: Write a routine that will output the following numbers
    //       given the prefix and the last digit (i.e. first digit
    //       of the unique id)
    // 0 - Invalid prefix
    // 1 - Globe/TM
    // 2 - Smart/Sun/TNT
    // 3 - DITO

    if prefix == 917 && [3,5,6,8].contains(&last_digit) {return 1;} //Globe/TM
    
    else if prefix == 925 && [3,5,6,7,8].contains(&last_digit) {return 1;} //Globe/TM
    
    else if [817, 905, 906, 915, 916, 917, 926, 927, 935, 936, 937, 945, 953, 954,
    955, 956, 965, 966, 967, 975, 976, 977, 978, 979, 995, 996, 997].contains(&prefix) { return 1; } ////Globe/TM

    else if [922, 923, 924, 925, 931, 932, 933, 934, 940, 941, 942, 943, 973,
    974, 907, 909, 910, 912, 930, 938, 946, 948, 950, 908, 918, 919, 920, 921, 928, 929,
    939, 946, 947, 949, 951, 961, 998, 999].contains(&prefix) { return 2 } //Smart/Sun/TNT
    
    else if [895, 896, 897, 898, 991, 992, 993, 994].contains(&prefix)  { return 3; } //DITO
    
    else {0}

    
}

fn main() {
    let mut str_in = String::new();

    str_in.clear();
    io::stdin()
        .read_line(&mut str_in)
        .expect("Failed to read input");

    let n_testcases: u64 = str_in.trim()
        .parse()
        .expect("Input is not an integer!");

        let mut pre: u64;
        let mut lastd: u64;
        let mut mid: u64;
        let mut post: u64;

    for t in 1..=n_testcases {
        
        str_in.clear();
        io::stdin()
            .read_line(&mut str_in)
            .expect("Failed to read input");
        
        let phone_as_int: u64 = str_in.trim()
            .parse()
            .expect("Input is not an integer!");

        pre = phone_as_int / 10000000;
        lastd = phone_as_int / 1000000 % 10;
        mid = phone_as_int / 10000 % 1000;
        post = phone_as_int  % 10000;
        
        match get_network_from_prefix(pre, lastd){
            0 => println!("Case #{}: Invalid", t),
            1 => println!("Case #{}: Globe/TM | +63 {} {} {}", t, pre, mid , post),
            2 => println!("Case #{}: Smart/Sun/TNT | +63 {} {} {}", t, pre, mid, post),
            3 => println!("Case #{}: DITO | +63 {} {} {}", t, pre, mid, post),
            _ => println!("error"),
        }

        // TODO: Write a routine that will print out the mobile number and
        //       network provider according to the specs. Don't forget to use
        //       the get_network_from_prefix() function above!
    }
}