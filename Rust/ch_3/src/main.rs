use std::io;
use std::cmp::Ordering;
fn main()
{
    // OWNERSHIP

    // s is not valid outside the scope
    {

        let s="Hello!"; // s became valid from this point
        println!("{s}"); // s is used in the scope
    }
    // the scope ended, so 's' is no longer valid

    let mut test = String::from("Hello");
    test.push_str(",world!");
    println!("{}",test);

    //clone
    let clone1 = String::from("hello");
    let clone2 = clone1.clone();

    println!("Clone1 = {}, clone2 = {}",clone1,clone2);
    

//functions
    let s2 = String::from("hello");
    println!("s2 prints: {}!!",s2);
    let s3 = fn1(s2);
    println!("s3 prints {}. s2 doesn't!!",s3);


    //references and borrowing


    let mut user_password = String::new();
    println!("Type your password:");
    io::stdin()
    .read_line(&mut user_password)
    .expect("password bugged");

    let pwlen = pw_length(user_password);
    let minsize = 8;

    match pwlen.cmp(&minsize) {
        Ordering::Greater=>
        {
            println!("Strong Password");
        }
        Ordering::Equal|Ordering::Less=>
        {
            println!("Passwords require more than 8 chars");
        }
        _=>println!("uh oh"),
    }




        //At any given time, you can have either one mutable reference or any number of immutable references.
        //References must always be valid.

}

fn fn1(a_string:String) -> String{
    a_string
}
fn pw_length(pw: &String) -> usize
{
    pw.len()
}