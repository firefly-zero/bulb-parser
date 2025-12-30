use alloc::string::String;
use alloc::vec::Vec;

type ID = usize;

pub struct Sections {
    pub rooms: Vec<Room>,
    pub tiles: Vec<Tile>,
    pub images: Vec<Image>,
    pub actions: Vec<Action>,
    pub n_vars: usize,
}

pub struct Room {
    pub tiles: [[ID; 30]; 20],
}
pub struct Tile {
    pub image: Option<ID>,
    pub wall: bool,
    pub player: u8,
    pub action: Option<ID>,
}

pub struct Image {
    pub raw: [u8; 64],
}

pub struct Action {
    pub ado: Do,
    pub next: Option<ID>,
}

pub enum Do {
    /// Print a dialog line.
    Say(String),
    /// Pick up the tile as an item.
    ///
    /// Adds +1 to the value of the given variable
    /// and removes the tile from the room.
    Pick,
    /// End the game.
    End,
    /// Go to another room.
    Exit(ID, u8, u8),
    /// Replace the given tile in the current room.
    Tile(ID, u8, u8),
    /// If the condition is true, execute a different action.
    Branch(Cond, Option<ID>),
    /// Assign the given value to the variable.
    Set(ID, i32),
}

pub struct Cond {
    pub lhs: ID,
    pub op: Operator,
    pub rhs: i32,
}

pub enum Operator {
    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    Ne,
}
