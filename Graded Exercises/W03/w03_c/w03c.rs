use std::io;
use std::fmt;
use std::collections::HashMap;
#[derive(Clone, Debug,PartialEq)]
struct SplitDate{
    year: u64,
    month: u8,
    day: u8,
}
#[derive(Clone, Debug,PartialEq)]
struct LentItem <'a>{
    name: String,
    acquire_date: SplitDate,
    borrowed_by: Option<&'a Borrower>,
}
#[derive(Clone, Debug,PartialEq)]
struct Borrower{
    name: String,
    reg_date: SplitDate,
}


impl<'a> LentItem<'a>{
    fn new(name_i: String, year_i: u64, month_i: u8, day_i: u8) -> Self{
        Self{
            name: name_i,
            acquire_date: SplitDate{
                year: year_i,
                month: month_i,
                day: day_i,
            },
            borrowed_by: None,
        }
    }

    fn borrow(&mut self, to:&'a Borrower) -> Option<&'a Borrower>{
        if let Some(orig_borrow) = self.borrowed_by {
            return Some(orig_borrow);
        }
        else {
        self.borrowed_by = Some(to);
        return None;
        }
    }

    fn unborrow(&mut self) -> Option<&'a Borrower>{
        if let Some(orig_borrow) = self.borrowed_by {
            self.borrowed_by = None;
            return Some(orig_borrow);
        }
        else {
        return None;
        }
    }

    fn transfer(&mut self, from: &'a Borrower, to: &'a Borrower) -> 
        (Option<&'a Borrower>, Option<&'a Borrower>){

            match self.borrowed_by{
                Some(from_orig) => {
                    if from.name == from_orig.name && from.name != to.name{
                            self.borrowed_by = Some(to);
                            (Some(from_orig), (Option::Some(&to)))
                    } else{
                        (Some(from_orig), None)
                    }

                }
                None => {
                    return (None, None);
                }

            }
    }


}

impl Borrower{
    fn new(name_i: String, year_i: u64, month_i: u8, day_i: u8) -> Self{
        Self{
            name: name_i,
            reg_date: SplitDate{
                year: year_i,
                month: month_i,
                day: day_i,    
            },
        }
    }

    fn borrowed_items <'a> (&'a self, items: &'a Vec<&'a LentItem>) -> Vec<&'a LentItem>{
        let mut items_borrowed: Vec<&LentItem> = Vec::new();
        for x in 0..items.len(){
            
            match items[x].borrowed_by{
                Some(borrower) =>{
                    if borrower.name == self.name {

                        if items_borrowed.len() == 0 {items_borrowed.push(items[x]); continue;}
                        
                        for y in 0..items_borrowed.len(){
                            if items[x].name < items_borrowed[y].name {items_borrowed.insert(y, items[x]); break;}
                            else{if y == items_borrowed.len() - 1 {items_borrowed.push(items[x]); break;}}
                            } 
                        }
                }
                _ =>{}
            }
        }
        return items_borrowed;
    }
}

// This has been implemented for you as an example
impl fmt::Display for Borrower{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Borrower({}) [Registered {}]", self.name, self.reg_date)
    }
}

impl fmt::Display for LentItem<'_>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(borrower) = self.borrowed_by{
        write!(f, "Lentitem({}) [Acquired {}] [Borrowed by {}]", self.name, self.acquire_date, borrower.name)}
    
    else {write!(f, "Lentitem({}) [Acquired {}]", self.name, self.acquire_date)}
    
    }
}

impl fmt::Display for SplitDate{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn main() {
    let mut str_in = String::new();
    let mut borrower_list: HashMap <String, Borrower> = HashMap::new();
    let mut items_list: HashMap <String, LentItem> = HashMap::new();

    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let n_borrowers: usize = str_in.trim().parse().expect("Invalid number!");

    for _ in 0..n_borrowers {
        str_in.clear();
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

        let split_in: Vec <&str> = str_in.splitn(2, ' ').collect();
        let split_date: Vec <&str> = split_in[0].trim().splitn(3, '-').collect();

        borrower_list.insert(split_in[1].trim().to_string(), Borrower::new(
            split_in[1].trim().to_string(),
            split_date[0].parse::<u64>().expect(""),
            split_date[1].parse::<u8>().expect(""),
            split_date[2].parse::<u8>().expect(""),
        ));
    }

    str_in.clear();
    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let n_items: usize = str_in.trim().parse().expect("Invalid number!");

    for _ in 0..n_items {
        str_in.clear();
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

        let split_in: Vec <&str> = str_in.splitn(2, ' ').collect();
        let split_date: Vec <&str> = split_in[0].trim().splitn(3, '-').collect();
        
        items_list.insert(split_in[1].trim().to_string(), LentItem::new(
            split_in[1].trim().to_string(),
            split_date[0].parse::<u64>().expect(""),
            split_date[1].parse::<u8>().expect(""),
            split_date[2].parse::<u8>().expect(""),
        ));
    }

    str_in.clear();
    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let n_cmd: usize = str_in.trim().parse().expect("Invalid number!");

    for _ in 0..n_cmd {
        str_in.clear();
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

        let split_in: Vec <&str> = str_in.splitn(2, ' ').collect();
        let cmd: &str = split_in[0].trim();
        let subcmd: String = split_in[1].trim().to_string();

        match cmd {
            "bi" => {
                if let Some(borrower) = borrower_list.get(&subcmd) {
                    println!("[BINFO] {}", borrower);
                    println!("    -----BORROWED ITEMS-----");

                    let items_list_vals = items_list.values().collect();
                    let borrowed_items_list = borrower.borrowed_items(&items_list_vals);

                    if borrowed_items_list.is_empty() {
                        println!("    NONE");
                    }
                    else {
                        for v in &borrowed_items_list {
                            println!("    {}", v);
                        }
                    }
                }
                else {
                    println!("[BINFO] Borrower \"{}\" not found!", subcmd);
                }
            }
            "ii" => {
                if let Some(item) = items_list.get(&subcmd) {
                    println!("[IINFO] {}", item);
                }
                else {
                    println!("[IINFO] Item \"{}\" not found!", subcmd);
                }
            }
            "t" => {
                let split_subcmd: Vec <&str> = subcmd.splitn(3, ' ').collect();
                let item_t: String = split_subcmd[0].trim().to_string();
                let from_t: String = split_subcmd[1].trim().to_string();
                let to_t: String = split_subcmd[2].trim().to_string();

                if let None = items_list.get(&item_t) {
                    println!("[TRANSFER] Item \"{}\" not found!", item_t);
                    continue;
                }

                if let None = borrower_list.get(&from_t) {
                    println!("[TRANSFER] FROM borrower \"{}\" not found!", from_t);
                    continue;
                }

                if let None = borrower_list.get(&to_t) {
                    println!("[TRANSFER] TO borrower \"{}\" not found!", to_t);
                    continue;
                }

                if let (Some(item), Some(from_b), Some(to_b)) = (items_list.get_mut(&item_t), borrower_list.get(&from_t), borrower_list.get(&to_t)) {
                    let (old_from_b_w, new_b_w) = item.transfer(from_b, to_b);

                    if let Some(_) = new_b_w {
                        println!("[TRANSFER] Item \"{}\" transfered from \"{}\" to \"{}\"!", item.name, from_b.name, to_b.name);
                    }
                    else if let Some(old_from_b) = old_from_b_w {
                        if from_b.name == old_from_b.name {
                            println!("[TRANSFER] Item \"{}\" is already borrowed by requester \"{}\"!", item.name, to_b.name);
                        }
                        else {
                            println!("[TRANSFER] Item \"{}\" cannot be transferred as it is currently borrowed by \"{}\", not \"{}\"!", item.name, old_from_b.name, from_b.name);
                        }
                    }
                    else {
                        println!("[TRANSFER] Item \"{}\" does not have a borrower!", item.name);
                    }
                }
            }
            "b" => {
                let split_subcmd: Vec <&str> = subcmd.splitn(2, ' ').collect();
                let item_t: String = split_subcmd[0].trim().to_string();
                let borrower_t: String = split_subcmd[1].trim().to_string();

                if let None = items_list.get(&item_t) {
                    println!("[BORROW] Item \"{}\" not found!", item_t);
                    continue;
                }

                if let None = borrower_list.get(&borrower_t) {
                    println!("[BORROW] Borrower \"{}\" not found!", borrower_t);
                    continue;
                }

                if let (Some(item), Some(borrower)) = (items_list.get_mut(&item_t), borrower_list.get(&borrower_t)) {
                    if let Some(old_borrower) = item.borrow(&borrower) {
                        if old_borrower.name == borrower.name {
                            println!("[BORROW] Item \"{}\" already borrowed by requester and current borrower \"{}\"!", item.name, borrower.name);
                        }
                        else {
                            println!("[BORROW] Item \"{}\" cannot be borrowed by \"{}\" as it is currently borrowed by \"{}\"!", item.name, borrower.name, old_borrower.name);
                        }
                    }
                    else {
                        println!("[BORROW] Item \"{}\" borrowed by \"{}\"!", item.name, borrower.name);
                    }
                }
            }
            "u" => {
                if let Some(item) = items_list.get_mut(&subcmd) {
                    if let Some(borrower) = item.unborrow() {
                        println!("[UNBORROW] Item \"{}\" unborrowed from \"{}\"!", item.name, borrower.name);
                    }
                    else {
                        println!("[UNBORROW] Item \"{}\" has no borrower!", item.name);
                    }
                }
                else {
                    println!("[UNBORROW] Item \"{}\" not found!", subcmd);
                }
            }
            _ => {}
        }
    }
}