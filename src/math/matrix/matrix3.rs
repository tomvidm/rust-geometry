#![allow(dead_code)]

use math::vector::vector3::Vector3;
use math::vector::vector2::Vector2;
use math::matrix::matrix2::Matrix2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Matrix3 {
    data: [[f64; 3]; 3]
}

impl Matrix3 {
    pub fn identity() -> Matrix3 {
        Matrix3::new_from(1., 0., 0., 0., 1., 0., 0., 0., 1.)
    }

    pub fn new() -> Matrix3 {
        Matrix3 { 
            data: [
                [0., 0., 0.],
                [0., 0., 0.],
                [0., 0., 0.]
            ]
        }
    }

    pub fn new_from(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64, i: f64) -> Matrix3 {
        Matrix3 {
            data: [
                [a, b, c],
                [d, e, f],
                [g, h, i]
            ]
        }
    }

    pub fn new_from_columns(columns: [Vector3; 3]) -> Matrix3 {
        let mut mat = Matrix3::new();
        for n in 0..3 {
            mat.data[0][n] = columns[n].x;
            mat.data[1][n] = columns[n].y;
            mat.data[2][n] = columns[n].z;
        }
        return mat
    }

    pub fn get_row(&self, row: usize) -> Vector3 {
        assert!(row < 3);
        Vector3::new_from_array(self.data[row])
    }

    pub fn get_col(&self, col: usize) -> Vector3 {
        assert!(col < 3);
        Vector3::new_from(self.data[0][col], self.data[1][col], self.data[2][col])
    }

    pub fn transform_vector2(&self, other: Vector2) -> Vector2 {
        let homogenous = 
            self.transform_vector3(Vector3::new_from(other.x, other.y, 1.));
        return Vector2::new_from(homogenous.x, homogenous.y)
    }

    pub fn transform_vector3(&self, other: Vector3) -> Vector3 {
        return Vector3::new_from(
            self.data[0][0] * other.x + self.data[0][1] * other.y + self.data[0][2] * other.z,
            self.data[1][0] * other.x + self.data[1][1] * other.y + self.data[1][2] * other.z,
            self.data[2][0] * other.x + self.data[2][1] * other.y + self.data[2][2] * other.z
        )
    }

    pub fn transform_matrix(&self, other: Matrix3) -> Matrix3 {
        Matrix3::new_from_columns([
            self.transform_vector3(other.get_col(0)),
            self.transform_vector3(other.get_col(1)),
            self.transform_vector3(other.get_col(2))
        ])
    }

    pub fn determinant(&self) -> f64 {
        let submat_a = Matrix2::new_from(
            self.data[1][1], self.data[1][2],
            self.data[2][1], self.data[2][2]
        );
        let det_a = self.data[0][0] * submat_a.determinant();

        let submat_b = Matrix2::new_from(
            self.data[1][0], self.data[1][2],
            self.data[2][0], self.data[2][2]
        );
        let det_b = self.data[0][1] * submat_b.determinant();

        let submat_c = Matrix2::new_from(
            self.data[1][0], self.data[1][1],
            self.data[2][0], self.data[2][1]
        );
        let det_c = self.data[0][2] * submat_c.determinant();
        
        return det_a - det_b + det_c
    }

    pub fn transpose(&self) -> Matrix3 {
        Matrix3::new_from_columns([self.get_row(0), self.get_row(1), self.get_row(2)])
    }
}

#[cfg(test)]
#[test]
fn test_matrix3() {
    let mat3 = Matrix3::new_from(2., 1., 1., 1., 2., 1., 1., 1., 2.);
    assert_eq!(mat3.determinant(), 4.);
    let vec3 = Vector3::new_from(2., 2., 2.);
    assert_eq!(mat3.transform_vector3(vec3), Vector3::new_from(8., 8., 8.));
    assert_eq!(mat3.get_col(0), Vector3::new_from(2., 1., 1.));
    assert_eq!(mat3.get_row(0), Vector3::new_from(2., 1., 1.));
}

#[test]
fn test_composition_of_matrix3() {
    let mat_a = Matrix3::new_from(1., 2., 3., 4., 5., 6., 7., 8., 9.);
    let mat_b = Matrix3::new_from(9., 8., 7., 6., 5., 4., 3., 2., 1.);

    let res = Matrix3::new_from(30., 24., 18., 84., 69., 54., 138., 114., 90.);

    assert_eq!(mat_a.transform_matrix(mat_b), res);
    assert_eq!(Matrix3::identity().transform_matrix(mat_a), mat_a);
}