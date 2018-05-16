use nalgebra::{
	Real,
	Rotation3,
	Unit,
	Vector3
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
