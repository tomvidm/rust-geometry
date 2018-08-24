#![allow(dead_code)]

use std::ops::{Add, Mul};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

impl Vector2 {
    pub fn new() -> Vector2 {
        Vector2 { x: 0., y: 0. }
    }

    pub fn new_from(x: f64, y: f64) -> Vector2 {
        Vector2 { x: x, y: y }
    }

    pub fn new_from_array(array: [f64; 2]) -> Vector2 {
        Vector2 { x: array[0], y: array[1] }
    }

    pub fn get_data(&self) -> [f64; 2] { [self.x, self.y] }

    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn dot(&self, other: Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }
    
    pub fn cross(&self, other: Vector2) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2 { 
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, other: f64) -> Vector2 {
        Vector2 {
            x: self.x * other,
            y: self.y * other
        }
    }
}