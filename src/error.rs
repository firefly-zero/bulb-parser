#[derive(Clone, Debug)]
pub enum StateErr {
    NoStart,
    UnusedStart,
}

impl StateErr {
    pub fn as_str(&self) -> &'static str {
        match self {
            StateErr::NoStart => "START tile not found",
            StateErr::UnusedStart => "START tile is not used in any room",
        }
    }
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrKind {
    UnknownSection,
    NoRooms,
    NoID,
    BadPlayerID,
    NoValue,
    UnknownProperty,
    DuplicateProperty,
    BadHex,
    BadCmp,
    BadVal,
    BadX,
    BadY,
    NotEnoughArgs,
    TooManyArgs,
    NoJump,

    DuplicateRoom,
    DuplicateTile,
    DuplicateImage,
    DuplicateAction,

    UndefinedRoom,
    UndefinedTile,
    UndefinedImage,
    UndefinedAction,

    SmallRoomX,
    SmallRoomY,
    BigRoomX,
    BigRoomY,

    SmallImageX,
    SmallImageY,
    BigImageX,
    BigImageY,
}

impl ErrKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrKind::UnknownSection => "unknown section type",
            ErrKind::NoRooms => "file has no rooms",
            ErrKind::NoID => "ID is missing",
            ErrKind::BadPlayerID => "player ID must be \"idle\"",
            ErrKind::NoValue => "property has no value",
            ErrKind::UnknownProperty => "unknown property",
            ErrKind::DuplicateProperty => "duplicate property",
            ErrKind::BadHex => "invalid hexadecimal digit",
            ErrKind::BadCmp => "unknown comparison operator",
            ErrKind::BadVal => "invalid integer",
            ErrKind::BadX => "invalid X coordinate",
            ErrKind::BadY => "invalid Y coordinate",
            ErrKind::NotEnoughArgs => "not enough arguments",
            ErrKind::TooManyArgs => "too many arguments",
            ErrKind::NoJump => "JUMP or BREAK not found at the expected position",

            ErrKind::DuplicateRoom => "duplicate room ID",
            ErrKind::DuplicateTile => "duplicate tile ID",
            ErrKind::DuplicateImage => "duplicate image ID",
            ErrKind::DuplicateAction => "duplicate action ID",

            ErrKind::UndefinedRoom => "room is referenced but not defined",
            ErrKind::UndefinedTile => "tile is referenced but not defined",
            ErrKind::UndefinedImage => "image is referenced but not defined",
            ErrKind::UndefinedAction => "action is referenced but not defined",

            ErrKind::SmallRoomX => "room row has not enough columns",
            ErrKind::SmallRoomY => "room has not enough rows",
            ErrKind::BigRoomX => "room row has too many columns",
            ErrKind::BigRoomY => "room row has too many rows",

            ErrKind::SmallImageX => "image row has not enough columns",
            ErrKind::SmallImageY => "image has not enough rows",
            ErrKind::BigImageX => "image row has too many columns",
            ErrKind::BigImageY => "image row has too many rows",
        }
    }
}
