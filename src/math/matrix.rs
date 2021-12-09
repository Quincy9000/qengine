use crate::math::vector::*;

use std::ops::*;

pub struct Mat2 {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
}

impl Index<usize> for Mat2 {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

impl IndexMut<usize> for Mat2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

impl Index<usize> for Mat3 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

impl IndexMut<usize> for Mat3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

pub struct Mat3 {
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4,
}
