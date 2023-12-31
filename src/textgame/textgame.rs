use std::io::stdin;
use crate::game::game::*;
use crate::actor::actor::*;
use crate::ioadapter::ioadapter::IoAdapter;

pub fn start_text_game<T: IoAdapter>(io: &T) {
    io.write("WUMPUS 1");
    io.write("COPYRIGHT 1979  CREATIVE COMPUTING   MORRISTOWN, NJ");

    let mut gs = GameState::initialize();

    gs.start_game();

    while !gs.is_game_over() {
 
        //let actor_locations = gs.get_actor_locations();
        //actor_locations.iter().for_each(|x| io.write("{} {} in room {}", actor_to_string(&x), match x.actor_type { ActorType::You => "are", _ => "is" } ,x.room));

        let tunnels = gs.get_tunnels_for_actor(ActorType::You);
        let your_location = gs.get_actor(ActorType::You).room;
        let tunnel_list = tunnels.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
        io.write(&format!("YOU ARE IN ROOM {}", your_location)); 
        io.write(&format!("TUNNELS LEAD TO {}", tunnel_list));
        let dangers = dangers_nearby_to_string(gs.dangers_nearby());
        for danger in dangers {
            io.write(&format!("{}", danger));    
        }
        

        io.write("SHOOT OR MOVE (S-M)?");

        let mut move_or_shoot_text = String::new();
        stdin().read_line(&mut move_or_shoot_text).unwrap();

        let game_action: GameAction;

        match move_or_shoot_text.as_str().trim() {
            "M" | "m" => game_action = GameAction::MoveAction,
            "S" | "s" => game_action = GameAction::ShootAction,
            "Q" | "q" => game_action = GameAction::QuitAction,
            _ => game_action = GameAction::IlligalGameAction,
        }

        match game_action {
            GameAction::MoveAction => move_you(io, &mut gs),
            GameAction::ShootAction => shoot(io, &mut gs),
            GameAction::QuitAction => { gs.game_over = true; },
            GameAction::IlligalGameAction => io.write("I do not understand"),
        };

        if gs.is_game_over() {
            io.write(&format!("{}", game_over_reason_to_string(gs.get_game_over_reason())))
        }
    }
}

fn move_you<T: IoAdapter>(io: &T, gs: &mut GameState) {
    io.write("WHERE TO:");
    let tunnel_number= io.read_number();

    gs.move_you(tunnel_number);

    if gs.bumped_wumpus {
        io.write("...OOPS! BUMPED A WUMPUS!")
    }
    if gs.super_bat_move {
        io.write("ZAP--SUPER BAT SNATCH! ELSEWHEREVILLE FOR YOU!")
    }

    if gs.wumpus_moves {
        gs.move_wumpus();
    }
}

fn shoot<T: IoAdapter>(io: &T, gs: &mut GameState) {
    io.write("NO. OF ROOMS(1-5)");
    let number_of_rooms = io.read_number();

    let mut arrow_rooms = Vec::<u8>::new();
    if 1 <= number_of_rooms && number_of_rooms <= 5 {
        let mut rooms_collected = 0;
        while rooms_collected < number_of_rooms {

            io.write("ROOM #");

            let room_number = io.read_number();

            arrow_rooms.push(room_number);
            rooms_collected += 1;
        }

        gs.shoot(arrow_rooms);

    }
}

// fn actor_to_string(actor: &Actor) -> String {
//     match actor.actor_type {
//         ActorType::You => "You".to_string(),
//         ActorType::Wumpus => "Wumpus".to_string(),
//         ActorType::Pit => "Pit".to_string(),
//         ActorType::Bat => "Bat".to_string(),
//     }
// }

fn game_over_reason_to_string(reason: GameOverReason) -> String {
    match reason {
        GameOverReason::NotDeadYet => "You still alive".to_string(),
        GameOverReason::YouFellIntoPit => "YYYIIIIEEEE . . . FELL IN PIT".to_string(),
        GameOverReason::WumpusGotYou => "TSK TSK TSK- WUMPUS GOT YOU!".to_string(),
        GameOverReason::YouAreOutOfArrows => "You are out of arrows".to_string(),
        GameOverReason::YouShotYourself => "You have shot Youself".to_string(),
        GameOverReason::YouShotWumpus => "AHA! YOU GOT THE WUMPUS!".to_string()
    }
}

fn dangers_nearby_to_string(actors: Vec<&Actor>) -> Vec<String> {
    let mut danger_list: Vec<String> = Vec::new();
    for actor in actors {
        match actor.actor_type {
            ActorType::Wumpus => danger_list.push("I SMELL A WUMPUS!".to_string()),
            ActorType::Bat => danger_list.push("BATS NEARBY!".to_string()),
            ActorType::Pit => danger_list.push("I FEEL A DRAFT".to_string()),
            ActorType::You => {}
        }
    }
    danger_list
}