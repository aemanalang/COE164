use std::io;

pub trait Media{
    fn play(&self);
    
    fn title(&self) -> String;
    
    fn artist(&self) -> String;
    
}
#[derive(Clone, Debug)]
struct Song{
    title: String,
    artist: String,
}

impl Media for Song{
    fn play(&self){
        println!("Now Playing: {} by {}",self.title(), self.artist());
    }
    
    fn title(&self) -> String{
        return self.title.to_string();
    }
    
    fn artist(&self) -> String{
        return self.artist.to_string();
    }
    
}

struct Queue<T>{
    list: Vec<T>,
}

impl <T: Media> Queue <T>{
    fn new() -> Self{
        Self {list: Vec::new()}
    }

    fn add(&mut self, song: T){
        if self.list.len() < 12{
        self.list.push(song);
        println!("Successfully added {} by {} to the queue!", self.list[self.list.len() - 1].title(), self.list[self.list.len() - 1].artist());
        }
        else { println!("Queue is full! {} by {} is dropped.", song.title(), song.artist());
    }
    }
    
    fn show_queue(&self){
        if self.list.len() > 0 {
            println!("-----mEEEdia bot-----");
            let mut c = 1;
            for x in &self.list{
                println!("{}. {} by {}", c, x.title(), x.artist());
                c += 1;
            }
            println!("---------------------");
        }
        else {println!("No media in queue.")}
    }
    
    fn play(&mut self){
        if self.list.len() > 0{
            let next_song = self.list.remove(0);
            next_song.play();
        } else {
            println!("Queue is empty! No media to play...");
        }
    }
}
fn main() {
    let mut songlist : Vec<Song> = Vec::new();

    songlist.push(Song { title: "OMG".to_string(), artist: "New Jeans".to_string() });
    songlist.push(Song { title: "Perfect Night".to_string(), artist: "LE SSERAFIM".to_string() });
    songlist.push(Song { title: "Raining in Manila".to_string(), artist: "Lola Amour".to_string() });
    songlist.push(Song { title: "Never Gonna Give You Up".to_string(), artist: "Rick Astley".to_string() });
    songlist.push(Song { title: "Mananatili".to_string(), artist: "Cup of Joe".to_string() });
    songlist.push(Song { title: "Aphrodite".to_string(), artist: "The Ridleys".to_string() });
    songlist.push(Song { title: "Hanggang sa Buwan".to_string(), artist: "Kenaniah".to_string() });
    songlist.push(Song { title: "Dumaloy".to_string(), artist: "SUD".to_string() });

    let mut queue: Queue<Song> = Queue::new();
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in)
    .expect("Invalid input!");

    let t: usize = str_in.trim()
    .parse()
    .expect("Invalid number!");

    for _ in 0..t{
        str_in.clear();
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

        let (n_cmd, songinput) = match str_in.find(' ') {
            Some(index) => {
                let (left, right) = str_in.split_at(index);
                (left.trim(), right.trim())
            },
            None => (str_in.trim(), ""),
        };
    
        match n_cmd{
            "add" => {
                for song in &songlist{
                    
                    if song.title() == songinput{
                        queue.add((*song).clone());
                        break;
                    }
                }
                
            }
            
            "play" => {
                    queue.play();

            }
            
            "show_queue" => {
                    queue.show_queue();
            }

            _ =>{

            }
        }

    }

}