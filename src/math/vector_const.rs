pub type Prec = f32;

use std::ops::*;

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

impl Zero for Prec {
    fn zero() -> Self {
        Prec::default()
    }
}

impl One for Prec {
    fn one() -> Self {
        1.0
    }
}

pub struct VecN<T, const N: usize> {
    inner: [T; N],
}

pub type Vec2f = VecN<Prec, 2>;
pub type Vec3f = VecN<Prec, 3>;
pub type Vec4f = VecN<Prec, 4>;

impl<T, const N: usize> VecN<T, N>
where
    T: Zero + Default + Copy,
{
    pub fn new() -> Self {
        Self {
            inner: [T::zero(); N],
        }
    }

    pub fn as_arr(self) -> [T; N] {
        self.inner
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

#[test]
fn test_new() {
    let v1 = Vec2f::new();
    let v2 = Vec3f::new();
    println!("{}", v2[X]);
}
