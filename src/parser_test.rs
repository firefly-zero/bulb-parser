use crate::*;

#[test]
fn test_empty_file() {
    let res = parse("");
    let kind = res.unwrap_err().kind;
    assert_eq!(kind, ErrKind::NoRooms)
}

#[test]
fn test_room_without_id() {
    let res = parse("R");
    let kind = res.unwrap_err().kind;
    assert_eq!(kind, ErrKind::NoID)
}

#[test]
fn test_tile_without_id() {
    let res = parse("T");
    let kind = res.unwrap_err().kind;
    assert_eq!(kind, ErrKind::NoID)
}

#[test]
fn test_image_without_id() {
    let res = parse("I ");
    let kind = res.unwrap_err().kind;
    assert_eq!(kind, ErrKind::NoID)
}

#[test]
fn test_action_without_id() {
    let res = parse("A    ");
    let kind = res.unwrap_err().kind;
    assert_eq!(kind, ErrKind::NoID)
}

#[test]
fn test_room_without_rows() {
    let res = parse("R 1");
    let kind = res.unwrap_err().kind;
    assert_eq!(kind, ErrKind::SmallRoomY)
}
