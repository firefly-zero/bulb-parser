use crate::{entities::Entities, expr::*};

#[test]
fn test_tokenize() {
    use BinOp::*;
    use Token::*;
    fn t(s: &str) -> Vec<Token> {
        let mut vars = Entities::new();
        tokenize(s, &mut vars, 0).unwrap()
    }
    assert_eq!(t("1"), vec![Val(1)]);
    assert_eq!(t("13"), vec![Val(13)]);
    assert_eq!(t("6247"), vec![Val(6247)]);

    assert_eq!(t("3+4"), vec![Val(3), Op(Add), Val(4)]);
    assert_eq!(t("13+14"), vec![Val(13), Op(Add), Val(14)]);
    assert_eq!(t("3>4"), vec![Val(3), Op(Gt), Val(4)]);
    assert_eq!(t("3>=4"), vec![Val(3), Op(Gte), Val(4)]);
    assert_eq!(t("3==4"), vec![Val(3), Op(Eq), Val(4)]);
    assert_eq!(t("  3 +  4 "), vec![Val(3), Op(Add), Val(4)]);
    assert_eq!(t("3-4"), vec![Val(3), Op(Sub), Val(4)]);

    assert_eq!(t("(3)"), vec![LPar, Val(3), RPar]);
    assert_eq!(t("(3-4)"), vec![LPar, Val(3), Op(Sub), Val(4), RPar]);

    assert_eq!(t("x"), vec![Var(0)]);
    assert_eq!(t("xyz"), vec![Var(0)]);
    assert_eq!(t("x13"), vec![Var(0)]);
    assert_eq!(t("x+x"), vec![Var(0), Op(Add), Var(0)]);
    assert_eq!(t("x+y"), vec![Var(0), Op(Add), Var(1)]);

    let mut vars = Entities::new();
    assert!(tokenize("01", &mut vars, 0).is_err());
    assert!(tokenize("?", &mut vars, 0).is_err());
}

#[test]
fn test_flatten() {
    use Op::*;
    let ops = flatten(Node::Val(3));
    assert_eq!(ops, vec![Val(3)]);
    let ops = flatten(Node::op(Node::Val(3), BinOp::Add, Node::Val(4)));
    assert_eq!(ops, vec![Val(3), Val(4), Add]);
    let mul = Node::op(Node::Val(2), BinOp::Mul, Node::Val(7));
    let ops = flatten(Node::op(Node::Val(3), BinOp::Add, mul));
    assert_eq!(ops, vec![Val(3), Val(2), Val(7), Mul, Add]);
}

#[test]
fn test_parse() {
    fn p(s: &str) -> Vec<Op> {
        let mut vars = Entities::new();
        parse(s, &mut vars, 0).unwrap().into_vec()
    }
    use Op::*;
    assert_eq!(p("1"), vec![Val(1)]);
    assert_eq!(p("1+2"), vec![Val(1), Val(2), Add]);
    assert_eq!(p("13+14"), vec![Val(13), Val(14), Add]);
    assert_eq!(p("13-14"), vec![Val(13), Val(14), Sub]);
    assert_eq!(p("2*7"), vec![Val(2), Val(7), Mul]);
    assert_eq!(p("2*7+3"), vec![Val(2), Val(7), Mul, Val(3), Add]);
    assert_eq!(p("3+2*7"), vec![Val(3), Val(2), Val(7), Mul, Add]);
    assert_eq!(p("4-5-6"), vec![Val(4), Val(5), Val(6), Sub, Sub]);
    assert_eq!(p("3>7"), vec![Val(3), Val(7), Gt]);
    assert_eq!(
        p("3+2>7+1"),
        vec![Val(3), Val(2), Add, Val(7), Val(1), Add, Gt]
    );
}
