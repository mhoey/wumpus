

// mod actor;
// mod game_constants;
// use dialoguer::Input;
// use crate::actor::actor::*;
mod game_constants;
mod game;
mod actor;
mod console_game;
use crate::console_game::console_game::start_console_game;

fn main() {
    start_console_game();

    // let actors = place_actors();
    // actors.iter().for_each(|x| println!("{} {} in room {}", actor_to_string(&x), match x.actor_type { ActorType::You => "are", _ => "is" } ,x.room));

    // // Get you
    // let you = actors.iter().find(|x| x.actor_type == ActorType::You).unwrap();

    // let mut dead = false;
    // while !dead {
    //     let tunnels = where_to(*you);
    //     let tunnel_list = tunnels.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
    //     let where_to = format!("You are in room {}. Tunnels lead to {}", you.room, tunnel_list);
    //     println!("{}", where_to);

    //     let answer : String = Input::new().with_prompt("Where to").interact_text().unwrap();

    //     println!("Ok lets go to {}", answer);

    //     dead = true;
    // }
    // let mut current_room = you.room;
    // let mut new_you = *you;

    // for index in 1..10 {
        
    //     let room_index = usize::try_from(current_room-1).unwrap();
    //     let next_tunnel = rand::thread_rng().gen_range(0..3);
    //     let next_room = game_constants::MAZE[room_index][next_tunnel];

    //     let dangers = danger_in_room(new_you, actors.to_vec());
    //     let danger_strings = danger_to_string(dangers.to_vec());

    //     for danger_string in danger_strings {
    //         println!("{}", danger_string);
    //     }

    //     let dangers_nearby = dangers_nearby(new_you, actors.to_vec());
    //     let danger_nearby_strings = dangers_nearby_to_string(dangers_nearby);

    //     for danger_string in danger_nearby_strings {
    //         println!("{}", danger_string);
    //     }

    //     if do_superbat_move(dangers.to_vec()) {
    //         let n = move_to_random_room(new_you);
    //         new_you = n;
    //         current_room = new_you.room;    
    //         println!("Bat picked You up and left You at room {}", current_room);
    //         println!("Move {} {}",index, where_to(new_you));
    //     } else {
    //         let (n, valid_move) = move_to(new_you, next_room);
    //         new_you = n;
    //         current_room = new_you.room;    

    //         if valid_move {
    //             println!("Move {} {}",index, where_to(new_you));
    //         } else {
    //             println!("Illigal move");
    //         }
    //     }
    //     let tunnel_list = tunnels.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
    //     return format!("You are in room {}. Tunnels lead to {}", room, tunnel_list);

    println!();
}
