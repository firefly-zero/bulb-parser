use crate::entities::Entities;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;

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

enum Node<'a> {
    Var(&'a str),
    Val(i32),
    BinOp(Box<Node<'a>>, BinOp, Box<Node<'a>>),
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

pub fn parse_expr(vars: Entities<'_, ()>, raw: &str) -> Option<Vec<Op>> {
    let root_node = parse_node(raw)?;
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

fn parse_node(raw: &str) -> Option<Node<'_>> {
    if let Ok(val) = raw.parse::<i32>() {
        return Some(Node::Val(val));
    };
    None
}
