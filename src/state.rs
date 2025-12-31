use crate::*;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct State {
    pub sections: Sections,
    pub vars: Vec<i32>,
    pub end: bool,
    pub pos: Pos,
    pub queue: VecDeque<Action>,
}

#[derive(Debug, Copy, Clone)]
pub struct Pos {
    pub room: usize,
    pub x: u8,
    pub y: u8,
}

impl State {
    pub fn enqueue(&mut self, id: usize) {
        let actions = &self.sections.actions[id];
        for action in actions {
            self.queue.push_back(action.clone());
        }
    }

    pub fn apply(&mut self, action: &Action) {
        match action {
            Action::Say(_) => {}
            Action::Pick(new_id) => {
                let room = &mut self.sections.rooms[self.pos.room];
                let x = usize::from(self.pos.x);
                let y = usize::from(self.pos.y);
                let old_id = room.tiles[y][x];
                self.vars[old_id] += 1;
                room.tiles[y][x] = *new_id;
            }
            Action::End => self.end = true,
            Action::Exit(room, x, y) => {
                self.pos = Pos {
                    room: *room,
                    x: *x,
                    y: *y,
                }
            }
            Action::Place(id, x, y) => {
                let room = &mut self.sections.rooms[self.pos.room];
                let x = usize::from(*x);
                let y = usize::from(*y);
                room.tiles[y][x] = *id;
            }
            Action::Branch(_, _) => todo!(),
            Action::Set(id, val) => self.vars[*id] = *val,
            Action::Add(id, val) => self.vars[*id] += val,
            Action::Enqueue(id) => self.enqueue(*id),
        }
    }
}
