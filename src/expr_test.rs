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
    assert!(tokenize("01", &mut vars, 0).is_none());
    assert!(tokenize("?", &mut vars, 0).is_none());
}

#[test]
fn test_parse() {
    fn p(s: &str) -> Vec<Op> {
        let mut vars = Entities::new();
        parse(s, &mut vars, 0).unwrap()
    }
    assert_eq!(p("1"), vec![Op::Val(1)]);
    assert_eq!(p("1+2"), vec![Op::Add, Op::Val(1), Op::Val(2)]);
    // assert_eq!(p("13+14"), vec![Op::Add, Op::Val(13), Op::Val(14)]);
    // assert_eq!(p("13+14"), vec![Op::Add, Op::Val(13), Op::Val(14)]);
    // assert_eq!(p("13-14"), vec![Op::Sub, Op::Val(13), Op::Val(14)]);
}
