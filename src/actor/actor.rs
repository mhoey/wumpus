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
