#![allow(dead_code)]

use std::ops::{Add, Mul};

use math::matrix::matrix2::Matrix2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn new() -> Vector3 {
        Vector3 { x: 0., y: 0., z: 0. }
    }

    pub fn new_from(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn new_from_array(array: [f64; 3]) -> Vector3 {
        Vector3 { x: array[0], y: array[1], z: array[2] }
    }

    pub fn get_data(&self) -> [f64; 3] { [self.x, self.y, self.z] }

    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn dot(&self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vector3) -> Vector3 {
        let x_comp = Matrix2::new_from(
            self.y, self.z,
            other.y, other.z
        ).determinant();

        let y_comp = Matrix2::new_from(
            self.x, self.z,
            other.x, other.z
        ).determinant();

        let z_comp = Matrix2::new_from(
            self.x, self.y,
            other.x, other.y
        ).determinant();

        Vector3::new_from(x_comp, -y_comp, z_comp)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 { 
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

#[cfg(test)]
#[test]
fn test_vector3() {
    let vec1 = Vector3::new_from(3., -3., 1.);
    let vec2 = Vector3::new_from(4., 9., 2.);
    let cross = Vector3::new_from(-15., -2., 39.);

    assert_eq!(vec1.cross(vec2), cross);
    assert_eq!(vec1.cross(vec2), vec2.cross(vec1) * -1.);
    assert_eq!(vec1.cross(vec1), Vector3::new());
}