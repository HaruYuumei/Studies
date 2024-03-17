use std::io;
use std::cmp::Ordering;

fn main() {
    println!("                                                                        
    _____             _              _____                     _           
   |     |___ ___ ___| |_ ___ ___   |   __|___ ___ ___ ___ ___| |_ ___ ___ 
   | | | | . |   |_ -|  _| -_|  _|  |  |  | -_|   | -_|  _| .'|  _| . |  _|
   |_|_|_|___|_|_|___|_| |___|_|    |_____|___|_|_|___|_| |__,|_| |___|_|  
                                                                          
                                                                          ");

    println!("Welcome to my DnD Monster Generator!");
    println!("This Program will help you by creating random monsters");
    println!("The generator work like this:");
    println!("######################################################################");
    println!("Step one: Monster Cr");
    println!("(1):1/8, (2):1/4, (3):1/2, (4):1");
    println!("");//user input
    println!("Step two: Monster Type:");
    println!("(1):Humanoid, (2):Beast, (3):Fly, (4):Mythic");
    println!("");//user input
}

struct Monsters{
    monster_type: String,
    class_rate: String,
    monster_stats: (i32,i32,i32,i32,i32,i32),
    monster_bonus: (i32,i32,i32,i32,i32,i32),
    
}