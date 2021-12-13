use crate::math::vector::*;
use crate::math::*;
use std::fmt::Display;
use std::ops::*;

pub struct Mat2<T> {
    pub x: VecN<T, 3>,
    pub y: VecN<T, 3>,
    pub z: VecN<T, 3>,
}

pub struct Mat3<T> {
    pub x: VecN<T, 4>,
    pub y: VecN<T, 4>,
    pub z: VecN<T, 4>,
    pub w: VecN<T, 4>,
}

pub type Mat2f = Mat2<f32>;
pub type Mat2d = Mat2<f64>;
pub type Mat3f = Mat3<f32>;
pub type Mat3d = Mat3<f64>;

impl<T> Index<usize> for Mat2<T> {
    type Output = VecN<T, 3>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

impl<T> Index<X> for Mat2<T> {
    type Output = VecN<T, 3>;

    fn index(&self, _: X) -> &Self::Output {
        &self.x
    }
}

impl<T> Index<Y> for Mat2<T> {
    type Output = VecN<T, 3>;

    fn index(&self, _: Y) -> &Self::Output {
        &self.y
    }
}

impl<T> Index<Z> for Mat2<T> {
    type Output = VecN<T, 3>;

    fn index(&self, _: Z) -> &Self::Output {
        &self.z
    }
}

impl<T> Index<X> for Mat3<T> {
    type Output = VecN<T, 4>;

    fn index(&self, _: X) -> &Self::Output {
        &self.x
    }
}

impl<T> Index<Y> for Mat3<T> {
    type Output = VecN<T, 4>;

    fn index(&self, _: Y) -> &Self::Output {
        &self.y
    }
}

impl<T> Index<Z> for Mat3<T> {
    type Output = VecN<T, 4>;

    fn index(&self, _: Z) -> &Self::Output {
        &self.z
    }
}

impl<T> Index<W> for Mat3<T> {
    type Output = VecN<T, 4>;

    fn index(&self, _: W) -> &Self::Output {
        &self.z
    }
}

impl<T> IndexMut<usize> for Mat2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

impl<T> Index<usize> for Mat3<T> {
    type Output = VecN<T, 4>;

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

impl<T> IndexMut<usize> for Mat3<T> {
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

impl<T> Mat2<T>
where
    T: One + Zero,
{
    pub fn identity() -> Self {
        Self {
            x: [T::one(), T::zero(), T::zero()].into(),
            y: [T::zero(), T::one(), T::zero()].into(),
            z: [T::zero(), T::zero(), T::one()].into(),
        }
    }

    pub fn len(self) -> usize {
        9
    }
}

impl<T> Mat3<T>
where
    T: Zero + One,
{
    pub fn identity() -> Self {
        Self {
            x: [T::one(), T::zero(), T::zero(), T::zero()].into(),
            y: [T::zero(), T::one(), T::zero(), T::zero()].into(),
            z: [T::zero(), T::zero(), T::one(), T::zero()].into(),
            w: [T::zero(), T::zero(), T::zero(), T::one()].into(),
        }
    }

    pub fn len(&self) -> usize {
        16
    }
}

impl<T> std::fmt::Display for Mat2<T>
where
    T: Display,
{
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

impl<T> std::fmt::Display for Mat3<T>
where
    T: Display,
{
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
    let _a = Mat3f::identity();
    println!("{}\n", _a);
    let _a = Mat2f::identity();
    println!("{}", _a);
}
