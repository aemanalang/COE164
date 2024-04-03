use std::io;
use std::string::String;
use std::collections::HashMap;

struct LetterFreq{
    dictionary: HashMap<char, u64>,
}

impl LetterFreq{

    fn new() -> Self{
        let mut dict: HashMap <char, u64> = HashMap::new();
        for c in 'a'..='z'{
            dict.insert(c, 0);
        }
            dict.insert(' ', 0);
        Self{
            dictionary: dict,
        }
    }

    fn count(&mut self, c : char){
        for (key, value) in &mut self.dictionary{
            if c == *key || c.to_ascii_lowercase() == *key {*value += 1;}
        }
    }

    fn current_counter(&self){
        for c in 'a'..='z'{
            for (key, value) in &self.dictionary{
                if c == *key {
                    if *value > 0 {println!("{c}: {value}");}
                }
            }
        }
        for (key, value) in &self.dictionary{
            if *key == ' ' {
                println!(" : {value}");
            }
        }
    }

}

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in)
    .expect("Invalid input!");

    let t: usize = str_in.trim()
    .parse()
    .expect("Invalid number!");

    for x in 1..=t{
        let mut lf = LetterFreq::new();

        str_in.clear();
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

            for c in str_in.chars(){
                lf.count(c);
            }
        println!("---LETTER FREQUENCY of CASE #{x}---");    
        lf.current_counter();
        println!("---------------------------------");

    }

}