use crate::*;
use alloc::{boxed::Box, vec::Vec};

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
    pub rooms: Vec<Entity<'a, Room>>,
    pub tiles: Vec<Entity<'a, Tile>>,
    pub images: Vec<Entity<'a, Image>>,
    pub actions: Vec<Entity<'a, Box<[Action]>>>,
    pub vars: Vec<Entity<'a, ()>>,
    pub player: Option<Image>,
}

struct Entity<'a, T> {
    /// The human-readable entity ID as defined in the file.
    id: &'a str,
    /// Row number where the entity is first referenced.
    first_ref: usize,
    /// Entity value. If None, the entity was referenced but definition not found yet.
    value: Option<T>,
}

impl<'a> Parser<'a> {
    fn new() -> Self {
        Self {
            rooms: Vec::new(),
            tiles: Vec::new(),
            images: Vec::new(),
            actions: Vec::new(),
            vars: Vec::new(),
            player: None,
        }
    }

    fn parse_room(&mut self, id: &'a str, lines: &mut Lines<'a>) -> Result<(), Err> {
        for entity in &self.rooms {
            if entity.value.is_some() && entity.id == id {
                let row = get_row(lines);
                return Err(Err::new(ErrKind::DuplicateRoom, row));
            }
        }
        // ...
        Ok(())
    }

    fn parse_tile(&mut self, id: &'a str, lines: &mut Lines<'a>) -> Result<(), Err> {
        for entity in &self.tiles {
            if entity.value.is_some() && entity.id == id {
                let row = get_row(lines);
                return Err(Err::new(ErrKind::DuplicateTile, row));
            }
        }
        // ...
        Ok(())
    }

    fn parse_image(&mut self, id: &'a str, lines: &mut Lines<'a>) -> Result<(), Err> {
        for entity in &self.images {
            if entity.value.is_some() && entity.id == id {
                let row = get_row(lines);
                return Err(Err::new(ErrKind::DuplicateImage, row));
            }
        }
        Ok(())
    }

    fn parse_player(&mut self, id: &'a str, lines: &mut Lines<'a>) -> Result<(), Err> {
        Ok(())
    }

    fn parse_actions(&mut self, id: &'a str, lines: &mut Lines<'a>) -> Result<(), Err> {
        for entity in &self.actions {
            if entity.value.is_some() && entity.id == id {
                let row = get_row(lines);
                return Err(Err::new(ErrKind::DuplicateAction, row));
            }
        }
        Ok(())
    }

    fn finalize(self) -> Result<Sections, Err> {
        if self.rooms.is_empty() {
            return Err(Err::new(ErrKind::NoRooms, 0));
        }
        Ok(Sections {
            rooms: drop_ids(self.rooms, ErrKind::UndefinedRoom)?,
            tiles: drop_ids(self.tiles, ErrKind::UndefinedTile)?,
            images: drop_ids(self.images, ErrKind::UndefinedImage)?,
            actions: drop_ids(self.actions, ErrKind::UndefinedAction)?,
            player: self.player,
            n_vars: self.vars.len(),
        })
    }
}

fn drop_ids<T>(items: Vec<Entity<T>>, kind: ErrKind) -> Result<Box<[T]>, Err> {
    let mut result: Vec<T> = Vec::new();
    for entity in items {
        let Some(val) = entity.value else {
            return Err(Err::new(kind, entity.first_ref));
        };
        result.push(val);
    }
    Ok(result.into_boxed_slice())
}

fn get_row(lines: &mut Lines<'_>) -> usize {
    match lines.next() {
        Some((i, _)) => i - 1,
        None => 0,
    }
}
