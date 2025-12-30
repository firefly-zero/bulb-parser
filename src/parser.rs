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
            break; // End of file.
        };
        let line = line.trim_ascii();
        let mut chars = line.chars();
        let Some(kind) = chars.next() else {
            continue; // Empty line.
        };
        let id = line[1..].trim_ascii();
        if id.is_empty() {
            return Err(Err::new(ErrKind::NoID, row));
        }
        match kind {
            'R' => parser.parse_room(id, &mut lines, row)?,
            'T' => parser.parse_tile(id, &mut lines, row)?,
            'I' => parser.parse_image(id, &mut lines, row)?,
            'P' => parser.parse_player(id, &mut lines, row)?,
            'A' => parser.parse_actions(id, &mut lines, row)?,
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

    fn parse_room(
        &mut self,
        id: &'a str,
        lines: &mut Lines<'a>,
        first_row: usize,
    ) -> Result<(), Err> {
        if self.rooms.is_defined(id) {
            return Err(Err::new(ErrKind::DuplicateRoom, first_row));
        }
        let mut room: Vec<[usize; 30]> = Vec::new();
        for (row, line) in lines {
            let line = line.trim_ascii();
            if line.is_empty() {
                break;
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
        if room.len() < 20 {
            return Err(Err::new(ErrKind::SmallRoomY, first_row));
        }
        let Ok(room) = room.try_into() else {
            return Err(Err::new(ErrKind::BigRoomY, first_row));
        };
        let room = Room { tiles: room };
        self.rooms.define(id, room);
        Ok(())
    }

    fn parse_tile(
        &mut self,
        id: &'a str,
        lines: &mut Lines<'a>,
        first_row: usize,
    ) -> Result<(), Err> {
        if self.tiles.is_defined(id) {
            return Err(Err::new(ErrKind::DuplicateTile, first_row));
        }
        let mut tile = Tile::default();
        for (row, line) in lines {
            let line = line.trim_ascii();
            if line.is_empty() {
                break;
            }
            let Some((name, rest)) = line.split_once(' ') else {
                return Err(Err::new(ErrKind::NoValue, row));
            };
            let rest = rest.trim_ascii();
            match name {
                "IMAGE" => {
                    if tile.image.is_some() {
                        return Err(Err::new(ErrKind::DuplicateProperty, row));
                    }
                    let image_id = self.images.reference(rest, row);
                    tile.image = Some(image_id);
                }
                "WALL" => {
                    if tile.wall {
                        return Err(Err::new(ErrKind::DuplicateProperty, row));
                    }
                    tile.wall = rest == "1"
                }
                "PLAYER" => {
                    if tile.player != 0 {
                        return Err(Err::new(ErrKind::DuplicateProperty, row));
                    }
                    tile.player = match rest {
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        _ => 4,
                    }
                }
                "ACTION" => {
                    if tile.action.is_some() {
                        return Err(Err::new(ErrKind::DuplicateProperty, row));
                    }
                    let action_id = self.actions.reference(rest, row);
                    tile.action = Some(action_id);
                }
                _ => return Err(Err::new(ErrKind::UnknownProperty, row)),
            };
        }
        self.tiles.define(id, tile);
        Ok(())
    }

    fn parse_image(
        &mut self,
        id: &'a str,
        lines: &mut Lines<'a>,
        first_row: usize,
    ) -> Result<(), Err> {
        if self.images.is_defined(id) {
            return Err(Err::new(ErrKind::DuplicateImage, first_row));
        }
        Ok(())
    }

    fn parse_player(
        &mut self,
        id: &'a str,
        lines: &mut Lines<'a>,
        first_row: usize,
    ) -> Result<(), Err> {
        Ok(())
    }

    fn parse_actions(
        &mut self,
        id: &'a str,
        lines: &mut Lines<'a>,
        first_row: usize,
    ) -> Result<(), Err> {
        if self.actions.is_defined(id) {
            return Err(Err::new(ErrKind::DuplicateAction, first_row));
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
