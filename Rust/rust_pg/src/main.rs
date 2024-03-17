use std::io;
use std::cmp::Ordering;
use std::

struct Player{

    username: String,
    user_level: i32,
    user_combat_skills: (i32,i32,i32),

}

impl Player{
    fn new_player() -> Player
    {
        let u:Player = Player{
            username:String::from("Haru"),
            user_level: 1,
            user_combat_skills: (1,1,1),
        };
        u
    }  
    fn set_level(&mut self)
    {
        let new = (self.user_combat_skills.0 + 19,
                   self.user_combat_skills.1 + 19,
                   self.user_combat_skills.2 + 19);
        self.user_combat_skills = new;
        
    } 
    fn cal_level(&self) ->i32
    {
        let max_lvl = 3;
        let mut result = self.user_combat_skills.0 + self.user_combat_skills.1 + self.user_combat_skills.2;
        result /= max_lvl;
        result
    }

    fn set_cbt(&mut self)
    {
        let new = self.cal_level();
        self.user_level = new;
    }

}


fn main() {

    let a = 3;


    let mut player_one: Player = Player::new_player();
    player_one.set_cbt();
    println!("{},{}",player_one.username,player_one.user_level);
    println!("{},{},{}",player_one.user_combat_skills.0,player_one.user_combat_skills.1,player_one.user_combat_skills.2);

    player_one.set_level();
    player_one.set_cbt();

    println!("{},{}",player_one.username,player_one.user_level);
    println!("{},{},{}",player_one.user_combat_skills.0,player_one.user_combat_skills.1,player_one.user_combat_skills.2);
}