use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use crate::game_constants;


#[derive(Copy, Clone, PartialEq)]
pub enum ActorType {
    You,
    Wumpus,
    Bat,
    Pit,
}

#[derive(Copy, Clone)]
pub struct Actor {
    pub actor_type: ActorType,
    pub room: u16,
}


pub fn place_actors() -> Vec<Actor> {
    let mut rooms: Vec<u16> = Vec::new();
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

pub fn where_to(actor: Actor) -> [u16;3] {
    if actor.actor_type == ActorType::You {
        let room = usize::try_from(actor.room).unwrap();
        let tunnels = game_constants::MAZE[room-1];
        return tunnels;
    }
    return [0,0,0]
}

pub fn move_to(actor: Actor, room: u16) -> (Actor, bool) {
    let current_room = usize::try_from(actor.room).unwrap();
    let tunnels = game_constants::MAZE[current_room-1];
    let move_valid = tunnels.iter().any(|x| *x == room);
    if move_valid {
        let new_actor = Actor {
            actor_type: actor.actor_type,
            room: room,
        };
        return (new_actor, true);
    } else {
        return (actor, false)
    }
}

pub fn move_to_random_room(actor: Actor) -> Actor {
    let mut rng = thread_rng();
    let new_random_room = rng.gen_range(1..game_constants::MAX_ROOMS);
    let new_actor = Actor {
        actor_type: actor.actor_type,
        room: new_random_room,
    };
    return new_actor;
}




pub fn do_superbat_move(danger_actors: Vec<Actor>) -> bool {

    if danger_actors.iter().any(|x| x.actor_type == ActorType::Bat) {
        return true;
    }
    return false;
}

pub fn danger_to_string(actors: Vec<Actor>) -> Vec<String> {
    let mut danger_list: Vec<String> = Vec::new();
    for actor in actors {
        match actor.actor_type {
            ActorType::Wumpus => danger_list.push("Wumpus GOT YOU!!".to_string()),
            ActorType::Bat => danger_list.push("Superbat move".to_string()),
            ActorType::Pit => danger_list.push("YAAAAAAAAAAAHHHH You fell in a pit".to_string()),
            ActorType::You => {}
        }
    }
    return danger_list;
}


pub fn dangers_nearby(actor: Actor, other_actors: Vec<Actor>) -> Vec<Actor> {
    let mut danger_actors: Vec<Actor> = Vec::new();

    if actor.actor_type == ActorType::You {
        let room = usize::try_from(actor.room).unwrap();
        let tunnels = game_constants::MAZE[room-1];

        for tunnel in tunnels {
            let actors_in_room = other_actors.iter().filter(|x| x.room == tunnel);

            for actor_in_room in actors_in_room {
                if !danger_actors.iter().any(|x| x.actor_type == actor_in_room.actor_type ) {
                    danger_actors.push(*actor_in_room);
                }
            }
        }
    }
    return danger_actors;
}

pub fn dangers_nearby_to_string(actors: Vec<Actor>) -> Vec<String> {
    let mut danger_list: Vec<String> = Vec::new();
    for actor in actors {
        match actor.actor_type {
            ActorType::Wumpus => danger_list.push("I smell a Wumpus!!".to_string()),
            ActorType::Bat => danger_list.push("Bats nearby".to_string()),
            ActorType::Pit => danger_list.push("I feel a draft".to_string()),
            ActorType::You => {}
        }
    }
    return danger_list;
}
