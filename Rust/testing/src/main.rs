use std::io;

// Using a single fn to get multiples strings and integer from user

fn main() {
    let name: String;
    let age: i32;
    let color: String;
    let day: i32;
    let month: i32;
    let year: i32;
    let mut game_stage: i32 = 0;
    gamestage_update(game_stage);
    let start = get_input_numbers();
    match start{
        1=> gamestage_update(start),
        2=>gamestage_update(start),
        _=>println!("pag error"),
    }
}


//Getting a String input from user
fn get_input_string()-> String
{
    let mut user_input = String::new();
    io::stdin()
    .read_line(&mut user_input)
    .unwrap();

    return user_input.trim().parse().unwrap();
}

//Getting a number Input from User
fn get_input_numbers() -> i32
{
    let mut user_input = String::new();
    io::stdin()
    .read_line(&mut user_input)
    .unwrap();
    
    return user_input.trim().parse::<i32>().expect("Input a number");
}

fn gamestage_update(gs: i32)
{
    match gs
    {
        0 =>
        println!("Would You like to start the game? 1- yes 2- no"),
        1 =>
        println!("Starting Game"),
        println!("Hello there, whats your name?"),
        name = get_input_string(),
        println!("Nice to meet you {name}!"),
        println!("Now {name} tell me, what's your age?"),
        age = get_input_numbers(),
        println!("Oooh So you are {age} years old huh?"),
        println!("ok, now that we know you are called {name}, and have {age} years old..."),
        println!("tell me, whats your favorite color?"),
        color = get_input_string(),
        println!("ahh.... of course its {color}!"),
        println!("now I really wanna know something about you..."),
        println!("What day is your Birthdate?"),
        day = get_input_numbers(),
        println!("and the month?..."),
        month = get_input_numbers(),
        println!("and the year?"),
        year = get_input_numbers(),
        println!("Hm... interesting thing that is {day}/{month}/{year}"),
        _=>println!("Game ended"),
    }
}