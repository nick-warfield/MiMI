use std::{thread, time};
use monad;
use monad::monad::{once, empty, bind};

fn main()
{
	let dur = time::Duration::new(0, 150_000_000);
	let mut factors = bind(
		1..,
		|x| once(format!("\nNumber: {}", x))
			.chain(factor_iter(x)
				   .map(|num| format!("\tFactor: {}", num)))
		);
	
	loop
	{
		println!("{}", factors.next().unwrap());
		thread::sleep(dur);
	}
}

struct FactorIter
{
	num: u64,
	factor: u64,
}

impl Iterator for FactorIter
{
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item>
	{
		let mut value = self.factor;
		while value > 0 && self.num % value != 0 { value -= 1; }

		if value == 0 { None }
		else { self.factor = value - 1; Some(value) }
	}
}

fn factor_iter(num: u64) -> FactorIter
{
	FactorIter{
		num,
		factor: num,
	}
}
