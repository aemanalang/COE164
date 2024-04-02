use std::io;

struct Player {
    name: String,
    pos: i64,
    item: PlayerItem,
}

impl Player {
    fn new(qty: u64) -> Self {
        if qty == 1
        {
            Self{
                name: String::from(""),
                pos: 0,
                item: PlayerItem{
                    name: String::from(""), 
                    item_type: ItemQtyType::Once(true),
                },
            }
        }
        else
        {
            Self{
                name: String::from(""),
                pos: 0,
                item: PlayerItem{
                    name: String::from(""), 
                    item_type: ItemQtyType::Consumable(qty),
                },
            }
        }
    }
        


    fn left(&mut self) {
        self.pos -= 1;
    }
    
    fn right(&mut self){
        self.pos += 1;
    }

}

struct PlayerItem {
    name: String,
    item_type: ItemQtyType,
}


enum ItemQtyType {
    Once(bool),
    Consumable(u64),
} 

fn main() {
    let mut str_in = String::new();
    let mut c = 0;

    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let t : usize = str_in.trim()
        .parse()
        .expect("Invalid number!");

    for c in 1..t+1  {

        str_in.clear();
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

        let split_in: Vec <&str> = str_in.splitn(3, ' ').collect();
        
        let p_name = split_in[0].trim().to_string();
        let ui_name =   split_in[1].trim().to_string();
        let ui_qty: u64 =   split_in[2].trim().parse().expect("Not a number!");
        
        let mut player = Player::new(ui_qty);

        player.name = p_name.clone();
        player.item.name = ui_name.clone();

        str_in.clear();
        io::stdin().read_line(&mut str_in).expect("Invalid input!");

        let n_cmd: u64 = str_in.trim().parse().expect("Input is not an integer!");

        println!("Player #{}:\nName: {}\nItem: {}x {}\n----------LOG----------", c, player.name, ui_qty, player.item.name);

        for _ in 0..n_cmd {
            str_in.clear();
            io::stdin().read_line(&mut str_in)
                .expect("Invalid input!");

            // TODO: Fill in the appropriate case with the related routines.
            // HINT: Use the declared Player struct earlier and edit and print its
            //       contents in the appropriate cases. 
            match str_in.trim() {
                "left" => {
                    player.left();
                    println!("New Position: {}", player.pos);
                }
                "right" => {
                    player.right();
                    println!("New Position: {}", player.pos);
                }
                "uitem" => {
                    match player.item.item_type{
                        
                        ItemQtyType::Once(value) => {
                            if value == false{
                                println!("Cannot use item as player does not have one.");
                            }
                            if value == true{
                                println!("Player used <{}>. It is now gone.",player.item.name);
                                player.item.item_type = ItemQtyType::Once(false);
                            }
                        }

                        ItemQtyType::Consumable(value) =>{
                            if value == 0{println!("Cannot use item as player does not have one.");}

                            else if value == 1{println!("Player used <{}>. It is now gone.",player.item.name); 
                            player.item.item_type = ItemQtyType::Consumable(value-1);}

                            else {println!("Player used <{}>. {}x of <{}> remains.", player.item.name, value-1, player.item.name);
                            player.item.item_type = ItemQtyType::Consumable(value-1);}

                            
                        }

                    }
                }
                _ => {
                    println!{"Not a known command"}
                }
            }
    
        }
    }
}