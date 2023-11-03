use crate::{actor::actor::*, game_constants::MAZE};

#[derive(Clone, Copy)]
pub enum GameOverReason {
    NotDeadYet,
    FellIntoPit,
    WumpusGotYou
}

#[derive(Clone)]
pub struct GameState {
    actors:Vec<Actor>,
    game_over:bool,
    game_over_reason:GameOverReason,
    illegal_move: bool,
}


impl GameState {
    pub fn start_game() -> GameState {
        GameState {
            game_over: false,
            game_over_reason: GameOverReason::NotDeadYet,
            illegal_move: false,
            actors: place_actors()
        }
    }

    pub fn is_game_over(&self) -> bool {
        return self.game_over;
    }

    pub fn is_illegal_move(&self) -> bool {
        return self.illegal_move;
    }

    pub fn get_game_over_reason(&self) -> GameOverReason {
        return self.game_over_reason;
    }

    pub fn get_actor_locations(&self) -> Vec<Actor> {
        return self.actors.clone();
    }

    pub fn get_actor_location(&self, actor_type: ActorType) -> u16 {
        let you = self.actors.iter().find(|x| x.actor_type == actor_type).unwrap();
        return you.room;        
    }

    pub fn get_tunnels(&self, actor_type: ActorType) -> [u16;3] {
        let your_location: u16 = self.get_actor_location(actor_type);
        let room_value: Result<usize, _> = your_location.try_into();
        match room_value {
            Ok(value) => {
                return MAZE[value - 1];

            },
            Err(_err) => {
                return [0,0,0];
            }
        }
    }

    pub fn dangers_nearby(&self) -> Vec<&Actor> {
        let your_location: u16 = self.get_actor_location(ActorType::You);
        let current_room = usize::try_from(your_location).unwrap();
        let tunnels: [u16; 3] = MAZE[current_room-1];
        let dangerous_actors = 
        self.actors.iter().filter(|x|
        {
            let found = tunnels.iter().any(|&y| 
                y == x.room &&
                (x.actor_type == ActorType::Bat ||
                 x.actor_type == ActorType::Pit ||
                 x.actor_type == ActorType::Wumpus));
            return found;     
        });
        return dangerous_actors.collect();
    }

    pub fn move_actor(&self, actor_type : ActorType, new_room: u16) -> GameState {
        // Check if move is legal
        let tunnels = self.get_tunnels(actor_type);
        let move_valid = tunnels.iter().any(|x| *x == new_room);
        let mut gs: GameState = self.clone();
        if move_valid {
            // if do_super_bat_move(self.actors) {
            //     let moved_actor = move_to_random_room();
            // }

            // Move actor
            let moved_actor = Actor {
                actor_type: actor_type,
                room: new_room,
            };
            // Replace moved actor in the actor vector
            let actors_with_moved_actor = self.actors.iter().map(|x| {
                if x.actor_type == actor_type {
                    return moved_actor;
                } else {
                    return *x;
                }}).collect();

            // Check for dangers
            let any_pits = self.is_danger_in_room(&actors_with_moved_actor, ActorType::Pit, new_room);
            if any_pits {
                gs.game_over = true;
                gs.game_over_reason = GameOverReason::FellIntoPit;
            }
            let wumpus = self.is_danger_in_room(&actors_with_moved_actor, ActorType::Wumpus, new_room);
            if wumpus {
                gs.game_over = true;
                gs.game_over_reason = GameOverReason::WumpusGotYou;
            }
            gs.actors = actors_with_moved_actor;
        }
        return gs;
    }

    // fn move_to_random_room(actor: Actor) -> Actor {
    //     let mut rng = thread_rng();
    //     let new_random_room = rng.gen_range(1..game_constants::MAX_ROOMS);
    //     let new_actor = Actor {
    //         actor_type: actor.actor_type,
    //         room: new_random_room,
    //     };
    //     return new_actor;
    // }
    
    fn is_danger_in_room(&self, actors: &Vec<Actor>, actor_type: ActorType, room: u16) -> bool {
    
        if actors.iter().any(|x| x.actor_type == actor_type && x.room == room) {
            return true;
        }
        return false;
    }
    
}