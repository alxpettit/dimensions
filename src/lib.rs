use derive_more::{Add, Div, Mul, Sub};
use rusttype::Scale;

#[derive(Add, Sub, Div, Mul, Clone, Copy, Debug)]
#[mul(forward)]
#[div(forward)]
pub struct Dimensions {
    pub x: f32,
    pub y: f32,
}

impl Dimensions {
    pub fn new(x: impl Into<i32>, y: impl Into<i32>) -> Self {
        let x = x.into() as f32;
        let y = y.into() as f32;
        Self { x, y }
    }

    pub fn width(&self) -> f32 {
        self.x
    }

    pub fn height(&self) -> f32 {
        self.y
    }

    pub fn i32_width(&self) -> i32 {
        self.x as i32
    }

    pub fn i32_height(&self) -> i32 {
        self.y as i32
    }

    pub fn zero() -> Self {
        Self { x: 0f32, y: 0f32 }
    }
}

impl std::ops::Add<i32> for Dimensions {
    type Output = Dimensions;
    fn add(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x + rhs as f32,
            y: self.y + rhs as f32,
        }
    }
}

impl From<f32> for Dimensions {
    fn from(value: f32) -> Self {
        Self { x: value, y: value }
    }
}

impl From<i32> for Dimensions {
    fn from(value: i32) -> Self {
        Self {
            x: value as f32,
            y: value as f32,
        }
    }
}

impl std::ops::Sub<i32> for Dimensions {
    type Output = Dimensions;
    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x - rhs as f32,
            y: self.y - rhs as f32,
        }
    }
}

impl std::ops::Mul<i32> for Dimensions {
    type Output = Dimensions;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
        }
    }
}

impl std::ops::Div<f32> for Dimensions {
    type Output = Dimensions;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Into<Scale> for Dimensions {
    fn into(self) -> Scale {
        Scale {
            x: self.x,
            y: self.y,
        }
    }
}

impl From<(i32, i32)> for Dimensions {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0 as f32,
            y: value.1 as f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
