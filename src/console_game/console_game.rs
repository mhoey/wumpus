use std::io::stdin;

use crate::game::game::*;
use crate::actor::actor::*;

pub fn start_console_game() {
    println!("Welcome to the game of Wumpus");

    let mut gs = GameState::start_game();

    let mut loop_index = 0;
    while loop_index < 4 {
        println!("Game over {}", gs.is_game_over());
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
        gs = gs.move_actor(ActorType::You, tunnel_number);
        loop_index+=1;
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