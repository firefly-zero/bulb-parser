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
    pub player: Option<ID>,
    pub n_vars: usize,
    pub start_tile: usize,
    pub start_pos: Pos,
}

#[derive(Debug)]
pub struct Room {
    pub tiles: [[ID; 15]; 10],
}

#[derive(Debug, Default)]
pub struct Tile {
    pub image: Option<usize>,
    pub wall: bool,
    pub start: u8,
    pub action: Option<ID>,
}

#[derive(Debug, Clone, Default)]
pub struct Image {
    pub pos: (u16, u16),
    pub frames: u8,
    pub player: (u8, String),
}

#[derive(Debug, Clone)]
pub enum Action {
    /// Print a dialog line.
    Say(String),
    /// End the game.
    End,
    /// Go to another room.
    Move(ID, u8, u8),
    /// Replace the tile at the given position in the current room.
    Place(ID, Option<(u8, u8)>),
    /// If the condition is true, clear the stack and execute a different action.
    If(Cond, Option<ID>),
    /// Assign the given value to the variable.
    Set(ID, i32),
    /// Add the given value to the variable.
    ///
    /// Negative values can be used to substract.
    Add(ID, i32),
    /// Clear the stack and execute the given action set instead.
    Jump(ID),
    Select(Box<[ID]>),
}

#[derive(Debug, Clone)]
pub struct Cond {
    pub lhs: ID,
    pub cmp: Cmp,
    pub rhs: i32,
}

/// Comparison operator.
#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub struct Pos {
    pub room: usize,
    pub x: u8,
    pub y: u8,
}
