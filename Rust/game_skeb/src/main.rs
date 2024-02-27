use std::io;
fn main() 
{


    println!("Welcome to the bad game I've made it");
    println!("First, what should I call you?");
    
    //player Username Input
    let mut user = get_username();
    
    println!("Alright {user} lets start the game them!");

    let mut user_class = select_class();

    if user_class == 1
    {
        println!("You've choosen the Warrior Class.");
    }
    if user_class == 2
    {
        println!("You've choosen the Mage Class.");
    };
    if user_class == 3
    {
        println!("You've choosen the Rogue Class."); 
    };
    println!("Are you sure you want to use this class? 1-Yes 2-No");
    let mut classchanger = change_class();
    if(classchanger == 2)
    {
        select_class();
    }
}


    fn get_class() -> i32
    {
        let mut class:String = String::new();
        io::stdin().read_line(&mut class).unwrap();
        return class.trim().parse().unwrap();
    }
    fn get_username() -> String
    {
        let mut username = String::new();
        io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
        return username;
    }
    fn select_class() -> i32
    {
        let mut returned;
        println!("Please Select a Class that you would like to play!");
        println!("1-Warrior, 2-Mage, 3-Rogue");
        returned = get_class();
        return returned;
    }
    fn change_class() ->i32
    {
        let mut change = String::new();
        io::stdin()
        .read_line(&mut change)
        .unwrap();
    
        return change.trim().parse().unwrap();
    }