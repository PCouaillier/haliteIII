use hlt::{
    dropoff::Dropoff,
    DropoffId,
    game::GameState,
    input::Input,
    PlayerId,
    position::Position,
    ship::Ship,
    ShipId,
    shipyard::Shipyard,
};

use std::collections::HashMap;

pub struct Player {
    pub id: PlayerId,
    pub shipyard: Shipyard,
    pub halite: usize,
    pub ship_ids: Vec<ShipId>,
    pub dropoff_ids: Vec<DropoffId>,
}

impl Player {
    pub fn update(
        &mut self,
        input: &mut Input,
        ships: &mut HashMap<ShipId, Ship>,
        dropoffs: &mut HashMap<DropoffId, Dropoff>,
        game_state: GameState)
    {
        let max_halite = game_state.max_halite;
        let num_dropoffs = game_state.num_dropoffs;
        let num_ships = game_state.num_ships;

        self.halite = game_state.halite;

        self.ship_ids.clear();
        for _ in 0..num_ships {
            let ship = Ship::generate(input, self.id, max_halite);
            self.ship_ids.push(ship.id);
            ships.insert(ship.id, ship);
        }

        self.dropoff_ids.clear();
        for _ in 0..num_dropoffs {
            let dropoff = Dropoff::generate(input, self.id);
            self.dropoff_ids.push(dropoff.id);
            dropoffs.insert(dropoff.id, dropoff);
        }
    }

    pub fn generate(input: &mut Input) -> Player {
        input.read_and_parse_line();
        let id = PlayerId(input.next_usize());
        let shipyard_x = input.next_i32();
        let shipyard_y = input.next_i32();

        let shipyard = Shipyard { owner: id, position: Position { x: shipyard_x, y: shipyard_y } };

        Player { id, shipyard, halite: 0, ship_ids: Vec::new(), dropoff_ids: Vec::new() }
    }
}
