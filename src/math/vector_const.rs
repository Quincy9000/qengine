use crate::math::*;
use std::{fmt::Display, ops::*};

#[derive(Clone, Copy)]
pub struct VecN<T, const N: usize> {
    inner: [T; N],
}

pub type Vec2f = VecN<f32, 2>;
pub type Vec3f = VecN<f32, 3>;
pub type Vec4f = VecN<f32, 4>;

pub type Vec2d = VecN<f64, 2>;
pub type Vec3d = VecN<f64, 3>;
pub type Vec4d = VecN<f64, 4>;

impl<T, const N: usize> VecN<T, N>
where
    T: Zero + Copy,
    T: Default + Clone,
    T: Add<Output = T> + Sub<Output = T>,
    T: Mul<Output = T> + Div<Output = T>,
    T: Sqrt<T> + Cos<T> + Sin<T> + Abs<T>,
    T: PartialEq<T>,
{
    pub fn new() -> Self {
        Self {
            inner: [T::zero(); N],
        }
    }

    pub fn as_arr(&self) -> [T; N] {
        self.inner
    }

    pub fn mag(&self) -> T {
        let mut total = T::default();
        for i in 0..N {
            total = self[i] * self[i] + total;
        }
        total.sqrt()
    }

    pub fn norm(self) -> Self {
        let mag = self.mag();

        let mut inner = self.inner;

        if mag == T::zero() {
            VecN::zero()
        } else {
            inner.iter_mut().enumerate().for_each(|(i, val)| {
                *val = self.inner[i] / mag;
            });

            VecN { inner }
        }
    }

    pub fn lerp<S: Into<Self>>(self, e: S, t: T) -> Self
    where
        Self: Copy,
    {
        self + (e.into() - self) * t
    }
}

impl<T, const N: usize> Zero for VecN<T, N>
where
    T: Zero + Copy,
{
    fn zero() -> Self {
        Self {
            inner: [T::zero(); N],
        }
    }
}

impl<T, const N: usize> Display for VecN<T, N>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{").expect("Write failed");
        for i in 0..N {
            let char = CHAR_DIM[i];
            write!(f, "{}: {}", char, self.inner[i]).expect("Write failed");
            if i < N - 1 {
                write!(f, ", ").expect("Write failed")
            }
        }
        write!(f, "}}")
    }
}

impl<T, const N: usize> Index<usize> for VecN<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for VecN<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<T, const N: usize> Index<X> for VecN<T, N> {
    type Output = T;

    fn index(&self, _: X) -> &Self::Output {
        &self.inner[0]
    }
}

impl<T, const N: usize> Index<Y> for VecN<T, N> {
    type Output = T;

    fn index(&self, _: Y) -> &Self::Output {
        &self.inner[1]
    }
}

impl<T, const N: usize> Index<Z> for VecN<T, N> {
    type Output = T;

    fn index(&self, _: Z) -> &Self::Output {
        &self.inner[2]
    }
}

impl<T, const N: usize> Index<W> for VecN<T, N> {
    type Output = T;

    fn index(&self, _: W) -> &Self::Output {
        &self.inner[3]
    }
}

impl<T, const N: usize> IndexMut<X> for VecN<T, N> {
    fn index_mut(&mut self, _: X) -> &mut Self::Output {
        &mut self.inner[0]
    }
}

impl<T, const N: usize> IndexMut<Y> for VecN<T, N> {
    fn index_mut(&mut self, _: Y) -> &mut Self::Output {
        &mut self.inner[1]
    }
}

impl<T, const N: usize> IndexMut<Z> for VecN<T, N> {
    fn index_mut(&mut self, _: Z) -> &mut Self::Output {
        &mut self.inner[2]
    }
}

impl<T, const N: usize> IndexMut<W> for VecN<T, N> {
    fn index_mut(&mut self, _: W) -> &mut Self::Output {
        &mut self.inner[3]
    }
}

impl<T, const N: usize> From<[T; N]> for VecN<T, N> {
    fn from(v: [T; N]) -> Self {
        Self { inner: v }
    }
}

impl<T, const N: usize> Add for VecN<T, N>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, b: Self) -> Self::Output {
        let mut inner = self.inner;

        for i in 0..N {
            inner[i] = self[i] + b[i];
        }

        Self { inner }
    }
}

impl<T, const N: usize> Sub for VecN<T, N>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, b: Self) -> Self::Output {
        let mut inner = self.inner;

        for i in 0..N {
            inner[i] = self[i] - b[i];
        }

        Self { inner }
    }
}

impl<T, const N: usize> Mul<T> for VecN<T, N>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, b: T) -> Self::Output {
        let mut inner = self.inner;

        for i in 0..N {
            inner[i] = self[i] * b;
        }

        Self { inner }
    }
}

impl<T, const N: usize> Div<T> for VecN<T, N>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, b: T) -> Self::Output {
        let mut inner = self.inner;

        for i in 0..N {
            inner[i] = self[i] / b;
        }

        Self { inner }
    }
}

#[test]
fn test_new() {
    let mut v1 = Vec2f::from([2.0, 1.0]);
    v1[X] = 10.0;
    println!("{}", v1[X]);
    println!("{}", v1[Y]);
    let v2 = Vec3f::new();
    println!("{}", v2[Y]);
}

#[test]
fn test_math() {
    let v = Vec2d::from([1.0, 1.0]).mag();
    println!("{}", v);
    let v = Vec2f::from([1.0, 0.0]).norm();
    println!("{}", v);
}

#[test]
fn test_lerp() {
    let mut v = Vec2f::from([0.0, 0.0]);

    for _ in 0..10_000 {
        v = v.lerp([10.0, 10.0], 0.1);
        println!("{}", v);
    }
}
