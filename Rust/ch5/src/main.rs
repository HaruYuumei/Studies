use std::io;


struct Player {
    username: String,
    pw: String,
    vip: bool,
    ismod: bool,
    
 }

fn main() {
    //STRUCTS

 let user = create_player();
 let user2 = create_player();

 println!("username:{}, password:{}, Vip:{}, Mod:{}",user.username,user.pw,user.vip,user.ismod);
 println!("username:{}, password:{}, Vip:{}, Mod:{}",user2.username,user2.pw,user2.vip,user2.ismod);



}

fn create_player() -> Player
{
    println!("Type your username:");
    let namae = get_input_string();
    
    println!("Type your password:");
    let pasuworudo = get_input_string();

    Player{
        username: namae,
        pw: pasuworudo,
        vip:false,
        ismod:false,
    }
}

fn get_input_string() -> String
{
    let mut user_input:String = String::new();

    io::stdin()
    .read_line(&mut user_input)
    .expect("Cant read Line");

    user_input.trim().parse().unwrap()

}
