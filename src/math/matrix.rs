use crate::math::vector_const::*;
use crate::math::*;
use std::ops::*;

pub struct Mat2 {
    pub x: Vec3f,
    pub y: Vec3f,
    pub z: Vec3f,
}

pub struct Mat3 {
    pub x: Vec4f,
    pub y: Vec4f,
    pub z: Vec4f,
    pub w: Vec4f,
}

impl Index<usize> for Mat2 {
    type Output = Vec3f;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

impl Index<X> for Mat2 {
    type Output = Vec3f;

    fn index(&self, _: X) -> &Self::Output {
        &self.x
    }
}

impl Index<Y> for Mat2 {
    type Output = Vec3f;

    fn index(&self, _: Y) -> &Self::Output {
        &self.y
    }
}

impl Index<Z> for Mat2 {
    type Output = Vec3f;

    fn index(&self, _: Z) -> &Self::Output {
        &self.z
    }
}

impl Index<X> for Mat3 {
    type Output = Vec4f;

    fn index(&self, _: X) -> &Self::Output {
        &self.x
    }
}

impl Index<Y> for Mat3 {
    type Output = Vec4f;

    fn index(&self, _: Y) -> &Self::Output {
        &self.y
    }
}

impl Index<Z> for Mat3 {
    type Output = Vec4f;

    fn index(&self, _: Z) -> &Self::Output {
        &self.z
    }
}

impl Index<W> for Mat3 {
    type Output = Vec4f;

    fn index(&self, _: W) -> &Self::Output {
        &self.z
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
    type Output = Vec4f;

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
