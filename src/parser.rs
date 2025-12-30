use crate::entities::*;
use crate::*;
use alloc::boxed::Box;
use alloc::vec::Vec;

type Lines<'a> = core::iter::Enumerate<core::str::Lines<'a>>;

pub fn parse(raw: &str) -> Result<Sections, Err> {
    let mut parser = Parser::new();
    let mut lines = raw.lines().enumerate();
    loop {
        let Some((row, line)) = lines.next() else {
            break;
        };
        let line = line.trim_ascii();
        let mut chars = line.chars();
        let Some(kind) = chars.next() else {
            continue;
        };
        let id = line[1..].trim_ascii();
        if id.is_empty() {
            return Err(Err::new(ErrKind::NoID, row));
        }
        match kind {
            'R' => parser.parse_room(id, &mut lines)?,
            'T' => parser.parse_tile(id, &mut lines)?,
            'I' => parser.parse_image(id, &mut lines)?,
            'P' => parser.parse_player(id, &mut lines)?,
            'A' => parser.parse_actions(id, &mut lines)?,
            _ => return Err(Err::new(ErrKind::UnknownSection, row)),
        };
    }
    parser.finalize()
}

struct Parser<'a> {
    pub rooms: Entities<'a, Room>,
    pub tiles: Entities<'a, Tile>,
    pub images: Entities<'a, Image>,
    pub actions: Entities<'a, Box<[Action]>>,
    pub vars: Entities<'a, ()>,
    pub player: Option<Image>,
}

impl<'a> Parser<'a> {
    fn new() -> Self {
        Self {
            rooms: Entities::new(),
            tiles: Entities::new(),
            images: Entities::new(),
            actions: Entities::new(),
            vars: Entities::new(),
            player: None,
        }
    }

    fn parse_room(&mut self, id: &'a str, lines: &mut Lines<'a>) -> Result<(), Err> {
        if self.rooms.is_defined(id) {
            let row = get_row(lines);
            return Err(Err::new(ErrKind::DuplicateRoom, row));
        }
        let mut room: Vec<[usize; 30]> = Vec::new();
        for (row, line) in lines.by_ref().take(20) {
            let line = line.trim_ascii();
            if line.is_empty() {
                return Err(Err::new(ErrKind::SmallRoomY, row));
            }
            let mut tiles = Vec::new();
            for tile_id in line.split_ascii_whitespace() {
                let tile_id = self.tiles.reference(tile_id, row);
                tiles.push(tile_id);
            }
            if tiles.len() < 30 {
                return Err(Err::new(ErrKind::SmallRoomX, row));
            }
            let Ok(tiles) = tiles.try_into() else {
                return Err(Err::new(ErrKind::BigRoomX, row));
            };
            room.push(tiles);
        }
        let room = Room {
            tiles: room.try_into().unwrap(),
        };
        self.rooms.define(id, room);
        Ok(())
    }

    fn parse_tile(&mut self, id: &'a str, lines: &mut Lines<'a>) -> Result<(), Err> {
        if self.tiles.is_defined(id) {
            let row = get_row(lines);
            return Err(Err::new(ErrKind::DuplicateTile, row));
        }
        // ...
        Ok(())
    }

    fn parse_image(&mut self, id: &'a str, lines: &mut Lines<'a>) -> Result<(), Err> {
        if self.images.is_defined(id) {
            let row = get_row(lines);
            return Err(Err::new(ErrKind::DuplicateImage, row));
        }
        Ok(())
    }

    fn parse_player(&mut self, id: &'a str, lines: &mut Lines<'a>) -> Result<(), Err> {
        Ok(())
    }

    fn parse_actions(&mut self, id: &'a str, lines: &mut Lines<'a>) -> Result<(), Err> {
        if self.actions.is_defined(id) {
            let row = get_row(lines);
            return Err(Err::new(ErrKind::DuplicateAction, row));
        }
        Ok(())
    }

    fn finalize(self) -> Result<Sections, Err> {
        if self.rooms.is_empty() {
            return Err(Err::new(ErrKind::NoRooms, 0));
        }
        Ok(Sections {
            rooms: self.rooms.finalize(ErrKind::UndefinedRoom)?,
            tiles: self.tiles.finalize(ErrKind::UndefinedTile)?,
            images: self.images.finalize(ErrKind::UndefinedImage)?,
            actions: self.actions.finalize(ErrKind::UndefinedAction)?,
            player: self.player,
            n_vars: self.vars.len(),
        })
    }
}

fn get_row(lines: &mut Lines<'_>) -> usize {
    match lines.next() {
        Some((i, _)) => i - 1,
        None => 0,
    }
}
