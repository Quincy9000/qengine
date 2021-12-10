use crate::math::vector::*;

use std::ops::*;

const CHAR_DIM: [char; 4] = ['x', 'y', 'z', 'w'];

pub struct Mat2 {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
}

pub struct Mat3 {
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4,
    pub w: Vec4,
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
            3 => &self.w,
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
            3 => &mut self.w,
            _ => panic!("Index out of range!"),
        }
    }
}

impl Mat2 {
    pub fn identity() -> Self {
        Self {
            x: [1.0, 0.0, 0.0].into(),
            y: [0.0, 1.0, 0.0].into(),
            z: [0.0, 0.0, 1.0].into(),
        }
    }

    pub fn len(self) -> usize {
        9
    }
}

impl Mat3 {
    pub fn identity() -> Self {
        Self {
            x: [1.0, 0.0, 0.0, 0.0].into(),
            y: [0.0, 1.0, 0.0, 0.0].into(),
            z: [0.0, 0.0, 1.0, 0.0].into(),
            w: [0.0, 0.0, 0.0, 1.0].into(),
        }
    }

    pub fn len(&self) -> usize {
        16
    }
}

impl std::fmt::Display for Mat2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..3 {
            write!(f, "[ ")?;
            for j in 0..3 {
                if j < 2 {
                    write!(f, "{}{}: {}, ", CHAR_DIM[j], CHAR_DIM[i], self[i][j])?;
                } else {
                    write!(f, "{}{}: {} ", CHAR_DIM[j], CHAR_DIM[i], self[i][j])?;
                }
            }
            writeln!(f, "]")?;
        }
        writeln!(f, "")
    }
}

impl std::fmt::Display for Mat3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..4 {
            write!(f, "[ ")?;
            for j in 0..4 {
                if j < 3 {
                    write!(f, "{}{}: {}, ", CHAR_DIM[j], CHAR_DIM[i], self[i][j])?;
                } else {
                    write!(f, "{}{}: {} ", CHAR_DIM[j], CHAR_DIM[i], self[i][j])?;
                }
            }
            if i < 3 {
                writeln!(f, "]")?;
            } else {
                write!(f, "]")?;
            }
        }
        write!(f, "")
    }
}

#[test]
fn test_mat_fmt() {
    let _a = Mat3::identity();
    println!("{}\n", _a);
    let _a = Mat2::identity();
    println!("{}", _a);
}
