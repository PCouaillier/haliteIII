use hlt::{
    PlayerId,
    position::Position,
};

pub trait Entity {
    fn owner(&self) -> PlayerId;
    fn position(&self) -> Position;
}
