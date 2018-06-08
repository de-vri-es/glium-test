use nalgebra as na;

/// Trait for things that can be adjust other things.
pub trait Adjust<T>: Sized {
	type Output;

	fn adjust(&self, original: T) -> Self::Output;

	fn chain<A2: Adjust<Self::Output>>(self, a2: A2) -> Chain<Self, A2> {
		Chain(self, a2)
	}
}

/// Implement Adjustable<FnMut(T)> for T generically.
impl<F: Fn(T), T> Adjust<T> for F {
	type Output = F::Output;

	fn adjust(&self, original: T) -> Self::Output {
		self(original)
	}
}

/// Chain two adjustments.
/**
 * The adjusted output is equivalent to:
 *   a.adjust(b.adjust(t))
 *
 * That is: A is the left/outer adjustment,
 * and B is the right/inner adjustment.
 *
 * In terms of transformations the result is: A * B * x
 */
pub struct Chain<A, B>(pub A, pub B);

/// Implement Adjust<T> for Chain<A, B>.
impl<A, B, T> Adjust<T> for Chain<A, B>
where
	B: Adjust<T>,
	A: Adjust<B::Output>
{
	type Output = A::Output;

	fn adjust(&self, original: T) -> A::Output {
		let Chain(a, b) = self;
		a.adjust(b.adjust(original))
	}
}

/// Identity adjustment that works on all types.
/**
 * Mainly useful for cases where an adjustment is expected,
 * but you don't actually want to perform an adjustment.
 */
#[derive(Clone, Copy, Debug)]
pub struct Id;

/// Identity adjustment.
impl<T> Adjust<T> for Id {
	type Output = T;

	fn adjust(&self, original: T) -> T { original }
}

#[derive(Clone, Debug)]
pub struct Adjusted<O, A> {
	pub data:   O,
	pub adjust: A,
}

impl<O: Copy, A: Copy> Copy for Adjusted<O, A> {}

impl<O, A>  Adjusted<O, A> {
	pub fn chain_right<A2>(self, a2: A2) -> Adjusted<O, Chain<A, A2>> {
		Adjusted{data: self.data, adjust: Chain(self.adjust, a2)}
	}
	pub fn chain_left<A1>(self, a1: A1) -> Adjusted<O, Chain<A1, A>> {
		Adjusted{data: self.data, adjust: Chain(a1, self.adjust)}
	}
}

pub fn adjust<O, A>(data: O, adjust: A) -> Adjusted<O, A> {
	Adjusted{data, adjust}
}

// Common adjustments.

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct SetTime(pub f32);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SetLightDirection(pub na::Unit<na::Vector3<f32>>);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LeftTransformGeometry(pub na::Transform3<f32>);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RightTransformGeometry(pub na::Transform3<f32>);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SetMaterial<M>(pub M);

macro_rules! impl_adjust {
	($A:ty, $T:ty, $O:ty, $self:ident, $value:ident, $body:block) => {
		impl<'a> Adjust<&'a $T> for $A {
			type Output = $O;
			fn adjust(&$self, $value: &$T) -> $O { $body }
		}

		impl Adjust<$T> for $A {
			type Output = $O;
			fn adjust(&self, n: $T) -> $O {
				self.adjust(&n)
			}
		}
	};
}
