use std::any::Any;
use std::io;
use std::io::Write;
use rand::Rng;

struct Henchman{
    name: String,
    health: i32,
    damage: u32,
}
impl Henchman{
    fn new(name: &str, health: i32, damage: u32) -> Henchman {
        Henchman {
            name: name.to_string(),
            health,
            damage,

        }
    }
    fn attack(&self) -> u32 {
        rand::thread_rng().gen_range(1..=self.damage)
    }
    fn take_damage(&mut self, damage: u32){
        self.health = self.health.saturating_sub(damage as i32);
    }

    fn is_alive(&self)-> bool{
        self.health > 0
    }

}


pub fn confront(){
    println!("E to peek your head around the corner, and assess the situation");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_ascii_lowercase().eq("e"){
        shoot()
    } else{
        confront();
    }
}

pub fn shoot(){
    let mut player_health = 100;

    println!("You see 3 henchmen. It's time to shoot it out!");
    println!("You will begin with 100 health. For every shot you take, you risk taking return fire from the henchmen.");
    println!("You must defeat all the henchmen while keeping your health above 100. If you have a health potion, you can use it by entering 'H'");

    let target1 = Henchman::new("Roger", 75, 15);
    let target2 = Henchman::new("Steve", 30, 35);
    let target3 = Henchman::new("Tracy", 55, 20);

    let mut targets = vec![target1, target2, target3];
    let mut targets_size = targets.len();

    while player_health > 0 {

        if targets.is_empty(){
            println!("You eliminated all of the targets. Check their bodies for a message?");
            //searchBodies();
            break;
        }

        println!("Targets: ");
        for (i, target) in targets.iter().enumerate(){
            println!("{}: {} (Health: {}, Damage: {})", i + 1, target.name, target.health, target.damage);
        }
        println!("Your health points: {}", player_health);

        println!("Select a target 1-3:");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("panic!");

        let target_choice: usize;

        match input.trim() {
            "1" => target_choice = 0,
            "2" => target_choice = 1,
            "3" => target_choice = 2,
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 3.");
                continue;
            }
        }

        // let target = &mut targets[target_choice];

        let player_attack: u32 = rand::thread_rng().gen_range(20..=60); // 20 to 60 possible damage
        let player_hit_chance: f32 = rand::thread_rng().gen_range(0.0..1.0);

        if player_hit_chance <=  0.35{
            println!("You missed!");
        } else{
            let target = &mut targets[target_choice];
            let damage_dealt = player_attack;
            println!("You hit {} for {} damage!", target.name, damage_dealt);
            if damage_dealt < 45 && damage_dealt > target.health as u32 {
                println!("Critical hit!");
            }
            target.take_damage(damage_dealt);

            if !target.is_alive(){
                println!("You defeated {}", target.name);
                targets_size -= 1;
                println!("{} targets remaining", targets_size);
                targets.remove(target_choice);
            } else{
                let target_attack: u32 = rand::thread_rng().gen_range(5..=target.damage);
                let target_hit_chance: f32 = rand::thread_rng().gen_range(0.0..1.0);

                if target_hit_chance >= 0.45 {
                    player_health -= target_attack;
                    println!("{} hit you for {} damage!", target.name, target_attack);
                } else{
                    println!("{} misses!", target.name);
                }
            }

        }



        if player_health.lt(&(0)){
            println!("You took a fatal shot. Game over!");
        }

    }


}