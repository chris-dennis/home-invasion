use std::io;
use std::time::{Duration, Instant};

fn prompt() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // accepts any answer starting with "y"
    return input.to_ascii_lowercase().starts_with("y");
}

// this could ask and then ask if
fn options(){
    println!("... you walk down the stairs towards the noise. You peak and see 3 dudes");
    println!("WHAT SHOULD WE DO? (1)confront, (2)run, or (3)hide?");
    // you can (1)confront, (2)run, or (3)hide. print this here
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // after I have saved the input in input, then we want to check if it is either 1 or 2 or 3
    // if the input is 1, then we call confront()
    if input.trim() == "1" {
        println!("you have chosen confront()");
        //confront();
    } else if input.trim() == "2"{
        println!("you have chosen run, we find the key and allat");
        // findKey()
    } else {
        println!("you chose hide and the robbers got to you!");
    // if the input is 3 or else, then they lose, we assume that the robbers get to you.
}

// we want to run() and then find the key and a strategy to escape
fn findKey() {
    println!("we get away from them");
    println!("You want to find a key and a strategy to leave");
    println!("You forgot where you put the key, so we go upstairs and search the rooms");
    println!("what do you want to search first? bathroom? storage room? your bedroom");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

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
            // here we have that if this is the case, then the next prompt would be:
            options();
            // ... you walk down the stairs towards the noise. You peak and see 3 dudes

            // 3 options: confront, hide, or run
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
