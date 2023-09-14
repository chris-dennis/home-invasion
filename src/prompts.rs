use std::io;
use std::process::exit;
use std::time::{Duration, Instant};
use rand::seq::SliceRandom;
use rand::thread_rng;
use colored::Colorize;

mod confront;

// struct for player inventory
pub struct Inventory {
    pub key: bool,
    pub weapon: bool,
}

fn prompt() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // accepts any answer starting with "y"
    return input.to_ascii_lowercase().starts_with("y");
}

fn init_inventory() -> Inventory {
 Inventory{
     key: false,
     weapon: false,
 }
}

// this could ask and then ask if
fn options() {
    let inventory = init_inventory();
    println!("You quickly check around the corner, and see 3 figures standing {}", "menacingly".italic().red());
    println!("WHAT SHOULD WE DO? (1) Confront, (2) Run (and search for more resources), or (3) Hide?");
    // you can (1)confront, (2)run, or (3)hide. print this here
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "1" {
        confront::confront(&inventory);
    } else if input.trim() == "2" {
        println!("You have chosen to run. Let's see what we can find...");
        find_key();
    } else {
        println!("You chose to hide, and the robbers storm the room and capture you.");
        println!("Game Over");
        exit(0);
    }
}

fn find_key() {
    let mut inventory = init_inventory();

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
    // println!("storage_room: {}", storage_item);
    // println!("bedroom: {}", bedroom_item);
    // println!("bathroom: {}", bathroom_item);

    // we might need to have a list that keeps track of the rooms visited. Need to make sure we do
    // not visit more than two, so a counter and a list for visited rooms

    // we are in storage room
    if input.trim() == "1" {
        println!("Check the locker in the storage");
        if puzzle1(){
            if storage_item == "key" {
                println!("You got the key");
                println!("Time to confront these guys!");
                inventory.key = true;
                confront::confront(&inventory);
            } else {
                println!("The key is not here, but you found: {}", storage_item);
                if storage_item == "shotgun"{
                    inventory.weapon = true;
                }
                println!("You can check another room before they get to you");
                println!("The remaining rooms are: (2) bedroom, and (3) bathroom");
                input.clear(); // Clear the previous content
                io::stdin().read_line(&mut input).unwrap();
                // if they key is there then you win else you run out of time
                if input.trim() == "2"{
                    println!("Check the safe under the bed");
                    if puzzle2() {
                        if bedroom_item == "key" {
                            println!("You got the key");
                            println!("Time to confront these guys!");
                            inventory.key = true;
                            confront::confront(&inventory);
                        } else {
                            println!("You couldn't find the key and the robbers got to you!");
                            exit(0);
                        }
                    } else {
                        println!("You couldn't open the safe!");
                        println!("the robbers got to you :( Game Over");
                        exit(0);                    }

                } else if input.trim() == "3" {
                    println!("Check the bathroom cabinet");
                    if bathroom_item == "key" {
                        println!("You got the key");
                        println!("Time to confront these guys!");
                        inventory.key = true;
                        confront::confront(&inventory);
                    } else {
                        println!("You couldn't open the safe!");
                        println!("the robbers got to you :( Game Over");
                        exit(0);                    }
                } else {
                    println!("invalid answer, robbers got to you!");
                }
            }
        } else{
            println!("You couldn't open the safe!");
            println!("the robbers got to you :( Game Over");
            exit(0);
        }

    // if we are in the bedroom
    } else if input.trim() == "2" {
        println!("Check the safe under the bed");
        if puzzle2(){
            if bedroom_item == "key" {
                println!("You got the key");
                println!("Time to confront these guys!");
                inventory.key = true;
                confront::confront(&inventory);
            } else {
                println!("The key is not here, but you found: {}", bedroom_item);
                if bedroom_item == "shotgun"{
                    inventory.weapon = true;
                }
                println!("You can check another room before they get to you");
                println!("the remaining rooms are: (1) storage room, and (3) bathroom");
                input.clear(); // Clear the previous content
                io::stdin().read_line(&mut input).unwrap();

                if input.trim() == "1"{
                    println!("Check the locker in the storage");
                    if puzzle1() {
                        if storage_item == "key" {
                            println!("You got the key");
                            println!("Time to confront these guys!");
                            inventory.key = true;
                            confront::confront(&inventory);
                        }
                    } else {
                        println!("You couldn't open the safe!");
                        println!("the robbers got to you :( Game Over");
                        exit(0);
                    }

                } else if input.trim() == "3" {
                    println!("Check the bathroom cabinet");
                    if bathroom_item == "key" {
                        println!("You got the key");
                        println!("time to get out of here and confront these guys");
                        inventory.key = true;
                        confront::confront(&inventory);
                    } else {
                        println!("You couldn't find the key and the robbers got to you!");
                        exit(0);
                    }
                } else {
                    println!("invalid answer, robbers got to you!");
                    exit(0);
                }
            }
        } else {
            println!("You couldn't open the safe!");
            println!("the robbers got to you :( Game Over");
            exit(0);
        }

    // if we are in the bathroom
    } else if input.trim() == "3" {
        println!("Check the bathroom cabinet");
        if bathroom_item == "key" {
            println!("You got the key");
            println!("Time to confront these guys!");
            inventory.key = true;
            confront::confront(&inventory);
        } else {
            println!("The key is not here, but you found: {}", bathroom_item);
            if bathroom_item == "shotgun"{
                inventory.weapon = true;
            }
            println!("We can check another room before they get to you");
            println!("The remaining rooms are: (1) storage room, and (3) bathroom");
            input.clear(); // Clear the previous content
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "1"{
                println!("Check the locker in the storage");
                if puzzle1() {
                    if storage_item == "key" {
                        println!("You got the key");
                        println!("Time to confront these guys!");
                        inventory.key = true;
                        confront::confront(&inventory);
                    }
                } else {
                    println!("You couldn't open the safe!");
                    println!("the robbers got to you :( Game Over");
                    exit(0);
                }

            }  else if input.trim() == "2"{
                println!("Check the safe under the bed");
                if puzzle2() {
                    if bedroom_item == "key" {
                        println!("You got the key");
                        println!("Time to confront these guys!");
                        inventory.key = true;
                        confront::confront(&inventory);
                    }
                } else {
                    println!("You couldn't open the safe!");
                    println!("the robbers got to you :( Game Over");
                    exit(0);
                }
            } else {
                println!("invalid answer, robbers got to you!");
                exit(0);
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

    return input.trim() == "silent" || input.trim() == "tinsel";
}

fn puzzle2() -> bool {
    println!("To open the locker, rearrange the letters to form a word");
    println!("letters to rearrange: p, n, o, e");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    return input.trim() == "open";
}

pub fn intro() {
    println!("{}", "You hear a faint knock at the door. Investigate? (y/n)".purple());
    if prompt() {
        investigate();
    } else {
        println!("Must have been the wind...");
        exit(0);
    }
}

pub fn investigate() {
    println!("You take a look out the window, but see nothing.");
    println!("{}", "Suddenly, you hear the front door being kicked down!".red().on_bright_white());
    println!("You run to the gun safe.");
    println!("You must enter the gun safe combination as shown within 6 seconds! Begin? (y/n)");

    if prompt() {
        println!("{}", "677301".green().on_black());
        if !qte() {
            println!("The robbers got you. Game over");
            exit(0);
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
