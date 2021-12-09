pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRGB {
    pub fn new() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    // 0xFF_00_00_00 is red
    // 0x00_FF_00_00 is green
    // 0x00_00_FF_00 is blue
    // 0x00_00_00_FF is alpha
    pub fn from_u32(c: u32) -> Self {
        Self {
            r: (c >> 24) as u8,
            g: (c >> 16) as u8,
            b: (c >> 8) as u8,
        }
    }
}

impl ColorRGBA {
    pub fn new() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    // 0xFF0000
    pub fn from_u32(c: u32) -> Self {
        Self {
            r: (c >> 24) as u8,
            g: (c >> 16) as u8,
            b: (c >> 8) as u8,
            a: (c >> 0) as u8,
        }
    }
}

#[test]
fn test_i32_to_color() {
    let c = ColorRGB::from_u32(255 << 24);
    assert!(c.r == 255);
    let c = ColorRGB::from_u32(0xFF_00_00_00);
    assert!(c.r == 255);
    let c = ColorRGB::from_u32(0x00_FF_00_00);
    assert!(c.g == 255);
    let c = ColorRGB::from_u32(0x00_00_FF_00);
    assert!(c.b == 255);

    let c = ColorRGBA::from_u32(255 << 24);
    assert!(c.r == 255);
    let c = ColorRGBA::from_u32(0xFF_00_00_00);
    assert!(c.r == 255);
    let c = ColorRGBA::from_u32(0x00_FF_00_00);
    assert!(c.g == 255);
    let c = ColorRGBA::from_u32(0x00_00_FF_00);
    assert!(c.b == 255);
    let c = ColorRGBA::from_u32(0x00_00_00_FF);
    assert!(c.a == 255);

    let c = ColorRGBA::from_u32(0xFF_FF_FF_FF);
    assert!(c.r == 255 && c.g == 255 && c.b == 255 && c.a == 255);
}
