use std::io;

fn main() {
    println!("Hello, and welcome to the Rust textbased math game!");
    println!("The game is pretty simple, you will be presented with a math problem...");
    println!("if you answear right, you get one point, if you make a mistake you lose one point...");
    println!("Now that you know how it works, lets start the game:");
    println!("But first, tell me your name:");
    let user = set_username();
    let mut points: i32 = 0;
    let add_point: i32 = 1;
    let sub_point: i32 = -1;

    println!("Hello {user}!");
    println!("The Game is starting now...");
    println!("The player is {} , you current have {} points!",get_username(user),get_points(points));
    
    println!("What is 1 + 1?");
    let a: i32 = 2;
    let answear = get_user_input();
    println!("user typed {}",answear);
    check_add(answear,a);

}

fn set_username() -> String
{
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .unwrap();
    return name.trim().parse().unwrap();
}

fn get_user_input() ->i32
{
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .unwrap();
    return input.trim().parse().unwrap();
}

fn get_username(name: String) -> String
{
    return name;
}
fn set_points(add_point: i32,current_points: i32)
{
    let mut result = current_points + add_point;
    println!("New Point: {}",result);
}
fn get_points(cr_points: i32) -> i32
{
    return cr_points;
}

fn check_add(mut n1: i32,mut n2: i32) -> i32
{

    if n1 == n2
    {
        let mut r = 1 + 1;
      println!("Correct, {} plus {} is {} ",1,1,r);
    }
    return n2;
}
