use crate::{entities::Entities, expr::*};

#[test]
fn text_tokenize() {
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
    assert_eq!(t("3>4"), vec![Val(3), Op(Gt), Val(4)]);
    assert_eq!(t("3>=4"), vec![Val(3), Op(Gte), Val(4)]);
    assert_eq!(t("3==4"), vec![Val(3), Op(Eq), Val(4)]);
    assert_eq!(t("  3 +  4 "), vec![Val(3), Op(Add), Val(4)]);
    assert_eq!(t("3-4"), vec![Val(3), Op(Sub), Val(4)]);
    assert_eq!(t("(3)"), vec![Paren('('), Val(3), Paren(')')]);
    assert_eq!(
        t("(3-4)"),
        vec![Paren('('), Val(3), Op(Sub), Val(4), Paren(')')]
    );
}
