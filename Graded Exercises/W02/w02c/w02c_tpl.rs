/*
    Note: the derive attribute, #[derive(Debug, Copy, Clone, Partial Eq)],
    placed at the beginning of enums and structs allows them to be printable
    using the "{:?}". Furthermore, this would allow them to be copied from
    memory and do logic operations on them. 
*/

use std::io;

#[derive(Debug, Copy, Clone, PartialEq)] 
enum ItemType{
    Beverage,
    Cleaners,
    Electronics,
    Fruits,
    Meats,
    None,
}

#[derive(Debug, Copy, Clone, PartialEq)] 
struct GroceryItem{
    item: ItemType,
    price: f64,
    weight: f64,
}

impl GroceryItem {
    fn new() -> Self {
        Self{
            item: None,
            price: 0.0,
            weight: 0.0,
        }
    }

#[derive(Debug, Copy, Clone, PartialEq)]
struct SmartCart{
    items: [GroceryItem; 10], 
    max_budget: f64,
    max_weight: f64, 
    current_value: f64, 
    current_weight: f64,
    current_size: usize,
}

impl SmartCart{
    fn new(max_budget: f64) -> SmartCart{
        Smartcart{
        items: [GroceryItem.new(); 10],
        max_budget: max_budget,
        max_weight: 12.0, 
        current_value: 0.0, 
        current_weight: 0.0,
        current_size: 0.0,
        }
    }
    }

    fn add_item(&mut self, grocery_item: GroceryItem){ 
        // TODO: Create a routine that modifies the SmartCart when
        //       an item is added
    }

    fn remove_item(&mut self, index: usize){
        // TODO: Create a routine that modifies the SmartCart when
        //       an item is removed
    }

    fn show_info(self){
        // TODO: Create a routine that prints the contents of the
        //       cart according to specs.
}

}

fn main(){
    let mut str_in = String::new(); 

    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");
    let budget: f64 = str_in.trim().parse().expect("Not a number!"); 

    str_in.clear();
    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");
    let n_cmd: u64 = str_in.trim().parse().expect("Input is not a decimal number!");

    let mut cart = SmartCart::new(budget);
    
    // TODO: Create a routine that reads and executes the listed command, as 
    //       well as the error handling requirements. 
}