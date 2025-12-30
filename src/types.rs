use alloc::boxed::Box;
use alloc::string::String;

type ID = usize;
type Array<T> = Box<[T]>;

#[derive(Debug)]
pub struct Sections {
    pub rooms: Array<Room>,
    pub tiles: Array<Tile>,
    pub images: Array<Image>,
    pub actions: Array<Array<Action>>,
    pub player: Option<Image>,
    pub n_vars: usize,
}

#[derive(Debug)]
pub struct Room {
    pub tiles: [[ID; 30]; 20],
}

#[derive(Debug, Default)]
pub struct Tile {
    pub image: Option<ID>,
    pub wall: bool,
    pub player: u8,
    pub action: Option<ID>,
}

#[derive(Debug)]
pub struct Image {
    pub raw: [u8; 64],
}

#[derive(Debug)]
pub enum Action {
    /// Print a dialog line.
    Say(String),
    /// Pick up the tile as an item.
    ///
    /// Adds +1 to the value of the given variable
    /// and removes the tile from the room.
    Pick(ID),
    /// End the game.
    End,
    /// Go to another room.
    Exit(ID, u8, u8),
    /// Replace the given tile in the current room.
    Tile(ID, u8, u8),
    /// If the condition is true, clear the stack and execute a different action.
    Branch(Cond, Option<ID>),
    /// Assign the given value to the variable.
    Set(ID, i32),
    /// Add the given value to the variable.
    ///
    /// Negative values can be used to substract.
    Add(ID, i32),
    /// Push an action at the end of the stack.
    Enqueue(ID),
}

#[derive(Debug)]
pub struct Cond {
    pub lhs: ID,
    pub cmp: Cmp,
    pub rhs: i32,
}

/// Comparison operator.
#[derive(Debug)]
pub enum Cmp {
    /// `<`: Less than.
    Lt,
    /// `<=`: Less than or equal.
    Lte,
    /// `>`: Greater than.
    Gt,
    /// `>=`: Greater than or equal.
    Gte,
    /// `==`: Equal.
    Eq,
    /// `!=`: Not equal.
    Ne,
}
