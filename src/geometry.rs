use nalgebra::{
	Affine3,
	Matrix4,
	Real,
	Rotation3,
	Translation3,
	Unit,
	Vector3,
};

use alga::linear::NormedSpace;

pub trait VectorEx: Sized {
	fn as_unit(self) -> Unit<Self>;
}

impl<T: NormedSpace> VectorEx for T {
	fn as_unit(self) -> Unit<T> {
		Unit::new_normalize(self)
	}
}

pub fn rotate<T: Real>(axis: Unit<Vector3<T>>, angle: T) -> Rotation3<T> {
	Rotation3::from_axis_angle(&axis, angle)
}

pub fn rotate_x<T: Real>(angle: T) -> Rotation3<T> { rotate(Vector3::x_axis(), angle) }
pub fn rotate_y<T: Real>(angle: T) -> Rotation3<T> { rotate(Vector3::y_axis(), angle) }
pub fn rotate_z<T: Real>(angle: T) -> Rotation3<T> { rotate(Vector3::z_axis(), angle) }

pub fn translate<T: Real>(amount: Vector3<T>) -> Translation3<T> {
	Translation3::from_vector(amount)
}

pub fn translate_x<T: Real>(distance: T) -> Translation3<T> { translate(Vector3::x_axis().unwrap() * distance) }
pub fn translate_y<T: Real>(distance: T) -> Translation3<T> { translate(Vector3::y_axis().unwrap() * distance) }
pub fn translate_z<T: Real>(distance: T) -> Translation3<T> { translate(Vector3::z_axis().unwrap() * distance) }

pub fn scale<T: Real>(factors: Vector3<T>) -> Affine3<T> {
	let mut result = Matrix4::<T>::identity();
	result[(0, 0)] = factors[0];
	result[(1, 1)] = factors[1];
	result[(2, 2)] = factors[2];
	Affine3::from_matrix_unchecked(result)
}

pub fn scale_x<T: Real>(f: T) -> Affine3<T> {
	let mut result = Matrix4::<T>::identity();
	result[(0, 0)] = f;
	Affine3::from_matrix_unchecked(result)
}

pub fn scale_y<T: Real>(f: T) -> Affine3<T> {
	let mut result = Matrix4::<T>::identity();
	result[(0, 0)] = f;
	Affine3::from_matrix_unchecked(result)
}

pub fn scale_z<T: Real>(f: T) -> Affine3<T> {
	let mut result = Matrix4::<T>::identity();
	result[(0, 0)] = f;
	Affine3::from_matrix_unchecked(result)
}
