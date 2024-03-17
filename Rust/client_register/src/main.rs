use std::io;
use std::cmp::Ordering;

fn main() {

    
    let cliente1 = create_new_user();
    
    show_clients(&cliente1);

    println!("cliente has 18yo? {}",cliente1.isofage());
}

//structs

struct Client{
    name: String,
    last_name: String,
    age: i32,
    email: String,
    phone: i32,
}

impl Client{
    fn new_c(client_name:String,
            client_last_name:String,
            client_age:i32,
            client_email:String,
            client_phone_number:i32) -> Client
    {
        let new_client = Client{
            name: client_name,
            last_name: client_last_name,
            age: client_age,
            email: client_email,
            phone: client_phone_number,
        };
        new_client
        
    }


    fn isofage(&self) -> bool
    {
        let mut isage = false;
        match self.age.cmp(&18){
            Ordering::Greater | Ordering::Equal =>{
                isage = true;
            }
            Ordering::Less => {
               isage = false; 
            }
            _=>println!("oops"),
        }
        isage
    }
}
    

fn get_user_input() -> String
{
    let mut user_input = String::new();

    io::stdin()
    .read_line(&mut user_input)
    .expect("Cant read this line");
    user_input.trim().parse().unwrap()
}

fn get_user_input_integer() -> i32
{
    let mut user_input = String::new();

    io::stdin()
    .read_line(&mut user_input)
    .expect("Only numbers Here");
    user_input.trim().parse::<i32>().unwrap()
}

fn create_new_user() -> Client 
{
    println!("Enter Client First Name:");
    let client_name = get_user_input();

    println!("Enter Client Last Name:");
    let client_last_name = get_user_input();

    println!("Enter Client Age:");
    let client_age = get_user_input_integer();

    println!("Enter Client Email:");
    let client_email= get_user_input();
 
    println!("Enter Client Phone Number:");
    let client_phone_number = get_user_input_integer();

    let new_cliente = Client::new_c(client_name,client_last_name,client_age,client_email,client_phone_number);
    new_cliente

}

fn show_clients(um: &Client)
{
    println!("Client 1 Name:{}, Last name:{}, Age:{}, Email:{}, phone:{} !",um.name, um.last_name, um.age, um.email, um.phone);
}