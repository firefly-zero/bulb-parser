use crate::entities::Entities;
use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Copy, Clone)]
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

pub enum Node {
    Var(usize),
    Val(i32),
    BinOp(Box<Node>, BinOp, Box<Node>),
}

impl Node {
    pub fn op(lhs: Node, op: BinOp, rhs: Node) -> Self {
        Self::BinOp(Box::new(lhs), op, Box::new(rhs))
    }
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

type R<T> = Result<T, &'static str>;

pub fn parse<'a>(input: &'a str, vars: &mut Entities<'a, ()>, row: usize) -> R<Box<[Op]>> {
    let tokens = tokenize(input, vars, row)?;
    let (root_node, consumed) = parse_cmp(&tokens, 0)?;
    if tokens.len() != consumed {
        return Err("expression can only be parsed partially");
    }
    Ok(flatten(root_node).into_boxed_slice())
}

/// Converts AST into a flat list of opcodes in postfix (aka reverse Polish) notation.
pub fn flatten(node: Node) -> Vec<Op> {
    let mut result = Vec::new();
    flatten_into(node, &mut result);
    result
}

pub fn flatten_into(node: Node, result: &mut Vec<Op>) {
    match node {
        Node::Var(var) => result.push(Op::Var(var)),
        Node::Val(x) => result.push(Op::Val(x)),
        Node::BinOp(lhs, op, rhs) => {
            flatten_into(*lhs, result);
            flatten_into(*rhs, result);
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
        }
    }
}

fn parse_cmp(tokens: &[Token], pos: usize) -> R<(Node, usize)> {
    let (subnode, next_pos) = parse_mul(tokens, pos)?;
    let c = tokens.get(next_pos);
    let Some(c) = c else {
        return Ok((subnode, next_pos));
    };
    match *c {
        Token::Op(
            op @ (BinOp::Lt | BinOp::Lte | BinOp::Gt | BinOp::Gte | BinOp::Eq | BinOp::Ne),
        ) => {
            let (rhs, i) = parse_cmp(tokens, next_pos + 1)?;
            let node = Node::op(subnode, op, rhs);
            Ok((node, i))
        }
        _ => Ok((subnode, next_pos)),
    }
}

fn parse_mul(tokens: &[Token], pos: usize) -> R<(Node, usize)> {
    let (subnode, next_pos) = parse_summand(tokens, pos)?;
    let c = tokens.get(next_pos);
    let Some(c) = c else {
        return Ok((subnode, next_pos));
    };
    match *c {
        Token::Op(op @ (BinOp::Add | BinOp::Sub)) => {
            let (rhs, i) = parse_mul(tokens, next_pos + 1)?;
            let node = Node::op(subnode, op, rhs);
            Ok((node, i))
        }
        _ => Ok((subnode, next_pos)),
    }
}

fn parse_summand(tokens: &[Token], pos: usize) -> R<(Node, usize)> {
    let (subnode, next_pos) = parse_term(tokens, pos)?;
    let c = tokens.get(next_pos);
    let Some(c) = c else {
        return Ok((subnode, next_pos));
    };
    match *c {
        Token::Op(op @ (BinOp::Mul | BinOp::Div | BinOp::Mod)) => {
            let (rhs, i) = parse_summand(tokens, next_pos + 1)?;
            let node = Node::op(subnode, op, rhs);
            Ok((node, i))
        }
        _ => Ok((subnode, next_pos)),
    }
}

fn parse_term(tokens: &[Token], pos: usize) -> R<(Node, usize)> {
    let Some(c) = tokens.get(pos) else {
        return Err("unexpected end of input, expected a term");
    };
    match *c {
        Token::Val(n) => Ok((Node::Val(n), pos + 1)),
        Token::Var(name) => Ok((Node::Var(name), pos + 1)),
        Token::LPar => {
            let (node, next_pos) = parse_mul(tokens, pos + 1)?;
            if let Some(Token::RPar) = tokens.get(next_pos) {
                Ok((node, next_pos + 1))
            } else {
                Err("matching closing parenthesis not found")
            }
        }
        _ => Err("unexpected token, expected a term"),
    }
}

/// Convert expression source code string into a list of tokens.
///
/// Variables are resolved into references.
/// Invalid input may define some references.
pub fn tokenize<'a>(input: &'a str, vars: &mut Entities<'a, ()>, row: usize) -> R<Vec<Token>> {
    let mut result = Vec::new();
    let mut input = input.as_bytes();
    while let Some(&c) = input.first() {
        let token = match c {
            b'0'..=b'9' => {
                let (n, shift) = parse_number(input);
                if c == b'0' && n != 0 {
                    return Err("an integer cannot start with zero");
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
                    return Err("equality must have two equal signs");
                }
            }
            b'!' => {
                input = &input[1..];
                if input.first() == Some(&b'=') {
                    input = &input[1..];
                    Token::Op(BinOp::Ne)
                } else {
                    return Err("invalid token '!'; did you mean '!='?");
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
            _ => return Err("invalid token"),
        };
        result.push(token);
    }
    Ok(result)
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
