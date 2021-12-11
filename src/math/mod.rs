use std::{
    ops::{Index, Neg},
    process::Output,
    sync::Once,
};

pub mod color;
pub mod matrix;
pub mod vector;
pub mod vector_const;

const CHAR_DIM: [char; 4] = ['x', 'y', 'z', 'w'];

// const ITER_DIMS: (X, Y, Z, W) = (X, Y, Z, W);

// impl Index<X> for (X, Y, Z, W) {
//     type Output = X;

//     fn index(&self, index: X) -> &Self::Output {
//         &self.0
//     }
// }

// impl Index<Y> for (X, Y, Z, W) {
//     type Output = Y;

//     fn index(&self, index: Y) -> &Self::Output {
//         &self.1
//     }
// }

// impl Index<Z> for (X, Y, Z, W) {
//     type Output = Z;

//     fn index(&self, index: Z) -> &Self::Output {
//         &self.2
//     }
// }

// impl Index<W> for (X, Y, Z, W) {
//     type Output = W;

//     fn index(&self, index: W) -> &Self::Output {
//         &self.3
//     }
// }

pub struct X;
pub struct Y;
pub struct Z;
pub struct W;

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

pub trait Sqrt<T> {
    fn sqrt(&self) -> T;
}

pub trait Sin<T> {
    fn sin(&self) -> T;
}

pub trait Cos<T> {
    fn cos(&self) -> T;
}

pub trait Abs<T> {
    fn abs(&self) -> T;
}

pub trait Clamp<T> {
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl Sqrt<f32> for f32 {
    fn sqrt(&self) -> f32 {
        f32::sqrt(*self)
    }
}

impl Sin<f32> for f32 {
    fn sin(&self) -> f32 {
        f32::sin(*self)
    }
}

impl Cos<f32> for f32 {
    fn cos(&self) -> f32 {
        f32::cos(*self)
    }
}

impl Abs<f32> for f32 {
    fn abs(&self) -> f32 {
        f32::abs(*self)
    }
}

impl Clamp<f32> for f32 {
    fn clamp(self, min: Self, max: Self) -> Self {
        f32::clamp(self, min, max)
    }
}

impl Sqrt<f64> for f64 {
    fn sqrt(&self) -> f64 {
        f64::sqrt(*self)
    }
}

impl Sin<f64> for f64 {
    fn sin(&self) -> f64 {
        f64::sin(*self)
    }
}

impl Cos<f64> for f64 {
    fn cos(&self) -> f64 {
        f64::cos(*self)
    }
}

impl Abs<f64> for f64 {
    fn abs(&self) -> f64 {
        f64::abs(*self)
    }
}

impl Clamp<f64> for f64 {
    fn clamp(self, min: Self, max: Self) -> Self {
        f64::clamp(self, min, max)
    }
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0
    }
}

impl One for f32 {
    fn one() -> Self {
        1.0
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
}

impl One for f64 {
    fn one() -> Self {
        1.0
    }
}

pub trait MemSize {
    fn mem_size() -> usize;
}

impl<T> MemSize for T {
    fn mem_size() -> usize {
        std::mem::size_of::<T>()
    }
}
