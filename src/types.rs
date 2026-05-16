use alloc::boxed::Box;
use alloc::string::String;

use crate::Expr;

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

/// Sub-image of the atlas (sprite in the sprite sheet).
#[derive(Debug, Clone, Default)]
pub struct Image {
    // Upper-left corner of the first animation frame.
    pub pos: ImagePos,
    /// The number of animation frames.
    /// Animation frames are always placed horizontally.
    pub frames: u8,
    pub player: Option<(u8, String)>,
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
    If(Expr, Option<ID>),
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

#[derive(Debug, Copy, Clone)]
pub struct Pos {
    pub room: usize,
    pub x: u8,
    pub y: u8,
}

/// Position of an image in the atlas.
#[derive(Debug, Default, Copy, Clone)]
pub struct ImagePos {
    pub x: u8,
    pub y: u8,
}
