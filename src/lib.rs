pub mod monad
{
	pub use std::iter;
	pub use std::iter::{empty, once};

	pub struct Bind<T, U, F>
	where
		T: Iterator,
		U: Iterator,
		F: Fn(T::Item) -> U,
	{
		input: T,
		output: Option<U>,
		func: F,
	}

	impl<T, U, F> Iterator for Bind<T, U, F>
	where
		T: Iterator,
		U: Iterator,
		F: Fn(T::Item) -> U,
	{
		type Item = U::Item;

		fn next(&mut self) -> Option<Self::Item>
		{
			if let Some(output) = self.output.as_mut()
			{
				if let Some(next) = output.next() { Some(next) }
				else if let Some(next) = self.input.next()
				{
					*output = (self.func)(next);
					output.next()
				}
				else { None }
			}
			else { None }
		}
	}

	pub fn bind<T, U, F>(
		mut iterator: T,
		function: F,
		) -> Bind<T, U, F>
	where
		T: Iterator,
		U: Iterator,
		F: Fn(T::Item) -> U,
	{
		if let Some(next) = iterator.next()
		{
			Bind {
				input : iterator,
				output: Some((function)(next)),
				func: function,
			}
		}
		else
		{
			Bind {
				input: iterator,
				output: None,
				func: function,
			}
		}
	}
}

#[cfg(test)]
pub mod tests
{
	use crate::monad::{once, empty, bind};

	#[test]
	fn create_bind()
	{
		let mut value = bind(
			once(4),
			|x| once(x * 2),
			);
		assert_eq!(value.next(), Some(8));
		assert_eq!(value.next(), None);
	}

	#[test]
	fn bind_to_none()
	{
		let mut value = bind(
			1..10,
			|_| empty::<i32>(),
			);
		assert_eq!(value.next(), None);
	}

	#[test]
	fn bind_to_new_type()
	{
		let mut value = bind(
			once(4),
			|x| once(format!("Number is {}", x)),
			);
		assert_eq!(value.next(), Some(String::from("Number is 4")));
		assert_eq!(value.next(), None);
	}

	#[test]
	fn bind_from_multiple()
	{
		let mut value = bind(
			once(4).chain(once(8)).chain(once(1)),
			|x| once(x * 2),
			);
		assert_eq!(value.next(), Some(8));
		assert_eq!(value.next(), Some(16));
		assert_eq!(value.next(), Some(2));
		assert_eq!(value.next(), None);
	}

	#[test]
	fn bind_from_infinite()
	{
		let mut value = bind(
			1..,
			|x| once(-x),
			);
		assert_eq!(value.next(), Some(-1));
		assert_eq!(value.next(), Some(-2));
		assert_eq!(value.next(), Some(-3));
		assert_eq!(value.next(), Some(-4));
		assert_ne!(value.next(), None);
		assert_ne!(value.next(), None);
		assert_eq!(value.next(), Some(-7));
	}

	#[test]
	fn bind_to_multiple()
	{
		let mut value = bind(
			once(4),
			|x| once(x).chain(once(x * 2)),
			);
		assert_eq!(value.next(), Some(4));
		assert_eq!(value.next(), Some(8));
		assert_eq!(value.next(), None);
	}

	#[test]
	fn chained_binds()
	{
		let mut value =
			bind(once(5), |x| once(x * 3))
			.chain(bind(once(7), |x| once(-x)));

		assert_eq!(value.next(), Some(15));
		assert_eq!(value.next(), Some(-7));
		assert_eq!(value.next(), None);
	}

	#[test]
	fn empty_input()
	{
		let mut value = bind(
			empty::<i32>(),
			|x| once(x),
			);
		assert_eq!(value.next(), None);
	}
}
