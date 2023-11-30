use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use crate::game_constants;
use crate::{actor::actor::*, game_constants::*};

#[derive(Clone, Copy)]
pub enum GameOverReason {
    NotDeadYet,
    FellIntoPit,
    WumpusGotYou
}

#[derive(Clone)]
pub struct GameState {
    pub actors:Vec<Actor>,
    pub game_over:bool,
    pub game_over_reason:GameOverReason,
    pub illegal_move: bool,
    pub wumpus_moves: bool,
}


impl GameState {
    pub fn initialize() -> GameState {
        return GameState {
            game_over: false,
            game_over_reason: GameOverReason::NotDeadYet,
            illegal_move: false,
            wumpus_moves: false,
            actors: vec![],
        };
    }


    pub fn start_game(&mut self)  {
        self.actors = self.place_actors();
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

    pub fn move_you(&self, new_room: u16) -> GameState {
        // Check if move is legal
        let tunnels = self.get_tunnels(ActorType::You);
        let move_valid = tunnels.iter().any(|x| *x == new_room);
        let mut gs: GameState = self.clone();
        if move_valid {
            // if do_super_bat_move(self.actors) {
            //     let moved_actor = move_to_random_room();
            // }

            // Move actor
            let moved_actor = Actor {
                actor_type: ActorType::You,
                room: new_room,
            };
            // Replace moved actor in the actor vector
            let actors_with_moved_actor = self.actors.iter().map(|x| {
                if x.actor_type == ActorType::You {
                    return moved_actor;
                } else {
                    return *x;
                }}).collect();

            // Check for dangers
            let any_pits = self.is_actor_in_room(&actors_with_moved_actor, ActorType::Pit, new_room);
            if any_pits {
                gs.game_over = true;
                gs.game_over_reason = GameOverReason::FellIntoPit;
            }
            let wumpus = self.is_actor_in_room(&actors_with_moved_actor, ActorType::Wumpus, new_room);
            if wumpus {
                gs.wumpus_moves = true;
            } 
            gs.actors = actors_with_moved_actor;
        }
        return gs;
    }

    pub fn move_wumpus(&self) -> GameState {
        let mut gs: GameState = self.clone();
        // Determine if wumpus stays or moves (1/4 stay, 3/4 move)
        let mut rng = rand::thread_rng();
        let properbility = rng.gen_range(1..100);
        let do_move = properbility > 25;
        if do_move {
            // Get tunnels where wumpus can move
            let tunnels = self.get_tunnels(ActorType::Wumpus);
            // Select a random tunnel
            let tunnel_index:usize = rng.gen_range(0..2);
            let new_room = tunnels[tunnel_index];

            let moved_actor = Actor {
                actor_type: ActorType::Wumpus,
                room: new_room,
            };

            // REFACTOR Also used in move_you, Replace moved actor in the actor vector
            let actors_with_moved_actor = self.actors.iter().map(|x| {
                if x.actor_type == ActorType::Wumpus {
                        return moved_actor;
                    } else {
                        return *x;
                }}).collect();

            let you = self.is_actor_in_room(&actors_with_moved_actor, ActorType::You, new_room);

            if you {
                gs.game_over_reason = GameOverReason::WumpusGotYou;
                gs.game_over = true;
            }
            gs.actors = actors_with_moved_actor;     
        }
        return gs;
    }

    fn place_actors(&self) -> Vec<Actor> {
        let mut rooms:Vec<u16> = Vec::new();
        for rn in 1..game_constants::MAX_ROOMS {
            rooms.push(rn);
        }
        let mut rg = thread_rng();
        rooms.shuffle(&mut rg);
    
        let mut actors: Vec<Actor> = Vec::new();
    
        actors.push(Actor {
            actor_type: ActorType::You,
            room: rooms[0],
        });
    
        actors.push(Actor {
            actor_type: ActorType::Pit,
            room: rooms[1],
        });
    
        actors.push(Actor {
            actor_type: ActorType::Pit,
            room: rooms[2],
        });
    
        actors.push(Actor {
            actor_type: ActorType::Bat,
            room: rooms[3],
        });
        actors.push(Actor {
            actor_type: ActorType::Bat,
            room: rooms[4],
        });
    
        actors.push(Actor {
            actor_type: ActorType::Wumpus,
            room: rooms[5],
        });
    
        return actors;
    }    

    fn is_actor_in_room(&self, actors: &Vec<Actor>, actor_type: ActorType, room: u16) -> bool {
        if actors.iter().any(|x| x.actor_type == actor_type && x.room == room) {
            return true;
        }
        return false;
    }
}