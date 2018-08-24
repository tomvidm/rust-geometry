#![allow(dead_code)]

use math::vector::vector2::Vector2;

#[derive(Clone, Copy, PartialEq)]
pub struct Matrix2 {
    data: [[f64; 2]; 2]
}

impl Matrix2 {
    pub fn identity() -> Matrix2 {
        Matrix2::new_from(1., 0., 0., 1.)
    }

    pub fn new() -> Matrix2 {
        Matrix2 { 
            data: [
                [0., 0.],
                [0., 0.]
            ]
        }
    }

    pub fn new_from(a: f64, b: f64, c: f64, d: f64) -> Matrix2 {
        Matrix2{
            data: [
                [a, b],
                [c, d]
            ]
        }
    }

    pub fn new_from_columns(columns: [Vector2; 2]) -> Matrix2 {
        let mut mat = Matrix2::new();
        for n in 0..3 {
            mat.data[0][n] = columns[n].x;
            mat.data[1][n] = columns[n].y;
        }
        return mat
    }

    pub fn get_row(&self, row: usize) -> Vector2 {
        assert!(row < 2);
        Vector2::new_from_array(self.data[row])
    }

    pub fn get_col(&self, col: usize) -> Vector2 {
        assert!(col < 2);
        Vector2::new_from(self.data[0][col], self.data[1][col])
    }

    pub fn transform_vector2(&self, other: Vector2) -> Vector2 {
        return Vector2::new_from(
            self.data[0][0] * other.x + self.data[0][1] * other.y,
            self.data[1][0] * other.x + self.data[1][1] * other.y
        )
    }

    pub fn transform_matrix(&self, other: Matrix2) -> Matrix2 {
        Matrix2::new_from_columns([
            self.transform_vector2(other.get_col(0)),
            self.transform_vector2(other.get_col(1))
        ])
    }

    pub fn determinant(&self) -> f64 {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }

    pub fn transpose(&self) -> Matrix2 {
        Matrix2::new_from_columns([self.get_row(0), self.get_row(1)])
    }
}

#[cfg(test)]
#[test]
fn test_matrix2() {
    let mat2 = Matrix2::new_from(2., 1., 1., 2.);
    assert_eq!(mat2.determinant(), 3.);
    let vec2 = Vector2::new_from(2., 2.);
    assert_eq!(mat2.transform_vector2(vec2), Vector2::new_from(6., 6.));
}