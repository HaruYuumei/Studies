
//====================Guessing Game
//======================to-do list:
//-Get User Name=== ok!
//-Generate a Random number=== ok!
//-Get user guess number=== ok!
//-Compare the numbers=== ok!
//-If correct number: win=== ok!
//-if wrong number: restart=== ok!

//======================Extras:
//-Create dificulties (higher dificulty = more numbers)=== ok!
//-Create score count, every time you got right you get one point===
//-Create Hp, if player miss loses 1 hp, if hits 0 ends the game


use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {  

// user name    
let user_input_string: String;

// Generating a random number using rand::Rng;
let _magic_number: i32 = rand::thread_rng().gen_range(1..=10);

//variable to recieve user input in integer
let mut user_input_integer: i32;

//tuple used to set the game difficulty
let mut dif = (0,0);
/*


Small story just to fill the game


*/
println!("Hello, and welcome to the amazing unique master blaster Guessing Game!!!!");
println!("First things first: What's your name?");

//Getting the user input in String to assing the user name
user_input_string = set_user_string();

println!("Welcome {}! a warm round of applause to the player!",user_input_string);
println!("Clap~ Clap~ Clap~ Clap~");
println!("If you don't know how this works, its simple:");
println!("First choose you difficulty: 1- easy 2- Medium 3- Hard");

//Getting the user input in a Integer to be used to define the game difficulty
user_input_integer = set_user_integer();
//calling a function to apply the game difficulty
dif = change_dif(user_input_integer);

println!("All you have to do now is guess the magic number!");
println!("Now please, tell me... What is the magic number???");

    //Game loop, asking the player to guess the magic number untill he got it right
    loop{
        //re-creating a random number every time the loop loads
        let magic_number: i32 = rand::thread_rng().gen_range(dif.0..=dif.1);

        //debugging to show what is the random number
        println!("[debug] the number is: {}",magic_number);

        //getting user input to define the user guess number
        user_input_integer = set_user_integer();

        //rp purpose, showing what the user choose
        println!("{}: its {}",user_input_string,user_input_integer);
        
        //using std::cmp::Ordering to compare the user guess number with the magic number
        match user_input_integer.cmp(&magic_number) {
            //if the user guess number is lower or higher than the magic number it will restart the loop
            Ordering::Less | Ordering::Greater => 
            {
                println!("Your guess it WRONG!!!");
                println!("Please, try again!");
            }
            //if the user guess number is the magic number, the game ends!
            Ordering::Equal =>
            {
                println!("Your guessed the magic number!");
                break;
            } 
            
            
        }

    }
}

//function to set the user input on a String, returning the string
fn set_user_string() -> String
{
    let mut user_string: String = String::new();
    io::stdin()
    .read_line(&mut user_string)
    .expect("Failed to read line");
    return user_string.trim().parse().unwrap();
}
//function to get user input as integer and returning the number inputed
fn set_user_integer() -> i32
{
    let mut user_string: String = String::new();
    io::stdin()
    .read_line(&mut user_string)
    .expect("Not a Number!!!");
    return user_string.trim().parse::<i32>().unwrap();
}

//function to change the difficulty based on the user choice
fn change_dif(user_dif: i32) -> (i32,i32)
{
    match user_dif
{
    1 =>{
        println!("Easy Difficulty Selected!");
        let mut d = (0,0);
        d.0 = 1;
        d.1 = 5;
        return (d.0,d.1);
    }
    
    2 =>{
        println!("Medium Difficulty Selected!");
        let mut d = (0,0);
        d.0 = 1;
        d.1 = 10;
        return (d.0,d.1);
    }
    
    3 =>{
        println!("Hard Difficulty Selected!");
        let mut d = (0,0);
        d.0 = 1;
        d.1 = 50;
        return (d.0,d.1);
    }
    _ =>{
        println!("error");
        return (0,0);
    } 
}
}