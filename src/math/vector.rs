use std::ops::*;

#[derive(Clone, Copy)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy)]
pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Into<Vec2> for Vec3 {
    fn into(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<Vec3> for Vec2 {
    fn into(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: 0f32,
        }
    }
}

impl Into<Vec4> for Vec3 {
    fn into(self) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: 0f32,
            w: 0f32,
        }
    }
}

impl Into<Vec3> for Vec4 {
    fn into(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Vec2 {
    pub fn as_arr(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl Vec3 {
    pub fn as_arr(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Vec4 {
    pub fn as_arr(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(a: [f32; 2]) -> Self {
        Vec2 { x: a[0], y: a[1] }
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(a: [f32; 3]) -> Self {
        Vec3 {
            x: a[0],
            y: a[1],
            z: a[2],
        }
    }
}

impl From<[f32; 4]> for Vec4 {
    fn from(a: [f32; 4]) -> Self {
        Vec4 {
            x: a[0],
            y: a[1],
            z: a[2],
            w: a[3],
        }
    }
}

impl Index<usize> for Vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of range!"),
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

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

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of range!"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

impl IndexMut<usize> for Vec4 {
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

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, b: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + b.x,
            y: self.y + b.y,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, b: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, b: Vec4) -> Self::Output {
        Vec4 {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
            w: self.w + b.w,
        }
    }
}

impl Vec2 {
    pub fn new() -> Self {
        Self { x: 0f32, y: 0f32 }
    }
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }
}

impl Vec4 {
    pub fn new() -> Self {
        Self {
            x: 0f32,
            y: 0f32,
            z: 0f32,
            w: 0f32,
        }
    }
}
