use std::io;
use std::string::String;

struct StudentEnrollInfo {
    sn: u64,
    is_eligible: bool,
    has_accountables: bool,
    has_taken_ge2017: [bool; 10],
}

impl StudentEnrollInfo {
    fn new() -> Self {
        Self{
            sn: 0,
            is_eligible: false,
            has_accountables: false,
            has_taken_ge2017: [false; 10],
        }
    }

    fn check_ge2017(&mut self, course: String) -> bool {
        // HINT: Create a routine that edits self.has_taken_ge2017
        //       depending on the course argument
        let ge_2017 = [
            "arts1", "fil40", "kas1", "philo1", "eng13",
            "speech30", "sts1", "drmaps", "socsci2", "socsci1"];

        for c in 0..9{
            if ge_2017[c] == course{
                self.has_taken_ge2017[c] = true;
            }

        }
        return false;
    }

    fn print_unsatisfied_ge2017(&self) -> bool {
        // HINT: Create a routine that prints the state of self.has_taken_ge2017
        //       according to the specs
        let ge_2017 = [
            "arts1", "fil40", "kas1", "philo1", "eng13",
            "speech30", "sts1", "drmaps", "socsci2", "socsci1"];

        for c in 0..4{
            if self.has_taken_ge2017[c] == false
            {
                print!("{} ",ge_2017[c].to_ascii_uppercase());
            }
        }
        let mut c = 4;
        while c < 9{
            if self.has_taken_ge2017[c] == false && self.has_taken_ge2017[c+1] == false
            {
                print!("{}/{} ",ge_2017[c].to_ascii_uppercase(), ge_2017[c+1].to_ascii_uppercase());
            }
            c += 2;
        }   

        println!("");
        return false;
    }
}

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let n_students: usize = str_in.trim()
        .parse()
        .expect("Invalid number!");

    for t in 1..=n_students {
        str_in.clear();
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

        let mut a_student = StudentEnrollInfo::new();

        let split_in: Vec <&str> = str_in.split(' ').collect();

        let n_cmd: u64 = split_in[0].trim().parse().expect("Not a number!");
        let sn: u64 = split_in[1].trim().parse().expect("Not a number!");

        a_student.sn = sn;

        for _ in 0..n_cmd {
            str_in.clear();
            io::stdin().read_line(&mut str_in)
                .expect("Invalid input!");

            let split_in: Vec <&str> = str_in.splitn(2, ' ').collect();

            let cmd: char = split_in[0].trim().parse().expect("Not a character!");
            let cmd_arg = split_in[1].trim();


            match cmd{
                'e' => {
                    if cmd_arg == "y" {a_student.is_eligible = true;}
                }

                'a' => {
                    if cmd_arg == "y" {a_student.has_accountables = true;}
                }

                'c' => {
                    a_student.check_ge2017(cmd_arg.to_string());
                }

                _ => { println!("Not a known command")

                }
            }

          
            // TODO: Create a routine that populates a struct with the provided values
            //       either through methods or initialization
            // TODO: Create a routine that prints to the standard output according to
            //       the specs
        }
        println!("Student #{}:",t);
        println!("Record for SN {}", a_student.sn);
        println!("\tIs eligible? {}",if a_student.is_eligible {"YES"} else {"NO"});
        println!("\tHas accountabilities? {}", if a_student.has_accountables {"YES"} else {"NO"});
        println!("\tUnsatisfied GE2017 Courses:");
        print!("\t");
        a_student.print_unsatisfied_ge2017();
    }
}