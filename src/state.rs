use crate::*;
use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct State {
    pub sections: Sections,
    /// If true, the game has ended and there is nothing else to do.
    pub end: bool,
    /// The current position of the player.
    pub pos: Pos,
    /// The position of the currently active tile.
    pub tile_pos: Pos,
    pub seed: u32,
    vars: Vec<i32>,
    queue: VecDeque<Action>,
}

#[derive(Debug, Copy, Clone)]
pub struct Pos {
    pub room: usize,
    pub x: u8,
    pub y: u8,
}

impl State {
    pub fn new(sections: Sections) -> Self {
        let start = find_start(&sections);
        let n_vars = sections.n_vars;
        Self {
            sections,
            vars: vec![0; n_vars],
            end: false,
            pos: start,
            seed: 1337,
            tile_pos: start,
            queue: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, id: usize) {
        let actions = &self.sections.actions[id];
        for action in actions {
            self.queue.push_back(action.clone());
        }
    }

    pub fn pop(&mut self) -> Option<Action> {
        self.queue.pop_front()
    }

    pub fn apply(&mut self, action: &Action) {
        match action {
            Action::Say(_) => {}
            Action::End => self.end = true,
            Action::Move(room, x, y) => {
                self.pos = Pos {
                    room: *room,
                    x: *x,
                    y: *y,
                }
            }
            Action::Place(id, pos) => {
                let room = &mut self.sections.rooms[self.pos.room];
                let (x, y) = if let Some(pos) = pos {
                    *pos
                } else {
                    (self.tile_pos.x, self.tile_pos.y)
                };
                let x = usize::from(x);
                let y = usize::from(y);
                room.tiles[y][x] = *id;
            }
            Action::If(cond, id) => {
                let lhs = self.vars[cond.lhs];
                let rhs = cond.rhs;
                let should_branch = match cond.cmp {
                    Cmp::Lt => lhs < rhs,
                    Cmp::Lte => lhs <= rhs,
                    Cmp::Gt => lhs > rhs,
                    Cmp::Gte => lhs >= rhs,
                    Cmp::Eq => lhs == rhs,
                    Cmp::Ne => lhs != rhs,
                };
                if should_branch {
                    self.queue.clear();
                    if let Some(id) = id {
                        self.enqueue(*id);
                    }
                }
            }
            Action::Set(id, val) => self.vars[*id] = *val,
            Action::Add(id, val) => self.vars[*id] += val,
            Action::Select(ids) => {
                self.seed = get_random(self.seed);
                let idx = self.seed as usize % ids.len();
                self.queue.clear();
                self.enqueue(ids[idx]);
            }
            Action::Jump(id) => {
                self.queue.clear();
                self.enqueue(*id);
            }
        }
    }
}

fn find_start(sections: &Sections) -> Pos {
    let tile = find_start_tile(sections);
    for (id, room) in sections.rooms.iter().enumerate() {
        for (y, line) in room.tiles.iter().enumerate() {
            for (x, room_tile) in line.iter().enumerate() {
                if *room_tile == tile {
                    return Pos {
                        room: id,
                        x: x as u8,
                        y: y as u8,
                    };
                }
            }
        }
    }
    Pos {
        room: 0,
        x: 8,
        y: 8,
    }
}

fn find_start_tile(sections: &Sections) -> usize {
    for (i, tile) in sections.tiles.iter().enumerate() {
        if tile.start != 0 {
            return i;
        }
    }
    0
}

/// Use xor-shift algorithm to derive a random number from the given value.
fn get_random(mut x: u32) -> u32 {
    if x == 0 {
        x = 1;
    }
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}
