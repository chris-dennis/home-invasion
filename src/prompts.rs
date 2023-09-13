use std::io;
use std::time::{Duration, Instant};

fn prompt() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // accepts any answer starting with "y"
    return input.to_ascii_lowercase().starts_with("y");
}

pub fn intro() {
    println!("You hear a faint knock at the door. Investigate? (y/n)");
    if prompt() {
        investigate();
    } else {
        println!("Must have been the wind...");
    }
}

pub fn investigate() {
    println!("You take a look at the window, but see nothing.");
    println!("Suddenly, you hear the front door being kicked down!");
    println!("You run to the gun safe.");
    println!("You must enter the gun safe combination as shown within 6 seconds! Begin? (y/n)");

    if prompt() {
        println!("677301");
        if !qte() {
            println!("The robbers got you. Game over");
        } else {
            println!("You grab your trusty Mississippi Carbine.");
        }
    } else {
        println!("The robbers got you. Game over");
    }
}
// quick time event but not really
pub fn qte() -> bool {
    let time = Duration::from_secs(6);

    let start_time = Instant::now();

    while start_time.elapsed() < time {
        println!("Go!");
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).unwrap();

        if user_input.trim() == "677301" && start_time.elapsed() < time {
            return true;
        }
        return false;
    }

    return false;
}
