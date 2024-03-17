use std::io;

//Still have to add a way to choose a diferent operation after choosing one


fn main() {
    println!("Welcome to the most simple calculator ever made in the history of my house");
    loop{
        println!("Please select which calculation you want to do:");
        println!("(1)-Add (2)-Sub (3)-Multiply (4)-Divide (5)-Exit app");
        let calc = set_integer();

            match calc{
                1.0 =>{
                    println!("type the first number you want to add:");
                    let mut n1 = set_integer();
                    println!("type the second number you want to add:");
                    let mut n2 = set_integer();
                    println!("Would you like to edit a number?");
                    println!("0-Don't edit, 1-Edit number 1({}), 2- Edit number 2({})",n1,n2);
                    let change = set_integer();
                    match change{
                        1.0=>{
                            println!("type the new number:");
                            n1 = set_integer();
                        }
                        2.0=>{
                            println!("type the new number:");
                            n2 = set_integer();
                        }
                        0.0=>{
                            println!("not editing the numbers");
                        }
                        _=>println!("alright"),
                    }
                    println!("Calculando:");
                    add(n1,n2);
                }
                2.0=>{
                    println!("type the first number you want to subtract:");
                    let mut n1 = set_integer();
                    println!("type the second number you want to subtract:");
                    let mut n2 = set_integer();
                    println!("Would you like to edit a number?");
                    println!("0-Don't edit, 1-Edit number 1({}), 2- Edit number 2({})",n1,n2);
                    let change = set_integer();
                    match change{
                        1.0=>{
                            println!("type the new number:");
                            n1 = set_integer();
                        }
                        2.0=>{
                            println!("type the new number:");
                            n2 = set_integer();
                        }
                        0.0=>{
                            println!("not editing the numbers");
                        }
                        _=>println!("alright"),
                    }
                    println!("Calculando:");
                    sub(n1,n2);
                }
                3.0=>{
                    println!("type the first number you want to multiply:");
                    let mut n1 = set_integer();
                    println!("type the second number you want to multiply:");
                    let mut n2 = set_integer();
                    println!("Would you like to edit a number?");
                    println!("0-Don't edit, 1-Edit number 1({}), 2- Edit number 2({})",n1,n2);
                    let change = set_integer();
                    match change{
                        1.0=>{
                            println!("type the new number:");
                            n1 = set_integer();
                        }
                        2.0=>{
                            println!("type the new number:");
                            n2 = set_integer();
                        }
                        0.0=>{
                            println!("not editing the numbers");
                        }
                        _=>println!("alright"),
                    }
                    println!("Calculando:");
                    mult(n1,n2);
                }
                4.0=>{
                    println!("type the first number you want to Divide:");
                    let mut n1 = set_integer();
                    println!("type the second number you want to Divide:");
                    let mut n2 = set_integer();
                    println!("Would you like to edit a number?");
                    println!("0-Don't edit, 1-Edit number 1({}), 2- Edit number 2({})",n1,n2);
                    let change = set_integer();
                    match change{
                        1.0=>{
                            println!("type the new number:");
                            n1 = set_integer();
                        }
                        2.0=>{
                            println!("type the new number:");
                            n2 = set_integer();
                        }
                        0.0=>{
                            println!("not editing the numbers");
                        }
                        _=>println!("alright"),
                    }
                    println!("Calculando:");
                    divide(n1,n2);
                }
                5.0=>{
                    println!("ooh... ok bye bye!");
                    break;
                }
                
                _=>println!("error"),
            }
    }
    

}

fn set_integer()->f64 
{
    let mut user_input: String = String::new();
    io::stdin()
    .read_line(&mut user_input)
    .expect("Gibe number dumbass!"); 
    return user_input.trim().parse::<f64>().unwrap();
}

fn add(a:f64,b:f64)
{
    let c:f64 = a + b;
    println!("{a} + {b} = {c}");

}

fn sub(a:f64,b:f64)
{
    let c:f64 = a - b;
    println!("{a} - {b} = {c}");

}

fn mult(a:f64,b:f64)
{
    let c:f64 = a * b;
    println!("{a} * {b} = {c}");

}

fn divide(a:f64,b:f64)
{
    let c:f64 = a / b;
    println!("{a} / {b} = {c}");

}