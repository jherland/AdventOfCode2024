use std::io;
use std::iter::{repeat_n, zip};

use itertools::Itertools;

#[derive(Clone, Debug)]
enum Op {
    Add,
    Mul,
    Cat,
}

impl Op {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
            Self::Cat => (a.to_string() + &b.to_string()).parse::<u64>().unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
struct Equation {
    result: u64,
    inputs: Vec<u64>,
    ops: Option<Vec<Op>>,
}

impl Equation {
    fn parse(line: String) -> Self {
        let (lhs, rest) = line.split_once(": ").unwrap();
        Self {
            result: lhs.parse::<u64>().unwrap(),
            inputs: rest.split(" ").map(|s| s.parse::<u64>().unwrap()).collect(),
            ops: None
        }
    }

    fn ops_permutations(&self, available_ops: Vec<Op>) -> impl Iterator<Item = Self> + use<'_> {
        let num_ops = self.inputs.len() - 1;
        repeat_n(available_ops, num_ops)
            .multi_cartesian_product()
            .map(|ops| Self { ops: Some(ops), ..self.clone() })
    }

    fn is_correct(&self) -> bool {
        let ops = self.ops.clone().unwrap().to_vec();
        assert!(ops.len() + 1 == self.inputs.len());
        let mut operands = self.inputs.iter();
        let mut result: u64 = *operands.next().unwrap();
        for (op, operand) in zip(ops.iter(), operands) {
            result = op.apply(result, *operand);
            if result > self.result {
                break;
            }
        }
        result == self.result
    }
}

fn solve(eqs: &Vec<Equation>, ops: Vec<Op>) -> u64 {
    eqs
        .iter()
        .filter(|partial| partial
            .ops_permutations(ops.to_vec())
            .any(|eq| eq.is_correct())
        )
        .map(|eq| eq.result)
        .sum()
}

fn main() {
    let eqs: Vec<Equation> = io::stdin().lines().flatten().map(Equation::parse).collect();
    println!("Part 1: {}", solve(&eqs, vec![Op::Add, Op::Mul]));
    println!("Part 2: {}", solve(&eqs, vec![Op::Add, Op::Mul, Op::Cat]));
}
