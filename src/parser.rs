use crate::*;
use alloc::{boxed::Box, vec::Vec};

type Lines<'a> = core::iter::Enumerate<core::str::Lines<'a>>;

pub fn parse(raw: &str) -> Result<Sections, Err> {
    let mut parser = Parser::new();
    let mut lines = raw.lines().enumerate();
    loop {
        let Some((i, line)) = lines.next() else {
            break;
        };
        let line = line.trim_ascii();
        let mut chars = line.chars();
        let Some(kind) = chars.next() else {
            continue;
        };
        let id = line[1..].trim_ascii();
        if id.is_empty() {
            return Err(Err::new(ErrKind::NoID, i));
        }
        match kind {
            'R' => parser.parse_room(id, &mut lines)?,
            'T' => parser.parse_tile(id, &mut lines)?,
            'I' => parser.parse_image(id, &mut lines)?,
            'P' => parser.parse_player(id, &mut lines)?,
            'A' => parser.parse_actions(id, &mut lines)?,
            _ => return Err(Err::new(ErrKind::UnknownSection, i)),
        };
    }
    parser.finalize()
}

struct Parser<'a> {
    pub rooms: Vec<(&'a str, Room)>,
    pub tiles: Vec<(&'a str, Tile)>,
    pub images: Vec<(&'a str, Image)>,
    pub actions: Vec<(&'a str, Box<[Action]>)>,
    pub vars: Vec<(&'a str, ())>,
    pub player: Option<Image>,
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
        Ok(())
    }

    fn parse_tile(&mut self, id: &'a str, lines: &mut Lines<'a>) -> Result<(), Err> {
        Ok(())
    }

    fn parse_image(&mut self, id: &'a str, lines: &mut Lines<'a>) -> Result<(), Err> {
        Ok(())
    }

    fn parse_player(&mut self, id: &'a str, lines: &mut Lines<'a>) -> Result<(), Err> {
        Ok(())
    }

    fn parse_actions(&mut self, id: &'a str, lines: &mut Lines<'a>) -> Result<(), Err> {
        Ok(())
    }

    fn finalize(self) -> Result<Sections, Err> {
        if self.rooms.is_empty() {
            return Err(Err::new(ErrKind::NoRooms, 0));
        }
        Ok(Sections {
            rooms: drop_ids(self.rooms),
            tiles: drop_ids(self.tiles),
            images: drop_ids(self.images),
            actions: drop_ids(self.actions),
            player: self.player,
            n_vars: self.vars.len(),
        })
    }
}

fn drop_ids<T>(items: Vec<(&'_ str, T)>) -> Box<[T]> {
    let items: Vec<T> = items.into_iter().map(|(_, v)| v).collect();
    items.into_boxed_slice()
}
