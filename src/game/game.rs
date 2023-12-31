use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use crate::game_constants;
use crate::{actor::actor::*, game_constants::*};

#[derive(Clone, Copy)]
pub enum GameAction {
    IlligalGameAction,
    MoveAction,
    ShootAction,
    QuitAction
}

#[derive(Clone, Copy)]
pub enum GameOverReason {
    NotDeadYet,
    YouFellIntoPit,
    YouShotYourself,
    YouShotWumpus,
    YouAreOutOfArrows,
    WumpusGotYou
}

#[derive(Clone)]
pub struct GameState {
    pub actors:Vec<Actor>,
    pub game_over:bool,
    pub game_over_reason:GameOverReason,
    pub illegal_move: bool,
    pub bumped_wumpus: bool,
    pub super_bat_move: bool,
    pub wumpus_moves: bool,
    pub shoot_arrow: bool,
    pub number_of_arrows:u8,
    pub number_of_arrow_rooms: u8,
    pub current_arrow_room_count: u8
}


impl GameState {
    pub fn initialize() -> GameState {
        return GameState {
            game_over: false,
            game_over_reason: GameOverReason::NotDeadYet,
            illegal_move: false,
            bumped_wumpus: false,
            super_bat_move: false,
            wumpus_moves: false,
            actors: vec![],
            shoot_arrow: false,
            number_of_arrows: 5,
            number_of_arrow_rooms: 0,
            current_arrow_room_count:0,
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

    // pub fn get_actor_locations(&self) -> Vec<Actor> {
    //     return self.actors.clone();
    // }



    pub fn get_actor(&self, actor_type: ActorType) -> &Actor {
        return self.actors.iter().find(|x| x.actor_type == actor_type).unwrap();
    }

    pub fn get_tunnels_for_actor(&self, actor_type: ActorType) -> [u8;3] {
        let your_location = self.get_actor(actor_type).room;
        return self.get_tunnels(your_location);
    }

    pub fn get_tunnels(&self, room: u8) -> [u8;3] {
        let room_value: Result<usize, _> = room.try_into();
        match room_value {
            Ok(value) => {
                return MAZE[value - 1];

            },
            Err(_err) => {
                return [0,0,0];
            }
        }
    }

    pub fn move_you(&mut self, mut new_room: u8) {
        // Check if move is legal
        self.bumped_wumpus = false;
        self.super_bat_move = false;
        let tunnels = self.get_tunnels_for_actor(ActorType::You);
        let move_valid = tunnels.iter().any(|x| *x == new_room);
        if move_valid {

            // Check for dangers
            let bats = self.is_actor_in_room(ActorType::Bat, new_room);
            if bats {
                let mut rng = rand::thread_rng();
                new_room = rng.gen_range(1..20);
                self.super_bat_move = true;
            } 

            let pits = self.is_actor_in_room(ActorType::Pit, new_room);
            if pits {
                self.game_over = true;
                self.game_over_reason = GameOverReason::YouFellIntoPit;
            }

            let wumpus = self.is_actor_in_room(ActorType::Wumpus, new_room);
            if wumpus && !self.wumpus_moves {
                self.wumpus_moves = true;
                self.bumped_wumpus = true;
            } else if wumpus && self.wumpus_moves {
                self.game_over = true;
                self.game_over_reason = GameOverReason::WumpusGotYou;
            }

            // All good, move into room
            let you_index = self.actors.iter().position(|x| x.actor_type == ActorType::You).unwrap();
            self.actors[you_index].room = new_room;
            
        }
    }

    pub fn move_wumpus(&mut self) {
        // Determine if wumpus stays or moves (1/4 stay, 3/4 move)
        let mut rng = rand::thread_rng();
        let properbility = rng.gen_range(1..100);
        let do_move = properbility > 25;
        if do_move {
            println!("Wumpus moves");
            // Get tunnels where wumpus can move
            let tunnels = self.get_tunnels_for_actor(ActorType::Wumpus);
            // Select a random tunnel
            let tunnel_index:usize = rng.gen_range(0..2);
            let new_room = tunnels[tunnel_index];

            let you = self.is_actor_in_room(ActorType::You, new_room);

            if you {
                self.game_over_reason = GameOverReason::WumpusGotYou;
                self.game_over = true;
            } else {
                // Did not get You, wumpus moves
                let wumpus_index = self.actors.iter().position(|x| x.actor_type == ActorType::Wumpus).unwrap();
                self.actors[wumpus_index].room = new_room;
            }
        }
    }

    pub fn shoot(&mut self, rooms: Vec<u8>) {
        // Check ammo
        self.number_of_arrows -= 1;
        if self.number_of_arrows == 0 {
            self.game_over = true;
            self.game_over_reason = GameOverReason::YouAreOutOfArrows;
        }
        
        let your_location = self.get_actor(ActorType::You).room;
        let wumpus_location = self.get_actor(ActorType::Wumpus).room;

        let mut arrow_location = self.get_actor(ActorType::You).room;

        for room in rooms {
            if room == your_location {
                    self.game_over = true;
                    self.game_over_reason = GameOverReason::YouShotYourself;
            } else if room == wumpus_location {
                    self.game_over = true;
                    self.game_over_reason = GameOverReason::YouShotWumpus;
            } else {
                let tunnels = self.get_tunnels(arrow_location);
                if tunnels.contains(&room) {
                    arrow_location = room;
                } else {
                    let mut rg = thread_rng();
                    let tunnel_index = rg.gen_range(0..2);
                    arrow_location = tunnels[tunnel_index]; 
                }
                self.move_wumpus();
            }
        }
    }

    pub fn dangers_nearby(&self) -> Vec<&Actor> {
        let your_location = self.get_actor(ActorType::You).room;
        let current_room = usize::try_from(your_location).unwrap();
        let tunnels: [u8; 3] = MAZE[current_room-1];
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

    fn place_actors(&self) -> Vec<Actor> {
        let mut rooms:Vec<u8> = Vec::new();
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

    fn is_actor_in_room(&self, actor_type: ActorType, room: u8) -> bool {
        if self.actors.iter().any(|x| x.actor_type == actor_type && x.room == room) {
            return true;
        }
        return false;
    }
}