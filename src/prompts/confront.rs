use std::any::Any;
use std::io;
use std::io::{Read, Write};
use std::process::exit;
use rand::Rng;
use crate::prompts::{Inventory, prompt};
use colored::Colorize;

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


pub fn confront(inv: &Inventory){
    println!("Enter E to peek your head around the corner, and assess the situation");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_ascii_lowercase().eq("e"){
        shoot(inv)
    } else{
        confront(inv);
    }
}

pub fn shoot(inv: &Inventory){
    let mut player_health:i32 = 100;

    println!("---------------------------------------------------------------------------------------------------------------------------------");
    println!("You see 3 henchmen. {}", "It's time to shoot it out!".on_bright_red());
    println!("For every shot you take, you risk taking return fire from the henchmen.");
    println!("You must defeat all the henchmen while keeping your health above 100.");
    println!("---------------------------------------------------------------------------------------------------------------------------------");

    let target1 = Henchman::new("Roger", 125, 25);
    let target2 = Henchman::new("Steve", 45, 75);
    let target3 = Henchman::new("Tracy", 55, 40);

    let mut targets = vec![target1, target2, target3];
    let mut targets_size = targets.len();

    if inv.weapon{
        println!("You decide to use your shotgun, giving you 1.7x damage");
    }

    while player_health > 0 {

        if targets.is_empty(){
            println!("You eliminated all of the targets. Check their bodies for a message? (y/n)");
            if prompt(){
                searchBodies();
            } else{
                println!("Game over. You failed to find out why they came.");
                break;
            }
            break;
        }
        println!("Enemies: ");
        for (i, target) in targets.iter().enumerate(){
            println!("{}: {} (Health: {}, Damage: {})", i + 1, target.name.white(), target.health.to_string().white(), target.damage.to_string().white());
        }
        println!("Your health points: {}", player_health.to_string().bright_green());

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
        if target_choice >= targets.len() {
            println!("Target {} has already been eliminated.", target_choice + 1);
            continue;
        }

        let mut player_attack: u32 = 0;
        let mut player_hit_chance: f32 = 0.0;

        if inv.weapon {
            player_attack = rand::thread_rng().gen_range(40..=75);
            player_hit_chance = rand::thread_rng().gen_range(0.25..1.0);
        } else{
            player_attack = rand::thread_rng().gen_range(20..=40); // 20 to 60 possible damage
            player_hit_chance = rand::thread_rng().gen_range(0.0..1.0);
        }


        if player_hit_chance <=  0.35{
            println!("You missed!");
            let target = &mut targets[target_choice];
            let target_attack: u32 = rand::thread_rng().gen_range(5..=target.damage);
            let target_hit_chance: f32 = rand::thread_rng().gen_range(0.0..1.0);

            if target_hit_chance >= 0.40 {
                player_health -= target_attack as i32;
                println!("{} hit you for {} damage!", target.name, target_attack.to_string().bright_red().bold());
            } else{
                println!("{} misses!", target.name);
            }

        } else{
            let target = &mut targets[target_choice];
            let damage_dealt = player_attack;
            println!("You hit {} for {} damage!", target.name, damage_dealt.to_string().yellow().bold());
            if damage_dealt > 45 && damage_dealt > target.health as u32 {
                println!("{}", "Critical hit!".cyan().italic());
            }
            target.take_damage(damage_dealt);

            if !target.is_alive(){
                println!("You defeated {}", target.name.red());
                targets_size -= 1;
                println!("{} targets remaining", targets_size.to_string().blue());
                targets.remove(target_choice);
            } else{
                let target_attack: u32 = rand::thread_rng().gen_range(5..=target.damage);
                let target_hit_chance: f32 = rand::thread_rng().gen_range(0.0..1.0);

                if target_hit_chance >= 0.35 {
                    player_health -= target_attack as i32;
                    println!("{} hit you for {} damage!", target.name, target_attack.to_string().bright_red().bold());
                } else{
                    println!("{} misses!", target.name);
                }
            }
        }



        if player_health.lt(&(0)){
            println!("{}", "You took a fatal shot. Game over!".on_bright_red());
            println!("Hint: Perhaps you need a better weapon.");

        }
        println!("---------------------------------------------------");

    }
}

pub fn searchBodies(){
    let ascii = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
|	o ____________________ 	   x	|
|   *	*/  |         *       |	x	|
|	$$ |   ##########   | 	o	|
|   *	*$$ |__ ######*#### *  |       x|
|	$$    $$ |          |		|
|	*$$$$$$$$ |  *########* |	|
|	      *$$ | #######*# |		|
|*	o    *  $$ |          |	 *	|
|	      $$/   #######  |		|
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~";

    println!("{}", "You rummage through their pockets, looking for any sign of why they came...".italic());
    println!("{}", "You find a number written on a tattered old note... but what is it?".italic());
    println!("(Enter the number to continue)");
    let mut attempts = 3;
    while attempts > 0 {
        println!("{}", ascii.bright_cyan());

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("err");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number");
                continue;
            }
        };

        if input == "47".parse().unwrap() {
            println!("You deciphered the number, but are left with many questions...");
            win();
            break;
        }
        else{
            println!("Incorrect. Try again");
            attempts -= 1;
            println!("Remaining attempts: {}", attempts);
        }
    }

    if attempts == 0{
        println!("Though you fought off the attackers, you don't know who they are or why they came.");
        println!("Many restless nights ahead...");
        println!("Game Over");
    }


}

pub fn win(){
    // check if has key. if not, println!("The attackers managed to put a lock on your front door. Find the key to proceed!")
    println!("Game over. You win!");
    exit(0);
}