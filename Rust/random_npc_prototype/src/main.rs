
//this is a prototype I've always wanted to try, the purpose of it is to generate a n number of npcs.
//to start with it, I want to determine how many parameters the npc will have, this will start with only a few
//but I aim to increase the more i learn about Rust and how it works


/* ===== npc stats =====
_____- Name:
_____- Age:
_____- Height:
_____- Weight:
_____- Occupation:
_____-

*/

use std::io;
use std::cmp::Ordering;

struct Npc {
    name: String,
    age:i32,
    height: f32,
    weight: f32,
    occupation:String,
    }

impl Npc {

    fn npc_data(&self) {
        println!("Name:{}",self.name);
        println!("Age:{}",self.age);
        println!("Height:{}",self.height);
        println!("Weight:{}",self.weight);
        println!("Occupation:{}",self.occupation);
    }
    fn alter_npc(&mut self,nname:String,nage:i32,nheight:f32,nweight:f32,noccupation:String)
    {
        self.name = nname;
        self.age = nage;
        self.height = nheight;
        self.weight = nweight;
        self.occupation = noccupation;
    }
    
}

fn main() {

    let mut npc1: Npc = Npc { name: String::from("()"), age: 0, height: 0.0, weight: 0.0, occupation: String::from("()") };

    println!("nome do npc?");
    let uname:String = get_input_string();
    println!("Npc Occupation?");
    let uoc:String = get_input_string();
    println!("Npc Age?");
    let ua:i32 = get_input_integer();
    println!("Npc height?");
    let uh:f32 = get_input_float();
    println!("Npc weight?");
    let uw:f32 = get_input_float();

    npc1.alter_npc(uname, ua, uh, uw, uoc);
    npc1.npc_data();
}

fn get_input_string()->String
{
    let mut user_input:String=String::new();
    io::stdin()
    .read_line(&mut user_input)
    .expect("deu ruim aqui rapaziada!");

    user_input.trim().parse().unwrap()
}
fn get_input_integer()->i32
{
    let mut user_input:String=String::new();
    io::stdin()
    .read_line(&mut user_input)
    .expect("deu ruim aqui rapaziada!");

    user_input.trim().parse::<i32>().unwrap()
}
fn get_input_float()->f32
{
    let mut user_input:String=String::new();
    io::stdin()
    .read_line(&mut user_input)
    .expect("deu ruim aqui rapaziada!");

    user_input.trim_end().parse::<f32>().unwrap()
}