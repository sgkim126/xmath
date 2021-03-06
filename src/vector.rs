use matrix::*;
use std::f32;
use std::ops::*;

pub trait Vector {
    fn zero() -> Self;
    fn one() -> Self;
    fn infinity() -> Self;
    fn nan() -> Self;
    fn epsilon() -> Self;
    fn replicate(value: f32) -> Self;

    fn swizzle(&self, e0: usize, e1: usize, e2: usize, e3: usize) -> Self;
    fn permute(&self, other: &Self, permute_x: usize, permute_y: usize, permute_w: usize, permute_z: usize) -> Self;

    fn transform(&self, matrix: &Matrix) -> Self;

    fn min(&self, other: &Self) -> Self;
    fn max(&self, other: &Self) -> Self;

    fn round(&self) -> Self;
    fn trunc(&self) -> Self;
    fn floor(&self) -> Self;
    fn ceil(&self) -> Self;
    fn clamp(&self, min: &Self, max: &Self) -> Self;

    fn multiply_add(&self, mul: &Self, add: &Self) -> Self;

    fn splat_x(&self) -> Self;
    fn splat_y(&self) -> Self;
    fn splat_z(&self) -> Self;
    fn splat_w(&self) -> Self;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector for Vector2 {
    fn zero() -> Self {
        Vector2 {
            x: 0.0,
            y: 0.0,
        }
    }

    fn one() -> Self {
        Vector2 {
            x: 1.0,
            y: 1.0,
        }
    }

    fn infinity() -> Self {
        Vector2 {
            x: f32::INFINITY,
            y: f32::INFINITY,
        }
    }

    fn nan() -> Self {
        Vector2 {
            x: f32::NAN,
            y: f32::NAN,
        }
    }

    fn epsilon() -> Self {
        Vector2 {
            x: f32::EPSILON,
            y: f32::EPSILON,
        }
    }

    fn replicate(value: f32) -> Self {
        Vector2 {
            x: value,
            y: value,
        }
    }

    fn swizzle(&self, e0: usize, e1: usize, _e2: usize, _e3: usize) -> Self {
        assert!(e0 < 4);
        assert!(e1 < 4);
        Vector2 {
            x: self[e0],
            y: self[e1],
        }
    }

    fn permute(&self, other: &Self, permute_x: usize, permute_y: usize, _permute_z: usize, _permute_w: usize) -> Self {
        assert!(permute_x < 8);
        assert!(permute_y < 8);
        Vector2 {
            x: if permute_x < 4 { self[permute_x] } else { other[permute_x - 4] },
            y: if permute_y < 4 { self[permute_y] } else { other[permute_y - 4] },
        }
    }

    fn transform(&self, matrix: &Matrix) -> Self {

        let x = self.x * matrix[0][0] + self.y * matrix[1][0] + matrix[3][0];
        let y = self.x * matrix[0][1] + self.y * matrix[1][1] + matrix[3][1];
        Vector2 {
            x: x,
            y: y,
        }
    }

    fn min(&self, other: &Self) -> Self {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);

        Vector2 {
            x: x,
            y: y,
        }
    }
    fn max(&self, other: &Self) -> Self {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);

        Vector2 {
            x: x,
            y: y,
        }
    }

    fn round(&self) -> Self {
        let x = self.x.round();
        let y = self.y.round();

        Vector2 {
            x: x,
            y: y,
        }
    }
    fn trunc(&self) -> Self {
        let x = self.x.trunc();
        let y = self.y.trunc();

        Vector2 {
            x: x,
            y: y,
        }
    }
    fn floor(&self) -> Self {
        let x = self.x.floor();
        let y = self.y.floor();

        Vector2 {
            x: x,
            y: y,
        }
    }
    fn ceil(&self) -> Self {
        let x = self.x.ceil();
        let y = self.y.ceil();

        Vector2 {
            x: x,
            y: y,
        }
    }
    fn clamp(&self, min: &Self, max: &Self) -> Self {
        assert!(min.x < max.x);
        assert!(min.y < max.y);
        self.max(min).min(max)
    }

    fn multiply_add(&self, mul: &Self, add: &Self) -> Self {
        *self * *mul + *add
    }

    fn splat_x(&self) -> Self {
        let x = self.x;
        Vector2 {
            x: x,
            y: x,
        }
    }
    fn splat_y(&self) -> Self {
        let y = self.y;
        Vector2 {
            x: y,
            y: y,
        }
    }
    fn splat_z(&self) -> Self {
        let z = 0.0;
        Vector2 {
            x: z,
            y: z,
        }
    }
    fn splat_w(&self) -> Self {
        let w = 0.0;
        Vector2 {
            x: w,
            y: w,
        }
    }
}

impl Vector for Vector3 {
    fn zero() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    fn one() -> Self {
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    fn infinity() -> Self {
        Vector3 {
            x: f32::INFINITY,
            y: f32::INFINITY,
            z: f32::INFINITY,
        }
    }

    fn nan() -> Self {
        Vector3 {
            x: f32::NAN,
            y: f32::NAN,
            z: f32::NAN,
        }
    }

    fn epsilon() -> Self {
        Vector3 {
            x: f32::EPSILON,
            y: f32::EPSILON,
            z: f32::EPSILON,
        }
    }

    fn replicate(value: f32) -> Self {
        Vector3 {
            x: value,
            y: value,
            z: value,
        }
    }

    fn swizzle(&self, e0: usize, e1: usize, e2: usize, _e3: usize) -> Self {
        assert!(e0 < 4);
        assert!(e1 < 4);
        assert!(e2 < 4);
        Vector3 {
            x: self[e0],
            y: self[e1],
            z: self[e2],
        }
    }

    fn permute(&self, other: &Self, permute_x: usize, permute_y: usize, permute_z: usize, _permute_w: usize) -> Self {
        assert!(permute_x < 8);
        assert!(permute_y < 8);
        assert!(permute_z < 8);
        Vector3 {
            x: if permute_x < 4 { self[permute_x] } else { other[permute_x - 4] },
            y: if permute_y < 4 { self[permute_y] } else { other[permute_y - 4] },
            z: if permute_z < 4 { self[permute_z] } else { other[permute_z - 4] },
        }
    }

    fn transform(&self, matrix: &Matrix) -> Self {
        let x = self.x * matrix[0][0] + self.y * matrix[1][0] + self.z * matrix[2][0] + matrix[3][0];
        let y = self.x * matrix[0][1] + self.y * matrix[1][1] + self.z * matrix[2][1] + matrix[3][1];
        let z = self.x * matrix[0][2] + self.y * matrix[1][2] + self.z * matrix[2][2] + matrix[3][2];
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }

    fn min(&self, other: &Self) -> Self {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let z = self.z.min(other.z);

        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
    fn max(&self, other: &Self) -> Self {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let z = self.z.max(other.z);

        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }

    fn round(&self) -> Self {
        let x = self.x.round();
        let y = self.y.round();
        let z = self.z.round();

        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
    fn trunc(&self) -> Self {
        let x = self.x.trunc();
        let y = self.y.trunc();
        let z = self.z.trunc();

        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
    fn floor(&self) -> Self {
        let x = self.x.floor();
        let y = self.y.floor();
        let z = self.z.floor();

        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
    fn ceil(&self) -> Self {
        let x = self.x.ceil();
        let y = self.y.ceil();
        let z = self.z.ceil();

        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
    fn clamp(&self, min: &Self, max: &Self) -> Self {
        assert!(min.x < max.x);
        assert!(min.y < max.y);
        assert!(min.z < max.z);
        self.max(min).min(max)
    }

    fn multiply_add(&self, mul: &Self, add: &Self) -> Self {
        *self * *mul + *add
    }

    fn splat_x(&self) -> Self {
        let x = self.x;
        Vector3 {
            x: x,
            y: x,
            z: x,
        }
    }
    fn splat_y(&self) -> Self {
        let y = self.y;
        Vector3 {
            x: y,
            y: y,
            z: y,
        }
    }
    fn splat_z(&self) -> Self {
        let z = self.z;
        Vector3 {
            x: z,
            y: z,
            z: z,
        }
    }
    fn splat_w(&self) -> Self {
        let w = 0.0;
        Vector3 {
            x: w,
            y: w,
            z: w,
        }
    }
}

impl Vector for Vector4 {
    fn zero() -> Self {
        Vector4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
    fn one() -> Self {
        Vector4 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        }
    }

    fn infinity() -> Self {
        Vector4 {
            x: f32::INFINITY,
            y: f32::INFINITY,
            z: f32::INFINITY,
            w: f32::INFINITY,
        }
    }

    fn nan() -> Self {
        Vector4 {
            x: f32::NAN,
            y: f32::NAN,
            z: f32::NAN,
            w: f32::NAN,
        }
    }

    fn epsilon() -> Self {
        Vector4 {
            x: f32::EPSILON,
            y: f32::EPSILON,
            z: f32::EPSILON,
            w: f32::EPSILON,
        }
    }

    fn replicate(value: f32) -> Self {
        Vector4 {
            x: value,
            y: value,
            z: value,
            w: value,
        }
    }

    fn swizzle(&self, e0: usize, e1: usize, e2: usize, e3: usize) -> Self {
        assert!(e0 < 4);
        assert!(e1 < 4);
        assert!(e2 < 4);
        assert!(e3 < 4);
        Vector4 {
            x: self[e0],
            y: self[e1],
            z: self[e2],
            w: self[e3],
        }
    }

    fn permute(&self, other: &Self, permute_x: usize, permute_y: usize, permute_z: usize, permute_w: usize) -> Self {
        assert!(permute_x < 8);
        assert!(permute_y < 8);
        assert!(permute_z < 8);
        assert!(permute_w < 8);
        Vector4 {
            x: if permute_x < 4 { self[permute_x] } else { other[permute_x - 4] },
            y: if permute_y < 4 { self[permute_y] } else { other[permute_y - 4] },
            z: if permute_z < 4 { self[permute_z] } else { other[permute_z - 4] },
            w: if permute_w < 4 { self[permute_w] } else { other[permute_w - 4] },
        }
    }

    fn transform(&self, matrix: &Matrix) -> Self {
        let x = self.x * matrix[0][0] + self.y * matrix[1][0] + self.z * matrix[2][0] + self.w * matrix[3][0];
        let y = self.x * matrix[0][1] + self.y * matrix[1][1] + self.z * matrix[2][1] + self.w * matrix[3][1];
        let z = self.x * matrix[0][2] + self.y * matrix[1][2] + self.z * matrix[2][2] + self.w * matrix[3][2];
        let w = self.x * matrix[0][3] + self.y * matrix[1][3] + self.z * matrix[2][3] + self.w * matrix[3][3];
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    fn min(&self, other: &Self) -> Self {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let z = self.z.min(other.z);
        let w = self.w.min(other.w);

        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
    fn max(&self, other: &Self) -> Self {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let z = self.z.max(other.z);
        let w = self.w.max(other.w);

        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    fn round(&self) -> Self {
        let x = self.x.round();
        let y = self.y.round();
        let z = self.z.round();
        let w = self.w.round();

        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
    fn trunc(&self) -> Self {
        let x = self.x.trunc();
        let y = self.y.trunc();
        let z = self.z.trunc();
        let w = self.w.trunc();

        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
    fn floor(&self) -> Self {
        let x = self.x.floor();
        let y = self.y.floor();
        let z = self.z.floor();
        let w = self.w.floor();

        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
    fn ceil(&self) -> Self {
        let x = self.x.ceil();
        let y = self.y.ceil();
        let z = self.z.ceil();
        let w = self.w.ceil();

        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
    fn clamp(&self, min: &Self, max: &Self) -> Self {
        assert!(min.x < max.x);
        assert!(min.y < max.y);
        assert!(min.z < max.z);
        assert!(min.w < max.w);
        self.max(min).min(max)
    }


    fn multiply_add(&self, mul: &Self, add: &Self) -> Self {
        *self * *mul + *add
    }

    fn splat_x(&self) -> Self {
        let x = self.x;
        Vector4 {
            x: x,
            y: x,
            z: x,
            w: x,
        }
    }
    fn splat_y(&self) -> Self {
        let y = self.y;
        Vector4 {
            x: y,
            y: y,
            z: y,
            w: y,
        }
    }
    fn splat_z(&self) -> Self {
        let z = self.z;
        Vector4 {
            x: z,
            y: z,
            z: z,
            w: z,
        }
    }
    fn splat_w(&self) -> Self {
        let w = self.w;
        Vector4 {
            x: w,
            y: w,
            z: w,
            w: w,
        }
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Vector2 {
            x: x,
            y: y,
        }
    }
}
impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Vector2 {
            x: x,
            y: y,
        }
    }
}
impl Div for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: Vector2) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        Vector2 {
            x: x,
            y: y,
        }
    }
}
impl Mul for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        Vector2 {
            x: x,
            y: y,
        }
    }
}


impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
}
impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
}
impl Div for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Vector3) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        let z = self.z / rhs.z;
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
}
impl Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
}

impl Add for Vector4 {
    type Output = Vector4;
    fn add(self, rhs: Vector4) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        let w = self.w + rhs.w;
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}
impl Sub for Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: Vector4) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        let w = self.w - rhs.w;
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}
impl Mul for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        let w = self.w * rhs.w;
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}
impl Div for Vector4 {
    type Output = Vector4;
    fn div(self, rhs: Vector4) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        let z = self.z / rhs.z;
        let w = self.w / rhs.w;
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Vector2 {
            x: x,
            y: y,
        }
    }
}
impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
}
impl Mul<f32> for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        let w = self.w * rhs;
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}

impl Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        Vector2 {
            x: x,
            y: y,
        }
    }
}
impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }
}
impl Neg for Vector4 {
    type Output = Vector4;
    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        let w = -self.w;
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}

const ZERO: &'static f32 = &0.0;

impl Index<usize> for Vector2 {
    type Output = f32;
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => ZERO,
            3 => ZERO,
            _ => panic!("index must be between 0~3, but {}", index),
        }
    }
}
impl Index<usize> for Vector3 {
    type Output = f32;
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => ZERO,
            _ => panic!("index must be between 0~3, but {}", index),
        }
    }
}
impl Index<usize> for Vector4 {
    type Output = f32;
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index must be between 0~3, but {}", index),
        }
    }
}
