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

	let ast = bind(
		range,
		|x| once(Exp::Op(Operation::Plus, Box::new(Exp::Int(x)), Box::new(Exp::Int(x))))
	);
	
	for tree in ast {
		println!("{:?}", tree);
		thread::sleep(dur);
	}
}

// Syntax:
// v ∈ Variable
// i ∈ Integer
// e ∈ Exp | e | i | v = e;
//		   | e + e | e - e | e * e | e / e
//		   | e > e | e < e | e == e | e != e
//		   | if (e) { e;* } | if (e) { e;* } else { e;* }
#[derive(Debug, Eq, PartialEq)]
enum Exp {
	Int(i32),
	Var(String, Box<Exp>),
	Op(Operation, Box<Exp>, Box<Exp>),
	If(Box<Exp>, Vec<Exp>, Vec<Exp>),
}
#[derive(Debug, Eq, PartialEq)]
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
