pub type Precision = f32;

pub struct Vector<const N: usize> {
    data: [Precision; N],
}

pub trait IntoPrecision {
    fn cast(self) -> Precision;
}

impl IntoPrecision for i32 {
    fn cast(self) -> Precision {
        self as Precision
    }
}

impl IntoPrecision for i64 {
    fn cast(self) -> Precision {
        self as Precision
    }
}

impl IntoPrecision for f32 {
    fn cast(self) -> Precision {
        self as Precision
    }
}

impl IntoPrecision for f64 {
    fn cast(self) -> Precision {
        self as Precision
    }
}

pub fn vec2<T, U>(t: T, u: U) -> Vector<2>
where
    T: IntoPrecision,
    U: IntoPrecision,
{
    Vector {
        data: [t.cast(), u.cast()],
    }
}

pub fn vec3<T, U, V>(t: T, u: U, v: V) -> Vector<3>
where
    T: IntoPrecision,
    U: IntoPrecision,
    V: IntoPrecision,
{
    Vector {
        data: [t.cast(), u.cast(), v.cast()],
    }
}

pub fn vec4<T, U, V, W>(t: T, u: U, v: V, w: W) -> Vector<4>
where
    T: IntoPrecision,
    U: IntoPrecision,
    V: IntoPrecision,
    W: IntoPrecision,
{
    Vector {
        data: [t.cast(), u.cast(), v.cast(), w.cast()],
    }
}

impl<const N: usize> Clone for Vector<N> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<const N: usize> Copy for Vector<N> {}

impl<const N: usize> std::fmt::Debug for Vector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector").field("data", &self.data).finish()
    }
}

impl<const N: usize> Vector<N> {
    pub fn new() -> Self {
        Self { data: [0.0; N] }
    }

    pub fn dist(self) -> Precision {
        let mut n = Precision::default();

        self.data.iter().for_each(|num| {
            n += *num * *num;
        });

        n.sqrt()
    }

    pub fn norm(self) -> Vector<N> {
        let mut new = Self::new();

        new
    }

    // /pub fn get_mut(&mut self) -> (x, y, z) {}
}

impl<const N: usize> std::ops::Index<usize> for Vector<N> {
    type Output = Precision;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize> std::ops::IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize> std::ops::Add for Vector<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [Precision::default(); N];
        self.data
            .iter()
            .enumerate()
            .zip(rhs.data.iter())
            .for_each(|((i, a), b)| {
                data[i] = *a + *b;
            });
        Self { data }
    }
}

impl<const N: usize> std::ops::Sub for Vector<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [Precision::default(); N];
        self.data
            .iter()
            .enumerate()
            .zip(rhs.data.iter())
            .for_each(|((i, a), b)| {
                data[i] = *a - *b;
            });
        Self { data }
    }
}

impl<const N: usize> std::ops::Mul<Precision> for Vector<N> {
    type Output = Self;

    fn mul(self, t: Precision) -> Self::Output {
        let mut data = [Precision::default(); N];

        self.data.iter().enumerate().for_each(|(i, v)| {
            data[i] = *v * t;
        });

        Self { data }
    }
}

mod tests {
    // Complains about this
    // but it does need it
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn vec3_into_test() {
        let v = vec3(0, 0, 0);
        println!("{:?}", v);
    }
}
