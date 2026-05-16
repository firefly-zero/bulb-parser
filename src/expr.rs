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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
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
pub enum Token {
    LPar,
    RPar,
    Op(BinOp),
    Val(i32),
    Var(usize),
}

pub fn parse_expr<'a>(input: &'a str, vars: &mut Entities<'a, ()>, row: usize) -> Option<Vec<Op>> {
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

pub fn tokenize<'a>(input: &'a str, vars: &mut Entities<'a, ()>, row: usize) -> Option<Vec<Token>> {
    let mut result = Vec::new();
    let mut input = input.as_bytes();
    while let Some(&c) = input.first() {
        let token = match c {
            b'0'..=b'9' => {
                let (n, shift) = parse_number(input);
                if c == b'0' && n != 0 {
                    return None;
                }
                input = &input[shift..];
                Token::Val(n)
            }
            b'+' => {
                input = &input[1..];
                Token::Op(BinOp::Add)
            }
            b'-' => {
                input = &input[1..];
                Token::Op(BinOp::Sub)
            }
            b'/' => {
                input = &input[1..];
                Token::Op(BinOp::Div)
            }
            b'%' => {
                input = &input[1..];
                Token::Op(BinOp::Mod)
            }
            b'*' => {
                input = &input[1..];
                Token::Op(BinOp::Mul)
            }
            b'<' => {
                input = &input[1..];
                if input.first() == Some(&b'=') {
                    input = &input[1..];
                    Token::Op(BinOp::Lte)
                } else {
                    Token::Op(BinOp::Lt)
                }
            }
            b'>' => {
                input = &input[1..];
                if input.first() == Some(&b'=') {
                    input = &input[1..];
                    Token::Op(BinOp::Gte)
                } else {
                    Token::Op(BinOp::Gt)
                }
            }
            b'=' => {
                input = &input[1..];
                if input.first() == Some(&b'=') {
                    input = &input[1..];
                    Token::Op(BinOp::Eq)
                } else {
                    return None;
                }
            }
            b'!' => {
                input = &input[1..];
                if input.first() == Some(&b'=') {
                    input = &input[1..];
                    Token::Op(BinOp::Ne)
                } else {
                    return None;
                }
            }
            b'(' => {
                input = &input[1..];
                Token::LPar
            }
            b')' => {
                input = &input[1..];
                Token::RPar
            }
            b' ' => {
                input = &input[1..];
                continue;
            }
            c if c.is_ascii_alphabetic() => {
                let shift = parse_id(input);
                let id = &input[..shift];
                let id = unsafe { str::from_utf8_unchecked(id) };
                input = &input[shift..];
                let var = vars.reference(id, row);
                Token::Var(var)
            }
            _ => return None,
        };
        result.push(token);
    }
    Some(result)
}

fn parse_id(input: &[u8]) -> usize {
    let mut shift = 0;
    while let Some(c) = input[shift..].first() {
        if !c.is_ascii_alphanumeric() {
            break;
        }
        shift += 1;
    }
    shift
}

fn parse_number(input: &[u8]) -> (i32, usize) {
    let mut number = 0;
    let mut shift = 0;
    while let Some(c) = input[shift..].first() {
        if !c.is_ascii_digit() {
            break;
        }
        let digit = *c - b'0';
        number = number * 10 + i32::from(digit);
        shift += 1;
    }
    (number, shift)
}
