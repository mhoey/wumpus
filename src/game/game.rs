use crate::{actor::actor::*, game_constants::MAZE};

#[derive(Clone)]
pub struct GameState {
    actors:Vec<Actor>,
    game_over:bool,
}


impl GameState {
    pub fn start_game() -> GameState {
        GameState {
            game_over: false,
            actors: place_actors()
        }
    }

    pub fn is_game_over(&self) -> bool {
        return self.game_over;
    }

    pub fn get_actor_locations(&self) -> Vec<Actor> {
        return self.actors.clone();
    }

    pub fn get_your_location(&self) -> u16 {
        let you = self.actors.iter().find(|x| x.actor_type == ActorType::You).unwrap();
        return you.room;        
    }

    pub fn get_tunnels(&self) -> [u16;3] {
        let your_location: u16 = self.get_your_location();
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

    pub fn move_actors(&self) -> GameState {
        let new_actors = place_actors();
        let mut gs: GameState = self.clone();
        gs.game_over = self.game_over;
        gs.actors = new_actors;
        return gs;
    } 
}

