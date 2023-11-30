use std::io::stdin;

use crate::game::game::*;
use crate::actor::actor::*;

pub fn start_console_game() {
    println!("Welcome to the game of Wumpus");

    let mut gs = GameState::initialize();

    gs.start_game();

    while !gs.is_game_over() {
 
        let actor_locations = gs.get_actor_locations();
        actor_locations.iter().for_each(|x| println!("{} {} in room {}", actor_to_string(&x), match x.actor_type { ActorType::You => "are", _ => "is" } ,x.room));
    
        let tunnels = gs.get_tunnels(ActorType::You);
        let your_location = gs.get_actor_location(ActorType::You);
        let tunnel_list = tunnels.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let tunnel_text = format!("You are in room {}. Tunnels lead to {}", your_location, tunnel_list);
        println!("{}",tunnel_text);
        println!("Which tunnel do You choose:");
        let mut tunnel_input_text = String::new(); 
        stdin().read_line(&mut tunnel_input_text).unwrap();

        let tunnel_number = match tunnel_input_text.trim().parse::<u16>() {
            Ok(tunnel_number) => tunnel_number,
            Err(error) => {
                println!("Illegal input value: {}", error);
                return;
            }
        };
        gs = gs.move_you(tunnel_number);
        if gs.is_illegal_move() {
            println!("Illegal move")
        }

        if gs.wumpus_moves {
            gs = gs.move_wumpus();
        }

        if gs.is_game_over() {
            println!("{}", game_over_reason_to_string(gs.get_game_over_reason()))
        }

        let dangers = dangers_nearby_to_string(gs.dangers_nearby());
        for danger in dangers {
            println!("Danger {}", danger);    
        }
    }
}

fn actor_to_string(actor: &Actor) -> String {
    match actor.actor_type {
        ActorType::You => "You".to_string(),
        ActorType::Wumpus => "Wumpus".to_string(),
        ActorType::Pit => "Pit".to_string(),
        ActorType::Bat => "Bat".to_string(),
    }
}

fn game_over_reason_to_string(reason: GameOverReason) -> String {
    match reason {
        GameOverReason::NotDeadYet => "You still alive".to_string(),
        GameOverReason::FellIntoPit => "You fell into a pit".to_string(),
        GameOverReason::WumpusGotYou => "Wumpus got You!!".to_string()
    }
}

fn dangers_nearby_to_string(actors: Vec<&Actor>) -> Vec<String> {
    let mut danger_list: Vec<String> = Vec::new();
    for actor in actors {
        match actor.actor_type {
            ActorType::Wumpus => danger_list.push("I smell a Wumpus!!".to_string()),
            ActorType::Bat => danger_list.push("Bats nearby".to_string()),
            ActorType::Pit => danger_list.push("I feel a draft".to_string()),
            ActorType::You => {}
        }
    }
    danger_list
}
