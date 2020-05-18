#![feature(box_patterns)]
extern crate rand;

// bind, once, empty, chain
use monad::monad::*;
use std::{thread, time};
use rand::distributions::{Uniform, Distribution};

fn main() {
	let dur = time::Duration::new(0, 150_000_000);
	let mut rng = rand::thread_rng();
	let range = Uniform::new(-100, 100);
	let range = range.sample_iter(&mut rng);
	let mut int = bind(range, |x| once(Exp::Var(String::from("x"), Box::new(Exp::Int(x)))));

	let ast = ASTGenIter { depth: 2, tree: int.next() };
	
	for tree in ast {
		println!("{:?}", tree);
		thread::sleep(dur);
	}
}

#[derive(Debug)]
struct ASTGenIter {
	depth: i32,
	tree: Option<Exp>,
}
impl Iterator for ASTGenIter {
	type Item = Exp;
	fn next(&mut self) -> Option<Self::Item> {
		if self.depth == 0 { return None; }
		self.depth -= 1;
		let next = self.tree.clone();
		self.tree = if let Some(tree) = &self.tree {
			match &tree {
				Exp::Int(_) => None,
				Exp::Var(_, box e) => Some(e.clone()),
				_ => None,
			}
		} else { None };
		next
	}
}

// Syntax:
// v ∈ Variable
// i ∈ Integer
// e ∈ Exp | e | i | v = e;
//		   | e + e | e - e | e * e | e / e
//		   | e > e | e < e | e == e | e != e
//		   | if (e) { e;* } | if (e) { e;* } else { e;* }
#[derive(Clone, Debug, Eq, PartialEq)]
enum Exp {
	Int(i32),
	Var(String, Box<Exp>),
	Op(Operation, Box<Exp>, Box<Exp>),
	If(Box<Exp>, Vec<Exp>, Vec<Exp>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Operation {
	Plus,
	Minus,
	Multiply,
	Divide,

	LessThan,
	GreatThan,
	Equals,
	NotEquals,
}

#[cfg(test)]
mod tests {
	#[test]
	fn dummy_test() {
		assert!(true)
	}
}
