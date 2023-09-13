use std::io;
use std::time::{Duration, Instant};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn prompt() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // accepts any answer starting with "y"
    return input.to_ascii_lowercase().starts_with("y");
}

// this could ask and then ask if
fn options() {
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
    } else if input.trim() == "2" {
        println!("you have chosen run, we find the key and allat");
        find_key();
    } else {
        println!("you chose hide and the robbers got to you!");
        // if the input is 3 or else, then they lose, we assume that the robbers get to you.
    }
}

// we want to run() and then find the key and a strategy to escape
fn find_key() {
    println!("we get away from them");
    println!("You want to find a key and a strategy to leave");
    println!("what do you want to search first? (1)bathroom, (2)storage room, (3)your bedroom?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let elements = vec!["key", "med kit", "shotgun"];

    // we want to randomly place the key, health potion, and gun in the three rooms
    let mut rng = rand::thread_rng();
    let mut shuffled_elements = elements.clone();
    shuffled_elements.shuffle(&mut rng);

    // Assign the shuffled elements to variables
    let bathroom = shuffled_elements[0].to_string();
    let bedroom = shuffled_elements[1].to_string();
    let storage_room = shuffled_elements[2].to_string();

    // Print the assigned variables
    println!("bathroom: {}", bathroom);
    println!("bedroom: {}", bedroom);
    println!("storage_room: {}", storage_room);

    // we might need to have a list that keeps track of the rooms visited. Need to make sure we do
    // not visit more than two, so a counter and a list for visited rooms


    // we are in bathroom
    if input.trim() == "1" {
        println!("we check the cabinet");
        if bathroom == "key" {
            println!("you got the key");
            // find a way out and call confront(), no need to check any more rooms?
        }
        // else then:
            // we retrieve the item. we could search another room or confront


    // if we are in the bedroom
    } else if input.trim() == "2" {
        println!("we check the safe under the bed");
        // BIG if puzzle is true then :
        if bedroom == "key" {
            println!("you got the key");
            // find a way out and call confront(), no need to check any more rooms?
        }
        // inner else with not key:
            // you pass and the key is not there
        // BIG else the robbers get to you if you run out of time, matched with puzzle if


    // if we are in the storage room
    } else {
        println!("we check the locker in the storage");
        // BIG if puzzle is true then :
        if bedroom == "key" {
            println!("you got the key");
            // find a way out and call confront(), no need to check any more rooms?
        }
        // inner else with not key:
            // you pass and the key is not there
        // BIG else the robbers get to you if you run out of time, matched with puzzle if

    }
}

// implement puzzle 1
// we do listen and silent

// implement puzzle 2
// we do open and p,n,o,e


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
