#[derive(Clone, Debug)]
pub struct Err {
    pub kind: ErrKind,
    pub row: usize,
}

impl Err {
    pub fn new(kind: ErrKind, row: usize) -> Self {
        Self { kind, row }
    }
}

#[derive(Clone, Debug)]
pub enum ErrKind {
    UnknownSection,
    NoRooms,
    NoID,

    DuplicateRoom,
    DuplicateTile,
    DuplicateImage,
    DuplicateAction,

    UndefinedRoom,
    UndefinedTile,
    UndefinedImage,
    UndefinedAction,
}

impl ErrKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrKind::UnknownSection => "unknown section type",
            ErrKind::NoRooms => "file has no rooms",
            ErrKind::NoID => "ID is missing",

            ErrKind::DuplicateRoom => "duplicate room ID",
            ErrKind::DuplicateTile => "duplicate tile ID",
            ErrKind::DuplicateImage => "duplicate image ID",
            ErrKind::DuplicateAction => "duplicate action ID",

            ErrKind::UndefinedRoom => "room is referenced but not defined",
            ErrKind::UndefinedTile => "tile is referenced but not defined",
            ErrKind::UndefinedImage => "image is referenced but not defined",
            ErrKind::UndefinedAction => "action is referenced but not defined",
        }
    }
}
