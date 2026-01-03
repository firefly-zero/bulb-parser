use crate::entities::*;
use crate::*;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::vec::Vec;

const TILES_X: usize = 15;
const TILES_Y: usize = 10;

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
    pub player: Option<usize>,
    pub start_tile: Option<usize>,
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
            start_tile: None,
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
        let mut room: Vec<[usize; TILES_X]> = Vec::with_capacity(TILES_Y);
        for (row, line) in lines {
            let line = line.trim_ascii();
            if line.is_empty() {
                break;
            }
            let mut tiles = Vec::with_capacity(TILES_X);
            for tile_id in line.split_ascii_whitespace() {
                let tile_id = self.tiles.reference(tile_id, row);
                tiles.push(tile_id);
            }
            if tiles.len() < TILES_X {
                return Err(Err::new(ErrKind::SmallRoomX, row));
            }
            let Ok(tiles) = tiles.try_into() else {
                return Err(Err::new(ErrKind::BigRoomX, row));
            };
            room.push(tiles);
        }
        if room.len() < TILES_Y {
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
        for (row, line) in lines.by_ref() {
            let line = line.trim_ascii();
            if line.is_empty() {
                break;
            }
            if line == "A" {
                let actions = self.parse_actions_inner(lines)?;
                let action_id = self
                    .actions
                    .define_duplicate(id, actions.into_boxed_slice());
                tile.action = Some(action_id);
                break;
            }
            let Some((name, rest)) = line.split_once(' ') else {
                return Err(Err::new(ErrKind::NoValue, row));
            };
            let rest = rest.trim_ascii();
            match name {
                "IMAGE" | "I" => {
                    if tile.image.is_some() {
                        return Err(Err::new(ErrKind::DuplicateProperty, row));
                    }
                    let image_id = self.images.reference(rest, row);
                    tile.image = Some(image_id);
                }
                "WALL" | "W" => {
                    if tile.wall {
                        return Err(Err::new(ErrKind::DuplicateProperty, row));
                    }
                    tile.wall = rest == "1"
                }
                "START" => {
                    if tile.start != 0 {
                        return Err(Err::new(ErrKind::DuplicateProperty, row));
                    }
                    tile.start = match rest {
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        _ => 4,
                    }
                }
                "ACTION" | "A" => {
                    if tile.action.is_some() {
                        return Err(Err::new(ErrKind::DuplicateProperty, row));
                    }
                    let action_id = self.actions.reference(rest, row);
                    tile.action = Some(action_id);
                }
                _ => return Err(Err::new(ErrKind::UnknownProperty, row)),
            };
        }
        let is_start = tile.start != 0;
        let tile_id = self.tiles.define(id, tile);
        if is_start {
            self.start_tile = Some(tile_id);
        }
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
        let mut img = Image::default();
        for (row, line) in lines.by_ref() {
            let line = line.trim_ascii();
            if line.is_empty() {
                break;
            }
            let Some((name, rest)) = line.split_once(' ') else {
                return Err(Err::new(ErrKind::NoValue, row));
            };
            let rest = rest.trim_ascii();
            match name {
                "POS" => {
                    if img.pos != (0, 0) {
                        return Err(Err::new(ErrKind::DuplicateProperty, row));
                    }
                    let (x, y) = split_2_args(rest, row)?;
                    let x = parse_img_coord(x, row)?;
                    let y = parse_img_coord(y, row)?;
                    img.pos = (x, y);
                }
                "FRAMES" => {
                    if img.frames != 0 {
                        return Err(Err::new(ErrKind::DuplicateProperty, row));
                    }
                    let Ok(frames) = rest.parse() else {
                        return Err(Err::new(ErrKind::BadCmp, row));
                    };
                    img.frames = frames
                }
                "PLAYER" => {
                    if img.player.0 != 0 {
                        return Err(Err::new(ErrKind::DuplicateProperty, row));
                    }
                    let (peer, anim) = split_2_args(rest, row)?;
                    let Ok(peer) = peer.parse() else {
                        return Err(Err::new(ErrKind::BadCmp, row));
                    };
                    img.player = (peer, anim.to_owned());
                }
                _ => return Err(Err::new(ErrKind::UnknownProperty, row)),
            };
        }
        self.images.define(id, img);
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
        let actions = self.parse_actions_inner(lines)?;
        self.actions.define(id, actions.into_boxed_slice());
        Ok(())
    }

    fn parse_actions_inner(&mut self, lines: &mut Lines<'a>) -> Result<Vec<Action>, Err> {
        let mut actions = Vec::new();
        for (row, line) in lines {
            let line = line.trim_ascii();
            if line.is_empty() {
                break;
            }
            let (name, rest) = line.split_once(' ').unwrap_or((line, ""));
            let rest = rest.trim_ascii();
            let action = match name {
                "SAY" => {
                    let msg = rest.to_owned();
                    Action::Say(msg)
                }
                "END" => Action::End,
                "MOVE" => {
                    let (room_id, x, y) = split_3_args(rest, row)?;
                    let room_id = self.rooms.reference(room_id, row);
                    let x = parse_x(x, row)?;
                    let y = parse_y(y, row)?;
                    Action::Move(room_id, x, y)
                }
                "PLACE" => {
                    let mut parts = rest.split_ascii_whitespace();
                    let tile_id = get_arg(&mut parts, row)?;
                    let tile_id = self.tiles.reference(tile_id, row);
                    if let Ok(x) = get_arg(&mut parts, row) {
                        let y = get_arg(&mut parts, row)?;
                        let x = parse_x(x, row)?;
                        let y = parse_y(y, row)?;
                        Action::Place(tile_id, Some((x, y)))
                    } else {
                        Action::Place(tile_id, None)
                    }
                }
                "IF" => {
                    let mut parts = rest.split_ascii_whitespace();
                    let lhs = get_arg(&mut parts, row)?;
                    let cmp = get_arg(&mut parts, row)?;
                    let rhs = get_arg(&mut parts, row)?;
                    let sep = get_arg(&mut parts, row)?;
                    let id = match sep {
                        "JUMP" => {
                            let id = get_arg(&mut parts, row)?;
                            Some(self.actions.reference(id, row))
                        }
                        "BREAK" => None,
                        _ => return Err(Err::new(ErrKind::NoJump, row)),
                    };
                    let lhs = self.vars.reference(lhs, row);
                    let cmp = parse_cmp(cmp, row)?;
                    let rhs = parse_val(rhs, row)?;
                    let cond = Cond { lhs, cmp, rhs };
                    Action::If(cond, id)
                }
                "SET" => {
                    let (var_id, val) = split_2_args(rest, row)?;
                    let var_id = self.vars.reference(var_id, row);
                    let val = parse_val(val, row)?;
                    Action::Set(var_id, val)
                }
                "ADD" => {
                    let (var_id, val) = split_2_args(rest, row)?;
                    let var_id = self.vars.reference(var_id, row);
                    let val = parse_val(val, row)?;
                    Action::Add(var_id, val)
                }
                "JUMP" => {
                    let action_id = self.actions.reference(rest, row);
                    Action::Jump(action_id)
                }
                "SELECT" => {
                    let mut subactions = Vec::new();
                    for action in rest.split_ascii_whitespace() {
                        let action = self.actions.reference(action, row);
                        subactions.push(action);
                    }
                    Action::Select(subactions.into_boxed_slice())
                }
                _ => {
                    return Err(Err::new(ErrKind::UnknownProperty, row));
                }
            };
            actions.push(action);
        }
        Ok(actions)
    }

    fn finalize(self) -> Result<Sections, Err> {
        if self.rooms.is_empty() {
            return Err(Err::new(ErrKind::NoRooms, 0));
        }
        let rooms = self.rooms.finalize(ErrKind::UndefinedRoom)?;
        let Some(start_tile) = self.start_tile else {
            return Err(Err::new(ErrKind::NoStart, 0));
        };
        let Some(start_pos) = find_tile_pos(&rooms, start_tile) else {
            return Err(Err::new(ErrKind::UnusedStart, 0));
        };
        let sections = Sections {
            rooms,
            tiles: self.tiles.finalize(ErrKind::UndefinedTile)?,
            images: self.images.finalize(ErrKind::UndefinedImage)?,
            actions: self.actions.finalize(ErrKind::UndefinedAction)?,
            player: self.player,
            n_vars: self.vars.len(),
            start_tile,
            start_pos,
        };
        Ok(sections)
    }
}

fn split_2_args(rest: &str, row: usize) -> Result<(&str, &str), Err> {
    let mut parts = rest.split_ascii_whitespace();
    let x = get_arg(&mut parts, row)?;
    let y = get_arg(&mut parts, row)?;
    if parts.next().is_some() {
        return Err(Err::new(ErrKind::TooManyArgs, row));
    }
    Ok((x, y))
}

fn split_3_args(rest: &str, row: usize) -> Result<(&str, &str, &str), Err> {
    let mut parts = rest.split_ascii_whitespace();
    let id = get_arg(&mut parts, row)?;
    let x = get_arg(&mut parts, row)?;
    let y = get_arg(&mut parts, row)?;
    if parts.next().is_some() {
        return Err(Err::new(ErrKind::TooManyArgs, row));
    }
    Ok((id, x, y))
}

fn get_arg<'a>(
    parts: &mut core::str::SplitAsciiWhitespace<'a>,
    row: usize,
) -> Result<&'a str, Err> {
    let Some(arg) = parts.next() else {
        return Err(Err::new(ErrKind::NotEnoughArgs, row));
    };
    Ok(arg)
}

fn parse_cmp(s: &str, row: usize) -> Result<Cmp, Err> {
    let cmp = match s {
        ">" => Cmp::Gt,
        ">=" => Cmp::Gte,
        "<" => Cmp::Lt,
        "<=" => Cmp::Lte,
        "==" | "=" => Cmp::Eq,
        "!=" | "<>" => Cmp::Ne,
        _ => return Err(Err::new(ErrKind::BadCmp, row)),
    };
    Ok(cmp)
}

fn parse_val(s: &str, row: usize) -> Result<i32, Err> {
    let Ok(val) = s.parse() else {
        return Err(Err::new(ErrKind::BadCmp, row));
    };
    Ok(val)
}

fn parse_img_coord(s: &str, row: usize) -> Result<u16, Err> {
    let Ok(val) = s.parse() else {
        return Err(Err::new(ErrKind::BadCmp, row));
    };
    Ok(val)
}

fn parse_x(s: &str, row: usize) -> Result<u8, Err> {
    let Ok(val) = s.parse() else {
        return Err(Err::new(ErrKind::BadX, row));
    };
    if val >= TILES_X as u8 {
        return Err(Err::new(ErrKind::BadX, row));
    }
    Ok(val)
}

fn parse_y(s: &str, row: usize) -> Result<u8, Err> {
    let Ok(val) = s.parse() else {
        return Err(Err::new(ErrKind::BadY, row));
    };
    if val >= TILES_Y as u8 {
        return Err(Err::new(ErrKind::BadY, row));
    }
    Ok(val)
}

fn find_tile_pos(rooms: &[Room], tile: usize) -> Option<Pos> {
    for (id, room) in rooms.iter().enumerate() {
        for (y, line) in room.tiles.iter().enumerate() {
            for (x, room_tile) in line.iter().enumerate() {
                if *room_tile == tile {
                    return Some(Pos {
                        room: id,
                        x: x as u8,
                        y: y as u8,
                    });
                }
            }
        }
    }
    None
}
