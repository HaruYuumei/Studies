use std::io;

fn main() {
    println!("Write something!");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read Line");

    let words = get_first_word(&input);
    println!("{}",words);

}

fn get_first_word(user_input: &String) -> &str
{
    let first_word = user_input.as_bytes();

    for(i,&item) in first_word.iter().enumerate()
    {
        if item == b' '{
            return &user_input[0..i];
        }
    }
    &user_input[..]
}