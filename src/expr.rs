use crate::entities::Entities;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::iter::Peekable;

pub enum Op {
    Var(usize),
    /// `0`: Integer value.
    Val(i32),
    /// `+`: Addition.
    Add,
    /// `-`: Substraction.
    Sub,
    /// `/`: Division.
    Div,
    /// `%`: Modulus.
    Mod,
    /// `*`: Multiplication.
    Mul,

    /// `<`: Less than.
    Lt,
    /// `<=`: Less than or equal.
    Lte,
    /// `>`: Greater than.
    Gt,
    /// `>=`: Greater than or equal.
    Gte,
    /// `==`: Equal.
    Eq,
    /// `!=`: Not equal.
    Ne,
}

enum Node {
    Var(usize),
    Val(i32),
    BinOp(Box<Node>, BinOp, Box<Node>),
}

enum BinOp {
    Add,
    Sub,
    Div,
    Mod,
    Mul,
    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    Ne,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Paren(char),
    Op(char),
    Val(i32),
    Var(usize),
}

pub fn parse_expr(input: &str, vars: &mut Entities<'_, ()>, row: usize) -> Option<Vec<Op>> {
    let tokens = tokenize(input, vars, row)?;
    let (root_node, rest) = parse_node(&tokens)?;
    if !rest.is_empty() {
        return None;
    }
    Some(flatten(root_node))
}

fn flatten(root_node: Node) -> Vec<Op> {
    let mut result = Vec::new();
    let mut queue = VecDeque::<Node>::new();
    queue.push_back(root_node);
    while let Some(node) = queue.pop_front() {
        match node {
            Node::Var(_) => todo!(),
            Node::Val(x) => result.push(Op::Val(x)),
            Node::BinOp(lhs, op, rhs) => {
                let op = match op {
                    BinOp::Add => Op::Add,
                    BinOp::Sub => Op::Sub,
                    BinOp::Div => Op::Div,
                    BinOp::Mod => Op::Mod,
                    BinOp::Mul => Op::Mul,
                    BinOp::Lt => Op::Lt,
                    BinOp::Lte => Op::Lte,
                    BinOp::Gt => Op::Gt,
                    BinOp::Gte => Op::Gte,
                    BinOp::Eq => Op::Eq,
                    BinOp::Ne => Op::Ne,
                };
                result.push(op);
                queue.push_back(*lhs);
                queue.push_back(*rhs);
            }
        }
    }
    result
}

fn parse_node(tokens: &[Token]) -> Option<(Node, &str)> {
    None
}

pub(crate) fn tokenize(input: &str, vars: &mut Entities<'_, ()>, row: usize) -> Option<Vec<Token>> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        let token = match c {
            '0'..='9' => {
                let n = parse_number(&mut it);
                Token::Val(n)
            }
            '+' | '-' | '*' | '/' => {
                it.next();
                Token::Op(c)
            }
            '(' | ')' => {
                it.next();
                Token::Paren(c)
            }
            ' ' => {
                it.next();
                continue;
            }
            c if c.is_ascii_alphabetic() => {
                // let id = parse_id(&mut it);
                // let var = vars.reference(&id, row);
                // Token::Var(var)
                Token::Var(0)
            }
            _ => return None,
        };
        result.push(token);
    }
    Some(result)
}

fn parse_number<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> i32 {
    let mut number = 0;
    while let Some(Some(digit)) = iter.peek().map(parse_digit) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}

fn parse_id<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> String {
    todo!()
}

fn parse_digit(c: &char) -> Option<i32> {
    if c.is_ascii_digit() {
        let n = *c as u8 - b'0';
        return Some(n.into());
    }
    None
}
