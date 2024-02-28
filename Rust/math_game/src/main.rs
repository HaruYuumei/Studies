use std::io;
use rand::Rng;

fn main() {

     //set random value
     let mut random_number1: i32;
     random_number1 = randomize();

     let mut random_number2: i32;
     random_number2 = randomize();
    
    println!("Hello, and welcome to the Rust textbased math game!");
    println!("The game is pretty simple, you will be presented with 5 math problems...");
    println!("if you answear right, you get one point, if you make a mistake you lose one point...");
    println!("Now that you know how it works, lets start the game:");
    println!("But first, tell me your name:");
    let user = set_username();
    let mut points: i32 = 0;


    println!("Hello {user}!");
    println!("The Game is starting now...");
    println!("The player is {} , you current have {} points!",get_username(user.clone()),get_points(points));
    
    println!("Question 1:");
    println!("What is {random_number1} + {random_number2}?");

    let answear = get_user_input();
    points = check_add(random_number1,random_number2,answear,points);


    println!("Question 2:");
    println!("You current have {} points!",get_points(points));
    random_number1 = randomize();
    random_number2 = randomize();
    println!("What is {random_number1} + {random_number2}?");
    let answear = get_user_input();
    points = check_add(random_number1,random_number2,answear,points);

    println!("Question 3:");
    println!("You current have {} points!",get_points(points));
    random_number1 = randomize();
    random_number2 = randomize();
    println!("What is {random_number1} + {random_number2}?");
    let answear = get_user_input();
    points = check_add(random_number1,random_number2,answear,points);

    println!("Question 4:");
    println!("You current have {} points!",get_points(points));
    random_number1 = randomize();
    random_number2 = randomize();
    println!("What is {random_number1} + {random_number2}?");
    let answear = get_user_input();
    points = check_add(random_number1,random_number2,answear,points);

    println!("Final Question!");
    println!("You current have {} points!",get_points(points));
    random_number1 = randomize();
    random_number2 = randomize();
    println!("What is {random_number1} + {random_number2}?");
    let answear = get_user_input();
    points = check_add(random_number1,random_number2,answear,points);
   

    println!("That's it!");
    println!("You ended the game, now lets see how many points you have!");
    println!("The player {}, end the game with {} out of 5 points he could get!",get_username(user.clone()),get_points(points));
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
fn set_points(add_point: i32,mut current_points: i32)->i32
{
    current_points = current_points + add_point;
    println!("[DEBUG] New Point: {}",current_points);
    return current_points;
}
fn get_points(cr_points: i32) -> i32
{
    return cr_points;
}

fn check_add(n1: i32,n2: i32,user: i32,mut point: i32)-> i32
{
    let result = n1 + n2;
    println!("Lets See the result......");
    if user == result
    {
        println!("Correct, {} plus {} is {} ",n1,n2,user);
        point = set_points(1,point);
        println!("[DEBUG] current point is: {point}");
        return point;
    }
    else
    {
        println!("Oh no!, {} plus {} is not {} ",n1,n2,user);
        println!("{} plus {} is {} ",n1,n2,result);
        point = set_points(-1,point);
        println!("[DEBUG] current point is: {point}");
        return point;
    }
    
}

fn randomize()-> i32
{
    let r = rand::thread_rng().gen_range(0..=10);
    return r;
}
