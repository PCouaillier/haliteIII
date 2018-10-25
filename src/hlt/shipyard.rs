use hlt::{
    command::Command,
    entity::Entity,
    PlayerId,
    position::Position,
};

pub struct Shipyard {
    pub owner: PlayerId,
    pub position: Position,
}

impl Shipyard {
    pub fn spawn(&self) -> Command {
        Command::spawn_ship()
    }
}

impl Entity for Shipyard {
    fn owner(&self) -> PlayerId {
        self.owner
    }

    fn position(&self) -> Position {
        self.position
    }
}
