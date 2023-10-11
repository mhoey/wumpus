use crate::{actor::actor::*, game_constants::MAZE};

#[derive(Clone)]
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
            Err(err) => {
                return [0,0,0];
            }
        }
    }

    pub fn move_actor(&self, actor_type : ActorType, new_room: u16) -> GameState {
        // Check if move is legal
        let tunnels = self.get_tunnels(actor_type);
        let move_valid = tunnels.iter().any(|x| *x == new_room);
        let mut gs: GameState = self.clone();
        if move_valid {
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
            gs.actors = actors_with_moved_actor;
        }
        return gs;
    }

} 


pub fn danger_in_room(actor: Actor, other_actors: Vec<Actor>) -> Vec<Actor> {
    let mut danger_actors: Vec<Actor> = Vec::new();

    if actor.actor_type == ActorType::You {
        let actors_in_room = other_actors.iter().filter(|x| x.room == actor.room);

        for actor_in_room in actors_in_room {
            if !danger_actors.iter().any(|x| x.actor_type == actor_in_room.actor_type ) {
                danger_actors.push(*actor_in_room);
            }
        }
    }
    return danger_actors;
}

