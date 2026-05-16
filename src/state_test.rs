use crate::*;

#[test]
fn test_eval_expr() {
    let sections = Sections {
        rooms: Box::new([]),
        tiles: Box::new([Tile {
            image: None,
            wall: false,
            start: 0,
            action: None,
        }]),
        images: Box::new([]),
        actions: Box::new([]),
        player: None,
        n_vars: 4,
        start_tile: 0,
        start_pos: Pos {
            room: 0,
            x: 5,
            y: 5,
        },
    };
    let state = State::new(sections);

    use Op::*;
    assert_eq!(state.eval_expr(&[Val(4)]).unwrap(), 4);
    assert_eq!(state.eval_expr(&[Val(4), Val(2), Add]).unwrap(), 6);
    assert_eq!(state.eval_expr(&[Val(7), Val(3), Sub]).unwrap(), 4);
    assert_eq!(state.eval_expr(&[Val(7), Val(3), Gt]).unwrap(), 1);
    assert_eq!(state.eval_expr(&[Val(3), Val(7), Gt]).unwrap(), 0);
    assert_eq!(state.eval_expr(&[Val(7), Val(3), Lt]).unwrap(), 0);
}
