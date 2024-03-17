
/* Struct Studies  */

struct Player {
    name: String,
    level: u64,
    ca:u64,
    health:i64,
}
impl Player{
    fn sethealth(&mut self,newhealth: i64)
    {
        self.health = self.health + newhealth;
    }
}


struct Enemy{
    name: String,
    level: u64,
    ca:u64,
    health:i64,
}
impl Enemy{
    fn sethealth(&mut self,newhealth: i64)
    {
        self.health = self.health + newhealth;
    }
}

fn main() {

    let mut user: Player = Player{
        name:String::from("Haru"),
        level: 10,
        ca:16,
        health:20,
    };

    let mut aienemy: Enemy = Enemy{
        name:String::from("Skeleton"),
        level: 8,
        ca:12,
        health:14,
    };

    println!("Player info:");
    println!("|Name:{}, Health:{},|",user.name,user.health);
    println!("|C.a:{}, Level:{},|",user.ca,user.level);


}
