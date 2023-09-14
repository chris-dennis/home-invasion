use std::io;
use std::time::{Duration, Instant};
use rand::seq::SliceRandom;
use rand::thread_rng;
use colored::Colorize;

mod confront;

// struct for player inventory
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

    if input.trim() == "1" {
        println!("you have chosen confront()");
        confront::confront();
    } else if input.trim() == "2" {
        println!("you have chosen run, we find the key and allat");
        find_key();
    } else {
        println!("you chose hide and the robbers got to you!");
    }
}

fn find_key() {
    println!("we get away from them");
    println!("You want to find a key and a strategy to leave");
    println!("what do you want to search first? (1)storage room, (2)bedroom, (3)bathroom");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let elements = vec!["key", "nothing", "shotgun"];
    let mut checked_rooms = vec![false, false, false];

    // we want to randomly place the key, health potion, and gun in the three rooms
    let mut rng = rand::thread_rng();
    let mut shuffled_elements = elements.clone();
    shuffled_elements.shuffle(&mut rng);

    // Assign the shuffled elements to variables
    let bathroom_item = shuffled_elements[0].to_string();
    let bedroom_item = shuffled_elements[1].to_string();
    let storage_item = shuffled_elements[2].to_string();

    // Print the assigned variables
    println!("storage_room: {}", storage_item);
    println!("bedroom: {}", bedroom_item);
    println!("bathroom: {}", bathroom_item);

    // we might need to have a list that keeps track of the rooms visited. Need to make sure we do
    // not visit more than two, so a counter and a list for visited rooms

    // we are in storage room
    if input.trim() == "1" {
        println!("we check the locker in the storage");
        if puzzle1(){
            if storage_item == "key" {
                println!("you got the key");
                println!("time to get out of here and confront these guys");
                confront::confront();
            } else {
                println!("the key is not here, you found: {}", storage_item);
                if storage_item == "shotgun"{
                    const HAS_WEAPON: bool = true;
                }
                println!("You can check another room before they get to you");
                println!("the remaining rooms are: (2)bedroom, and (3)bathroom");
                input.clear(); // Clear the previous content
                io::stdin().read_line(&mut input).unwrap();
                // if they key is there then you win else you run out of time
                if input.trim() == "2"{
                    println!("we check the safe under the bed");
                    if puzzle2() {
                        if bedroom_item == "key" {
                            println!("you got the key");
                            println!("time to get out of here and confront these guys");
                            confront::confront();
                        } else {
                            println!("You couldn't find the key and the robbers got to you!");
                        }
                    } else {
                        println!("You couldn't open the safe and the the robbers got to you :(");
                    }

                } else if input.trim() == "3" {
                    println!("we check the bathroom cabinet");
                    if bathroom_item == "key" {
                        println!("you got the key");
                        println!("time to get out of here and confront these guys");
                        confront::confront();
                    } else {
                        println!("You couldn't find the key and the robbers got to you!");
                    }
                } else {
                    println!("invalid answer, robbers got to you!");
                }
            }
        } else{
            println!("you couldn't open the safe!");
            println!("the robbers got to you :(");
        }

    // if we are in the bedroom
    } else if input.trim() == "2" {
        println!("we check the safe under the bed");
        if puzzle2(){
            if bedroom_item == "key" {
                println!("you got the key");
                println!("time to get out of here and confront these guys");
                confront::confront();
            } else {
                println!("the key is not here, you found: {}", bedroom_item);
                if bedroom_item == "shotgun"{
                    const HAS_WEAPON: bool = true;
                }
                println!("We can check another room before they get to you");
                println!("the remaining rooms are: (1)storage room, and (3)bathroom");
                input.clear(); // Clear the previous content
                io::stdin().read_line(&mut input).unwrap();

                if input.trim() == "1"{
                    println!("we check the locker in the storage");
                    if puzzle1() {
                        if storage_item == "key" {
                            println!("you got the key");
                            println!("time to get out of here and confront these guys");
                            confront::confront();
                        }
                    } else {
                        println!("You couldn't find the key and the robbers got to you!");
                    }

                } else if input.trim() == "3" {
                    println!("we check the bathroom cabinet");
                    if bathroom_item == "key" {
                        println!("you got the key");
                        println!("time to get out of here and confront these guys");
                        confront::confront();
                    } else {
                        println!("You couldn't find the key and the robbers got to you!");
                    }
                } else {
                    println!("invalid answer, robbers got to you!");
                }
            }
        } else {
            println!("you couldn't open the safe!");
            println!("the robbers got to you :(");
        }

    // if we are in the bathroom
    } else if input.trim() == "3" {
        println!("we check the bathroom cabinet");
        if bathroom_item == "key" {
            println!("you got the key");
            println!("time to get out of here and confront these guys");
            confront::confront();
        } else {
            println!("the key is not here, you found: {}", bathroom_item);
            if bathroom_item == "shotgun"{
                const HAS_WEAPON: bool = true;
            }
            println!("We can check another room before they get to you");
            println!("the remaining rooms are: (1)storage room, and (3)bathroom");
            input.clear(); // Clear the previous content
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "1"{
                println!("we check the locker in the storage");
                if puzzle1() {
                    if storage_item == "key" {
                        println!("you got the key");
                        println!("time to get out of here and confront these guys");
                        confront::confront();
                    }
                } else {
                    println!("you couldn't open the safe!");
                    println!("the robbers got to you :(");
                }

            }  else if input.trim() == "2"{
                println!("we check the safe under the bed");
                if puzzle2() {
                    if bedroom_item == "key" {
                        println!("you got the key");
                        println!("time to get out of here and confront these guys");
                        confront::confront();
                    }
                } else {
                    println!("you couldn't open the safe!");
                    println!("the robbers got to you :(");
                }
            } else {
                println!("invalid answer, robbers got to you!");
            }
        }
    } else {
        println!("Invalid choice. Please enter a number between 1 and 3.");
    }
}

fn puzzle1() -> bool {
    println!("To open the safe, solve the following puzzle:");
    println!("rearrange the letters in 'listen' to create another word");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    return input.trim() == "silent";
}

fn puzzle2() -> bool {
    println!("To open the locker, rearrange the letters to form the word");
    println!("letters to rearrange: p, n, o, e");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    return input.trim() == "open";
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
        println!("{}", "677301".green().on_black());
        if !qte() {
            println!("The robbers got you. Game over");
        } else {
            println!("You grab your trusty Mississippi Carbine.");
            options();
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
