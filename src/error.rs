#[derive(Clone, Debug)]
pub struct Err {
    pub kind: ErrKind,
    pub line: usize,
}

impl Err {
    pub fn new(kind: ErrKind, line: usize) -> Self {
        Self { kind, line }
    }
}

#[derive(Clone, Debug)]
pub enum ErrKind {
    UnknownSection,
    NoRooms,
    NoID,
}
