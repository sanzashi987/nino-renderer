#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod bresenham_line {
    use crate::image::ColorAttachment;
    use crate::math;
    pub fn draw_line(
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        color: &math::Vec4,
        color_pool: &mut ColorAttachment,
    ) {
        let mut dy = (y1 - y0).abs();
        let mut dx = (x1 - x0).abs();
        let mut x = x0;
        let mut y = y0;
        let mut step_x = if x1 > x0 { 1 } else { -1 };
        let mut step_y = if y1 > y0 { 1 } else { -1 };
        let y_grows_faster = dx < dy;
        let final_x = if y_grows_faster { y1 } else { x1 };
        if y_grows_faster {
            std::mem::swap(&mut dx, &mut dy);
            std::mem::swap(&mut x, &mut y);
            std::mem::swap(&mut step_x, &mut step_y);
        }
        let mut e = -dx;
        let step = 2 * dy;
        let desc = -2 * dx;
        while x != final_x {
            if y_grows_faster {
                color_pool.set(y as u32, x as u32, color);
            } else {
                color_pool.set(x as u32, y as u32, color);
            }
            x += step_x;
            e += step;
            y
                += if e >= 0 {
                    e += desc;
                    step_y
                } else {
                    0
                };
        }
    }
}
pub mod math {
    mod mat {
        use super::{Vec3, Vec4};
        use std::ops::{Div, Mul};
        pub struct Mat2 {
            data: [f32; 2 * 2],
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Mat2 {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Mat2",
                    "data",
                    &&self.data,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Mat2 {
            #[inline]
            fn clone(&self) -> Mat2 {
                let _: ::core::clone::AssertParamIsClone<[f32; 2 * 2]>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Mat2 {}
        #[automatically_derived]
        impl ::core::default::Default for Mat2 {
            #[inline]
            fn default() -> Mat2 {
                Mat2 {
                    data: ::core::default::Default::default(),
                }
            }
        }
        impl Mat2 {
            pub fn from_row(data: &[f32; 2 * 2]) -> Mat2 {
                Mat2 { data: data.clone() }
            }
            pub fn from_col(data: &[f32; 2 * 2]) -> Mat2 {
                let mut mat = Mat2::zeros();
                for x in 0..2 {
                    for y in 0..2 {
                        mat.set(x, y, data[y + 2 * x]);
                    }
                }
                mat
            }
            pub fn zeros() -> Mat2 {
                Mat2 { data: [0.; 2 * 2] }
            }
            pub fn ones() -> Mat2 {
                Mat2 { data: [1.; 2 * 2] }
            }
            pub fn identity() -> Mat2 {
                let mut mat = Mat2::zeros();
                for i in 0..2 {
                    mat.set(i, i, 1.0);
                }
                mat
            }
            pub fn get(&self, x: usize, y: usize) -> f32 {
                self.data[x + y * 2]
            }
            pub fn set(&mut self, x: usize, y: usize, value: f32) {
                self.data[x + y * 2] = value;
            }
            pub fn transpose(&self) -> Mat2 {
                let mut result = Mat2::identity();
                for x in 0..2 {
                    for y in 0..2 {
                        result.set(y, x, self.get(x, y));
                    }
                }
                result
            }
        }
        impl Mul for Mat2 {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let mut mat = Mat2::zeros();
                for y in 0..2 {
                    for x in 0..2 {
                        let mut sum = 0.0;
                        for d in 0..2 {
                            sum += self.get(d, y) * rhs.get(x, d);
                        }
                        mat.set(x, y, sum);
                    }
                }
                mat
            }
        }
        impl Mul<f32> for Mat2 {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self::Output {
                let mut mat = Mat2::zeros();
                for x in 0..2 {
                    for y in 0..2 {
                        mat.set(x, y, self.get(x, y) * rhs);
                    }
                }
                mat
            }
        }
        impl Div<f32> for Mat2 {
            type Output = Self;
            fn div(self, rhs: f32) -> Self::Output {
                self * (1.0 / rhs)
            }
        }
        impl PartialEq for Mat2 {
            fn eq(&self, other: &Self) -> bool {
                self.data == other.data
            }
        }
        pub struct Mat3 {
            data: [f32; 3 * 3],
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Mat3 {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Mat3",
                    "data",
                    &&self.data,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Mat3 {
            #[inline]
            fn clone(&self) -> Mat3 {
                let _: ::core::clone::AssertParamIsClone<[f32; 3 * 3]>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Mat3 {}
        #[automatically_derived]
        impl ::core::default::Default for Mat3 {
            #[inline]
            fn default() -> Mat3 {
                Mat3 {
                    data: ::core::default::Default::default(),
                }
            }
        }
        impl Mat3 {
            pub fn from_row(data: &[f32; 3 * 3]) -> Mat3 {
                Mat3 { data: data.clone() }
            }
            pub fn from_col(data: &[f32; 3 * 3]) -> Mat3 {
                let mut mat = Mat3::zeros();
                for x in 0..3 {
                    for y in 0..3 {
                        mat.set(x, y, data[y + 3 * x]);
                    }
                }
                mat
            }
            pub fn zeros() -> Mat3 {
                Mat3 { data: [0.; 3 * 3] }
            }
            pub fn ones() -> Mat3 {
                Mat3 { data: [1.; 3 * 3] }
            }
            pub fn identity() -> Mat3 {
                let mut mat = Mat3::zeros();
                for i in 0..3 {
                    mat.set(i, i, 1.0);
                }
                mat
            }
            pub fn get(&self, x: usize, y: usize) -> f32 {
                self.data[x + y * 3]
            }
            pub fn set(&mut self, x: usize, y: usize, value: f32) {
                self.data[x + y * 3] = value;
            }
            pub fn transpose(&self) -> Mat3 {
                let mut result = Mat3::identity();
                for x in 0..3 {
                    for y in 0..3 {
                        result.set(y, x, self.get(x, y));
                    }
                }
                result
            }
        }
        impl Mul for Mat3 {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let mut mat = Mat3::zeros();
                for y in 0..3 {
                    for x in 0..3 {
                        let mut sum = 0.0;
                        for d in 0..3 {
                            sum += self.get(d, y) * rhs.get(x, d);
                        }
                        mat.set(x, y, sum);
                    }
                }
                mat
            }
        }
        impl Mul<f32> for Mat3 {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self::Output {
                let mut mat = Mat3::zeros();
                for x in 0..3 {
                    for y in 0..3 {
                        mat.set(x, y, self.get(x, y) * rhs);
                    }
                }
                mat
            }
        }
        impl Div<f32> for Mat3 {
            type Output = Self;
            fn div(self, rhs: f32) -> Self::Output {
                self * (1.0 / rhs)
            }
        }
        impl PartialEq for Mat3 {
            fn eq(&self, other: &Self) -> bool {
                self.data == other.data
            }
        }
        pub struct Mat4 {
            data: [f32; 4 * 4],
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Mat4 {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Mat4",
                    "data",
                    &&self.data,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Mat4 {
            #[inline]
            fn clone(&self) -> Mat4 {
                let _: ::core::clone::AssertParamIsClone<[f32; 4 * 4]>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Mat4 {}
        #[automatically_derived]
        impl ::core::default::Default for Mat4 {
            #[inline]
            fn default() -> Mat4 {
                Mat4 {
                    data: ::core::default::Default::default(),
                }
            }
        }
        impl Mat4 {
            pub fn from_row(data: &[f32; 4 * 4]) -> Mat4 {
                Mat4 { data: data.clone() }
            }
            pub fn from_col(data: &[f32; 4 * 4]) -> Mat4 {
                let mut mat = Mat4::zeros();
                for x in 0..4 {
                    for y in 0..4 {
                        mat.set(x, y, data[y + 4 * x]);
                    }
                }
                mat
            }
            pub fn zeros() -> Mat4 {
                Mat4 { data: [0.; 4 * 4] }
            }
            pub fn ones() -> Mat4 {
                Mat4 { data: [1.; 4 * 4] }
            }
            pub fn identity() -> Mat4 {
                let mut mat = Mat4::zeros();
                for i in 0..4 {
                    mat.set(i, i, 1.0);
                }
                mat
            }
            pub fn get(&self, x: usize, y: usize) -> f32 {
                self.data[x + y * 4]
            }
            pub fn set(&mut self, x: usize, y: usize, value: f32) {
                self.data[x + y * 4] = value;
            }
            pub fn transpose(&self) -> Mat4 {
                let mut result = Mat4::identity();
                for x in 0..4 {
                    for y in 0..4 {
                        result.set(y, x, self.get(x, y));
                    }
                }
                result
            }
        }
        impl Mul for Mat4 {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let mut mat = Mat4::zeros();
                for y in 0..4 {
                    for x in 0..4 {
                        let mut sum = 0.0;
                        for d in 0..4 {
                            sum += self.get(d, y) * rhs.get(x, d);
                        }
                        mat.set(x, y, sum);
                    }
                }
                mat
            }
        }
        impl Mul<f32> for Mat4 {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self::Output {
                let mut mat = Mat4::zeros();
                for x in 0..4 {
                    for y in 0..4 {
                        mat.set(x, y, self.get(x, y) * rhs);
                    }
                }
                mat
            }
        }
        impl Div<f32> for Mat4 {
            type Output = Self;
            fn div(self, rhs: f32) -> Self::Output {
                self * (1.0 / rhs)
            }
        }
        impl PartialEq for Mat4 {
            fn eq(&self, other: &Self) -> bool {
                self.data == other.data
            }
        }
        impl Mul<Vec4> for Mat4 {
            type Output = Vec4;
            fn mul(self, rhs: Vec4) -> Self::Output {
                Vec4::new(
                    self.get(0, 0) * rhs.x + self.get(1, 0) * rhs.y
                        + self.get(2, 0) * rhs.z + self.get(3, 0) * rhs.w,
                    self.get(0, 1) * rhs.x + self.get(1, 1) * rhs.y
                        + self.get(2, 1) * rhs.z + self.get(3, 1) * rhs.w,
                    self.get(0, 2) * rhs.x + self.get(1, 2) * rhs.y
                        + self.get(2, 2) * rhs.z + self.get(3, 2) * rhs.w,
                    self.get(0, 3) * rhs.x + self.get(1, 3) * rhs.y
                        + self.get(2, 3) * rhs.z + self.get(3, 3) * rhs.w,
                )
            }
        }
        #[rustfmt::skip]
        pub fn apply_translate(offset: &Vec3) -> Mat4 {
            Mat4::from_row(
                &[
                    1.0,
                    0.0,
                    0.0,
                    offset.x,
                    0.0,
                    1.0,
                    0.0,
                    offset.y,
                    0.0,
                    0.0,
                    1.0,
                    offset.z,
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                ],
            )
        }
        #[rustfmt::skip]
        pub fn apply_eular_rotate_y(angle: f32) -> Mat4 {
            let c = angle.cos();
            let s = angle.sin();
            Mat4::from_row(
                &[
                    c,
                    0.0,
                    s,
                    0.0,
                    0.0,
                    1.0,
                    0.0,
                    0.0,
                    -s,
                    0.0,
                    c,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                ],
            )
        }
    }
    mod vec {
        use std::default::Default;
        use std::ops::{
            Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
        };
        pub struct Vec2 {
            pub x: f32,
            pub y: f32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Vec2 {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Vec2",
                    "x",
                    &self.x,
                    "y",
                    &&self.y,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Vec2 {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Vec2 {
            #[inline]
            fn eq(&self, other: &Vec2) -> bool {
                self.x == other.x && self.y == other.y
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Vec2 {}
        #[automatically_derived]
        impl ::core::clone::Clone for Vec2 {
            #[inline]
            fn clone(&self) -> Vec2 {
                let _: ::core::clone::AssertParamIsClone<f32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Vec2 {
            #[inline]
            fn default() -> Vec2 {
                Vec2 {
                    x: ::core::default::Default::default(),
                    y: ::core::default::Default::default(),
                }
            }
        }
        impl Vec2 {
            pub const fn new(x: f32, y: f32) -> Vec2 {
                Vec2 { x, y }
            }
            pub fn zero() -> Vec2 {
                Vec2 { x: 0f32, y: 0f32 }
            }
            pub fn length_square(&self) -> f32 {
                self.x * self.x + self.y * self.y + 0.0
            }
            pub fn length(&self) -> f32 {
                self.length_square().sqrt()
            }
            pub fn normalize(&self) -> Vec2 {
                *self / self.length()
            }
        }
        impl Neg for Vec2 {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.x, -self.y)
            }
        }
        impl Add for Vec2 {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Vec2 {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }
        impl Add<f32> for Vec2 {
            type Output = Self;
            fn add(self, rhs: f32) -> Self::Output {
                Vec2 {
                    x: self.x + rhs,
                    y: self.y + rhs,
                }
            }
        }
        impl Sub for Vec2 {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Vec2 {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }
        impl Sub<f32> for Vec2 {
            type Output = Self;
            fn sub(self, rhs: f32) -> Self::Output {
                Vec2 {
                    x: self.x - rhs,
                    y: self.y - rhs,
                }
            }
        }
        impl Mul for Vec2 {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Vec2 {
                    x: self.x * rhs.x,
                    y: self.y * rhs.y,
                }
            }
        }
        impl Mul<f32> for Vec2 {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self::Output {
                Vec2 {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }
        impl Div for Vec2 {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                Vec2 {
                    x: self.x / rhs.x,
                    y: self.y / rhs.y,
                }
            }
        }
        impl Div<f32> for Vec2 {
            type Output = Self;
            fn div(self, rhs: f32) -> Self::Output {
                Vec2 {
                    x: self.x / rhs,
                    y: self.y / rhs,
                }
            }
        }
        impl AddAssign for Vec2 {
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }
        impl AddAssign<f32> for Vec2 {
            fn add_assign(&mut self, rhs: f32) {
                self.x += rhs;
                self.y += rhs;
            }
        }
        impl SubAssign for Vec2 {
            fn sub_assign(&mut self, rhs: Self) {
                self.x -= rhs.x;
                self.y -= rhs.y;
            }
        }
        impl SubAssign<f32> for Vec2 {
            fn sub_assign(&mut self, rhs: f32) {
                self.x -= rhs;
                self.y -= rhs;
            }
        }
        impl MulAssign for Vec2 {
            fn mul_assign(&mut self, rhs: Self) {
                self.x *= rhs.x;
                self.y *= rhs.y;
            }
        }
        impl MulAssign<f32> for Vec2 {
            fn mul_assign(&mut self, rhs: f32) {
                self.x *= rhs;
                self.y *= rhs;
            }
        }
        impl DivAssign for Vec2 {
            fn div_assign(&mut self, rhs: Self) {
                self.x /= rhs.x;
                self.y /= rhs.y;
            }
        }
        impl DivAssign<f32> for Vec2 {
            fn div_assign(&mut self, rhs: f32) {
                self.x /= rhs;
                self.y /= rhs;
            }
        }
        pub struct Vec3 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Vec3 {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Vec3",
                    "x",
                    &self.x,
                    "y",
                    &self.y,
                    "z",
                    &&self.z,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Vec3 {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Vec3 {
            #[inline]
            fn eq(&self, other: &Vec3) -> bool {
                self.x == other.x && self.y == other.y && self.z == other.z
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Vec3 {}
        #[automatically_derived]
        impl ::core::clone::Clone for Vec3 {
            #[inline]
            fn clone(&self) -> Vec3 {
                let _: ::core::clone::AssertParamIsClone<f32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Vec3 {
            #[inline]
            fn default() -> Vec3 {
                Vec3 {
                    x: ::core::default::Default::default(),
                    y: ::core::default::Default::default(),
                    z: ::core::default::Default::default(),
                }
            }
        }
        impl Vec3 {
            pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
                Vec3 { x, y, z }
            }
            pub fn zero() -> Vec3 {
                Vec3 { x: 0f32, y: 0f32, z: 0f32 }
            }
            pub fn length_square(&self) -> f32 {
                self.x * self.x + self.y * self.y + self.z * self.z + 0.0
            }
            pub fn length(&self) -> f32 {
                self.length_square().sqrt()
            }
            pub fn normalize(&self) -> Vec3 {
                *self / self.length()
            }
        }
        impl Neg for Vec3 {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.x, -self.y, -self.z)
            }
        }
        impl Add for Vec3 {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Vec3 {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
            }
        }
        impl Add<f32> for Vec3 {
            type Output = Self;
            fn add(self, rhs: f32) -> Self::Output {
                Vec3 {
                    x: self.x + rhs,
                    y: self.y + rhs,
                    z: self.z + rhs,
                }
            }
        }
        impl Sub for Vec3 {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Vec3 {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                }
            }
        }
        impl Sub<f32> for Vec3 {
            type Output = Self;
            fn sub(self, rhs: f32) -> Self::Output {
                Vec3 {
                    x: self.x - rhs,
                    y: self.y - rhs,
                    z: self.z - rhs,
                }
            }
        }
        impl Mul for Vec3 {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Vec3 {
                    x: self.x * rhs.x,
                    y: self.y * rhs.y,
                    z: self.z * rhs.z,
                }
            }
        }
        impl Mul<f32> for Vec3 {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self::Output {
                Vec3 {
                    x: self.x * rhs,
                    y: self.y * rhs,
                    z: self.z * rhs,
                }
            }
        }
        impl Div for Vec3 {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                Vec3 {
                    x: self.x / rhs.x,
                    y: self.y / rhs.y,
                    z: self.z / rhs.z,
                }
            }
        }
        impl Div<f32> for Vec3 {
            type Output = Self;
            fn div(self, rhs: f32) -> Self::Output {
                Vec3 {
                    x: self.x / rhs,
                    y: self.y / rhs,
                    z: self.z / rhs,
                }
            }
        }
        impl AddAssign for Vec3 {
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
            }
        }
        impl AddAssign<f32> for Vec3 {
            fn add_assign(&mut self, rhs: f32) {
                self.x += rhs;
                self.y += rhs;
                self.z += rhs;
            }
        }
        impl SubAssign for Vec3 {
            fn sub_assign(&mut self, rhs: Self) {
                self.x -= rhs.x;
                self.y -= rhs.y;
                self.z -= rhs.z;
            }
        }
        impl SubAssign<f32> for Vec3 {
            fn sub_assign(&mut self, rhs: f32) {
                self.x -= rhs;
                self.y -= rhs;
                self.z -= rhs;
            }
        }
        impl MulAssign for Vec3 {
            fn mul_assign(&mut self, rhs: Self) {
                self.x *= rhs.x;
                self.y *= rhs.y;
                self.z *= rhs.z;
            }
        }
        impl MulAssign<f32> for Vec3 {
            fn mul_assign(&mut self, rhs: f32) {
                self.x *= rhs;
                self.y *= rhs;
                self.z *= rhs;
            }
        }
        impl DivAssign for Vec3 {
            fn div_assign(&mut self, rhs: Self) {
                self.x /= rhs.x;
                self.y /= rhs.y;
                self.z /= rhs.z;
            }
        }
        impl DivAssign<f32> for Vec3 {
            fn div_assign(&mut self, rhs: f32) {
                self.x /= rhs;
                self.y /= rhs;
                self.z /= rhs;
            }
        }
        pub struct Vec4 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub w: f32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Vec4 {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Vec4",
                    "x",
                    &self.x,
                    "y",
                    &self.y,
                    "z",
                    &self.z,
                    "w",
                    &&self.w,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Vec4 {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Vec4 {
            #[inline]
            fn eq(&self, other: &Vec4) -> bool {
                self.x == other.x && self.y == other.y && self.z == other.z
                    && self.w == other.w
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Vec4 {}
        #[automatically_derived]
        impl ::core::clone::Clone for Vec4 {
            #[inline]
            fn clone(&self) -> Vec4 {
                let _: ::core::clone::AssertParamIsClone<f32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Vec4 {
            #[inline]
            fn default() -> Vec4 {
                Vec4 {
                    x: ::core::default::Default::default(),
                    y: ::core::default::Default::default(),
                    z: ::core::default::Default::default(),
                    w: ::core::default::Default::default(),
                }
            }
        }
        impl Vec4 {
            pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
                Vec4 { x, y, z, w }
            }
            pub fn zero() -> Vec4 {
                Vec4 {
                    x: 0f32,
                    y: 0f32,
                    z: 0f32,
                    w: 0f32,
                }
            }
            pub fn length_square(&self) -> f32 {
                self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
                    + 0.0
            }
            pub fn length(&self) -> f32 {
                self.length_square().sqrt()
            }
            pub fn normalize(&self) -> Vec4 {
                *self / self.length()
            }
        }
        impl Neg for Vec4 {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.x, -self.y, -self.z, -self.w)
            }
        }
        impl Add for Vec4 {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Vec4 {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                    w: self.w + rhs.w,
                }
            }
        }
        impl Add<f32> for Vec4 {
            type Output = Self;
            fn add(self, rhs: f32) -> Self::Output {
                Vec4 {
                    x: self.x + rhs,
                    y: self.y + rhs,
                    z: self.z + rhs,
                    w: self.w + rhs,
                }
            }
        }
        impl Sub for Vec4 {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Vec4 {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                    w: self.w - rhs.w,
                }
            }
        }
        impl Sub<f32> for Vec4 {
            type Output = Self;
            fn sub(self, rhs: f32) -> Self::Output {
                Vec4 {
                    x: self.x - rhs,
                    y: self.y - rhs,
                    z: self.z - rhs,
                    w: self.w - rhs,
                }
            }
        }
        impl Mul for Vec4 {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Vec4 {
                    x: self.x * rhs.x,
                    y: self.y * rhs.y,
                    z: self.z * rhs.z,
                    w: self.w * rhs.w,
                }
            }
        }
        impl Mul<f32> for Vec4 {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self::Output {
                Vec4 {
                    x: self.x * rhs,
                    y: self.y * rhs,
                    z: self.z * rhs,
                    w: self.w * rhs,
                }
            }
        }
        impl Div for Vec4 {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                Vec4 {
                    x: self.x / rhs.x,
                    y: self.y / rhs.y,
                    z: self.z / rhs.z,
                    w: self.w / rhs.w,
                }
            }
        }
        impl Div<f32> for Vec4 {
            type Output = Self;
            fn div(self, rhs: f32) -> Self::Output {
                Vec4 {
                    x: self.x / rhs,
                    y: self.y / rhs,
                    z: self.z / rhs,
                    w: self.w / rhs,
                }
            }
        }
        impl AddAssign for Vec4 {
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
                self.w += rhs.w;
            }
        }
        impl AddAssign<f32> for Vec4 {
            fn add_assign(&mut self, rhs: f32) {
                self.x += rhs;
                self.y += rhs;
                self.z += rhs;
                self.w += rhs;
            }
        }
        impl SubAssign for Vec4 {
            fn sub_assign(&mut self, rhs: Self) {
                self.x -= rhs.x;
                self.y -= rhs.y;
                self.z -= rhs.z;
                self.w -= rhs.w;
            }
        }
        impl SubAssign<f32> for Vec4 {
            fn sub_assign(&mut self, rhs: f32) {
                self.x -= rhs;
                self.y -= rhs;
                self.z -= rhs;
                self.w -= rhs;
            }
        }
        impl MulAssign for Vec4 {
            fn mul_assign(&mut self, rhs: Self) {
                self.x *= rhs.x;
                self.y *= rhs.y;
                self.z *= rhs.z;
                self.w *= rhs.w;
            }
        }
        impl MulAssign<f32> for Vec4 {
            fn mul_assign(&mut self, rhs: f32) {
                self.x *= rhs;
                self.y *= rhs;
                self.z *= rhs;
                self.w *= rhs;
            }
        }
        impl DivAssign for Vec4 {
            fn div_assign(&mut self, rhs: Self) {
                self.x /= rhs.x;
                self.y /= rhs.y;
                self.z /= rhs.z;
                self.w /= rhs.w;
            }
        }
        impl DivAssign<f32> for Vec4 {
            fn div_assign(&mut self, rhs: f32) {
                self.x /= rhs;
                self.y /= rhs;
                self.z /= rhs;
                self.w /= rhs;
            }
        }
        impl Vec4 {
            pub fn from_vec3(v: &Vec3, w: f32) -> Vec4 {
                Self { x: v.x, y: v.y, z: v.z, w }
            }
            pub fn truncated_to_vec3(&self) -> Vec3 {
                Vec3 {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                }
            }
            pub fn truncated_to_vec2(&self) -> Vec2 {
                Vec2 { x: self.x, y: self.y }
            }
        }
        pub fn lerp<T>(a: T, b: T, t: f32) -> T
        where
            T: Sub<Output = T> + Add<Output = T> + Mul<f32, Output = T> + Copy + Clone,
        {
            a + (b - a) * t
        }
    }
    pub use self::mat::*;
    pub use self::vec::*;
}
mod cohen_sutherland {
    use crate::math::Vec2;
    const INSIDE: u8 = 0b0000;
    const LEFT: u8 = 0b0001;
    const RIGHT: u8 = 0b0010;
    const BOTTOM: u8 = 0b0100;
    const TOP: u8 = 0b1000;
    fn get_outcode(p: &Vec2, min: &Vec2, max: &Vec2) -> u8 {
        (if p.x < min.x { LEFT } else if p.x > max.x { RIGHT } else { INSIDE }
            | if p.y < min.y { BOTTOM } else if p.y > max.y { TOP } else { INSIDE })
    }
    pub fn clip(
        p1: &Vec2,
        p2: &Vec2,
        rect_min: &Vec2,
        rect_max: &Vec2,
    ) -> Option<(Vec2, Vec2)> {
        let mut pt1 = *p1;
        let mut pt2 = *p2;
        let mut outcode1 = get_outcode(&pt1, rect_min, rect_max);
        let mut outcode2 = get_outcode(&pt2, rect_min, rect_max);
        loop {
            if outcode1 & outcode2 != 0 {
                return None;
            } else if (outcode1 | outcode2) == 0 {
                return Some((pt1, pt2));
            }
            let mut p = Vec2 { x: 0.0, y: 0.0 };
            let outcode = if outcode2 > outcode1 { outcode2 } else { outcode1 };
            if outcode & TOP != 0 {
                p.x = p1.x + (pt2.x - pt1.x) * (rect_max.y - pt1.y) / (pt2.y - pt1.y);
                p.y = rect_max.y;
            } else if outcode & BOTTOM != 0 {
                p.x = p1.x + (pt2.x - pt1.x) * (rect_min.y - pt1.y) / (pt2.y - pt1.y);
                p.y = rect_min.y;
            } else if outcode & RIGHT != 0 {
                p.y = pt1.y + (pt2.y - pt1.y) * (rect_max.x - pt1.x) / (pt2.x - pt1.x);
                p.x = rect_max.x;
            } else if outcode & LEFT != 0 {
                p.y = pt1.y + (pt2.y - pt1.y) * (rect_min.x - pt1.x) / (pt2.x - pt1.x);
                p.x = rect_min.x;
            }
            if outcode == outcode1 {
                pt1 = p;
                outcode1 = get_outcode(&pt1, rect_min, rect_max);
            } else {
                pt2 = p;
                outcode2 = get_outcode(&pt2, rect_min, rect_max);
            }
        }
    }
}
pub mod renderer {
    use crate::{
        math::{Mat4, Vec2, Vec4},
        texture::Texture, vertex::Vertex,
    };
    pub struct Viewport {
        pub x: i32,
        pub y: i32,
        pub w: u32,
        pub h: u32,
    }
    pub trait RendererInterface {
        fn clear(&mut self, color: &Vec4);
        fn get_canvas_width(&self) -> u32;
        fn get_canvas_height(&self) -> u32;
        fn get_frame_image(&self) -> &[u8];
        fn draw_triangle(
            &mut self,
            model: &Mat4,
            vertices: &[Vertex],
            count: u32,
            texture: Option<&Texture>,
        );
    }
    pub fn texture_sample(texture: &Texture, textcoord: &Vec2) -> Vec4 {
        Vec4::zero()
    }
}
pub mod camera {
    use crate::math::Mat4;
    pub struct Frustum {
        near: f32,
        aspect: f32,
        fov: f32,
        mat: Mat4,
    }
    impl Frustum {
        #[rustfmt::skip]
        pub fn new(near: f32, far: f32, aspect: f32, fov: f32) -> Frustum {
            let a = 1.0 / (near * fov.tan());
            Self {
                near,
                aspect,
                fov,
                mat: if false {
                    Mat4::from_row(
                        &[
                            a,
                            0.0,
                            0.0,
                            0.0,
                            0.0,
                            aspect * a,
                            0.0,
                            0.0,
                            0.0,
                            0.0,
                            1.0,
                            0.0,
                            0.0,
                            0.0,
                            -1.0 / near,
                            0.0,
                        ],
                    )
                } else {
                    let half_w = near * fov.tan();
                    let half_h = half_w / aspect;
                    let near = near.abs();
                    let far = far.abs();
                    Mat4::from_row(
                        &[
                            a,
                            0.0,
                            0.0,
                            0.0,
                            0.0,
                            aspect * a,
                            0.0,
                            0.0,
                            0.0,
                            0.0,
                            1.0,
                            0.0,
                            0.0,
                            0.0,
                            -1.0 / near,
                            0.0,
                        ],
                    )
                },
            }
        }
        pub fn get_mat(&self) -> &Mat4 {
            &self.mat
        }
        pub fn near(&self) -> f32 {
            self.near
        }
    }
    pub struct Camera {
        frustum: Frustum,
    }
    impl Camera {
        pub fn new(near: f32, far: f32, aspect: f32, fov: f32) -> Self {
            Self {
                frustum: Frustum::new(near, far, aspect, fov),
            }
        }
        pub fn get_frustum(&self) -> &Frustum {
            &self.frustum
        }
    }
}
mod image {
    use crate::math;
    pub struct PureElemImage<T> {
        data: Vec<T>,
        w: u32,
        h: u32,
    }
    impl<T> PureElemImage<T> {
        pub fn width(&self) -> u32 {
            self.w
        }
        pub fn height(&self) -> u32 {
            self.h
        }
        pub fn in_box(&self, x: i32, y: i32) -> bool {
            x >= 0 && x < self.w.try_into().unwrap() && y >= 0
                && y < self.h.try_into().unwrap()
        }
        pub fn data(&self) -> &Vec<T> {
            &self.data
        }
    }
    impl PureElemImage<u8> {
        pub fn new(w: u32, h: u32) -> Self {
            Self {
                data: ::alloc::vec::from_elem(0, (w * h * 3) as usize),
                w,
                h,
            }
        }
        pub fn clear(&mut self, color: &math::Vec4) {
            for x in 0..self.w {
                for y in 0..self.h {
                    self.set(x, y, color);
                }
            }
        }
        pub fn set(&mut self, x: u32, y: u32, color: &math::Vec4) {
            self.data[(x + y * self.w) as usize * 3] = (color.x * 255.0) as u8;
            self.data[(x + y * self.w) as usize * 3 + 1] = (color.y * 255.0) as u8;
            self.data[(x + y * self.w) as usize * 3 + 2] = (color.z * 255.0) as u8;
        }
    }
    impl PureElemImage<f32> {
        pub fn new(w: u32, h: u32) -> Self {
            Self {
                w,
                h,
                data: ::alloc::vec::from_elem(std::f32::MAX, (w * h) as usize),
            }
        }
        pub fn clear(&mut self, value: f32) {
            self.data.fill(value)
        }
        pub fn set(&mut self, x: u32, y: u32, value: f32) {
            self.data[(x + y * self.w) as usize] = value;
        }
    }
    pub type ColorAttachment = PureElemImage<u8>;
}
mod scanline {
    use crate::vertex::{self, interp_attributes, Vertex};
    pub struct Edge {
        pub v1: Vertex,
        pub v2: Vertex,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Edge {
        #[inline]
        fn clone(&self) -> Edge {
            let _: ::core::clone::AssertParamIsClone<Vertex>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Edge {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Edge {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Edge",
                "v1",
                &self.v1,
                "v2",
                &&self.v2,
            )
        }
    }
    pub struct Trapezoid {
        pub top: f32,
        pub bottom: f32,
        pub left: Edge,
        pub right: Edge,
    }
    impl Trapezoid {
        fn get_hang_trap(vertices: &[Vertex; 3]) -> Self {
            Trapezoid {
                top: vertices[0].position.y,
                bottom: vertices[2].position.y,
                left: Edge {
                    v1: vertices[0],
                    v2: vertices[2],
                },
                right: Edge {
                    v1: vertices[1],
                    v2: vertices[2],
                },
            }
        }
        fn get_portrait_trap(vertices: &[Vertex; 3]) -> Self {
            Trapezoid {
                top: vertices[0].position.y,
                bottom: vertices[1].position.y,
                left: Edge {
                    v1: vertices[0],
                    v2: vertices[1],
                },
                right: Edge {
                    v1: vertices[0],
                    v2: vertices[2],
                },
            }
        }
        pub fn from_triangle(vertices: &[Vertex; 3]) -> [Option<Self>; 2] {
            let mut vertices = *vertices;
            vertices.sort_by(|a, b| a.position.y.partial_cmp(&b.position.y).unwrap());
            if (vertices[0].position.x == vertices[1].position.x
                && vertices[0].position.x == vertices[2].position.x)
                || (vertices[0].position.y == vertices[1].position.y
                    && vertices[0].position.y == vertices[2].position.y)
            {
                return [None, None];
            }
            if vertices[0].position.y == vertices[1].position.y {
                if vertices[0].position.x > vertices[1].position.x {
                    vertices.swap(0, 1);
                }
                let trap = Self::get_hang_trap(&vertices);
                return [Some(trap), None];
            }
            if vertices[1].position.y == vertices[2].position.y {
                if vertices[1].position.x > vertices[2].position.x {
                    vertices.swap(1, 2);
                }
                let trap = Self::get_portrait_trap(&vertices);
                return [Some(trap), None];
            }
            let k = (vertices[2].position.y - vertices[0].position.y)
                / (vertices[2].position.x - vertices[0].position.x);
            let dx = (vertices[1].position.y - vertices[0].position.y) / k
                + vertices[0].position.x;
            let t = (vertices[2].position.x - dx)
                / (vertices[2].position.x - vertices[0].position.x);
            let d_vertex = vertex::lerp_vertex(&vertices[0], &vertices[1], t);
            if dx > vertices[1].position.x {
                let trap1 = Self::get_portrait_trap(
                    &[vertices[0], vertices[1], d_vertex],
                );
                let trap2 = Self::get_hang_trap(&[vertices[1], d_vertex, vertices[2]]);
                return [Some(trap1), Some(trap2)];
            } else {
                let trap1 = Self::get_portrait_trap(
                    &[vertices[0], d_vertex, vertices[1]],
                );
                let trap2 = Self::get_hang_trap(&[d_vertex, vertices[1], vertices[2]]);
                return [Some(trap1), Some(trap2)];
            }
        }
    }
    pub struct Scanline {
        pub vertex: Vertex,
        pub step: Vertex,
        pub y: f32,
        pub width: f32,
    }
    impl Scanline {
        pub fn from_trapezoid(trap: &Trapezoid, init_y: f32) -> Self {
            let t1 = (init_y - trap.left.v1.position.y)
                / (trap.left.v2.position.y - trap.left.v1.position.y);
            let t2 = (init_y - trap.right.v1.position.y)
                / (trap.right.v2.position.y - trap.right.v1.position.y);
            let vertex_left = vertex::lerp_vertex(&trap.left.v1, &trap.left.v2, t1);
            let vertex_right = vertex::lerp_vertex(&trap.right.v1, &trap.right.v2, t2);
            let width = vertex_right.position.x - vertex_left.position.x;
            let rh_width = 1.0 / width;
            let position_step = (vertex_right.position - vertex_left.position)
                * rh_width;
            let attribute_step = interp_attributes(
                &vertex_right.attributes,
                &vertex_left.attributes,
                |v1, v2, t| (v2 - v1) * t,
                rh_width,
            );
            Self {
                vertex: vertex_left,
                step: Vertex {
                    position: position_step,
                    attributes: attribute_step,
                },
                y: init_y,
                width,
            }
        }
    }
}
pub mod cpu_renderer {
    use crate::{
        bresenham_line, camera::Camera, cohen_sutherland, image::ColorAttachment,
        math::{Mat4, Vec2, Vec4},
        renderer::{self, RendererInterface, Viewport},
        scanline, texture::Texture, vertex::{self, attributes_foreach, Vertex},
    };
    pub struct Renderer {
        color_attachment: ColorAttachment,
        viewport: Viewport,
        camera: Camera,
    }
    impl RendererInterface for Renderer {
        fn clear(&mut self, color: &Vec4) {
            self.color_attachment.clear(color)
        }
        fn get_canvas_width(&self) -> u32 {
            self.color_attachment.width()
        }
        fn get_canvas_height(&self) -> u32 {
            self.color_attachment.height()
        }
        fn get_frame_image(&self) -> &[u8] {
            self.color_attachment.data()
        }
        fn draw_triangle(
            &mut self,
            model: &Mat4,
            vertices: &[Vertex],
            count: u32,
            texture: Option<&Texture>,
        ) {
            for i in 0..count {
                let index = (i * 3) as usize;
                let mut vertices = [
                    vertices[index],
                    vertices[index + 1],
                    vertices[index + 2],
                ];
                for v in &mut vertices {
                    v.position = *model * v.position;
                }
                for v in &mut vertices {
                    v.position = *self.camera.get_frustum().get_mat() * v.position;
                }
                for v in &mut vertices {
                    v.position.z = -v.position.w * self.camera.get_frustum().near();
                }
                for v in &mut vertices {
                    v.position.y /= v.position.w;
                    v.position.x /= v.position.w;
                    v.position.w = 1.0;
                }
                for v in &mut vertices {
                    v.position.x = (v.position.x + 1.0) * 0.5
                        * (self.viewport.w as f32 - 1.0) + self.viewport.x as f32;
                    v.position.y = (v.position.y + 1.0) * 0.5
                        * (self.viewport.w as f32 - 1.0) + self.viewport.x as f32;
                }
                let [trap1, trap2] = &mut scanline::Trapezoid::from_triangle(&vertices);
                if let Some(trap) = trap1 {
                    self.draw_trapezoid(trap, texture);
                }
                if let Some(trap) = trap2 {
                    self.draw_trapezoid(trap, texture);
                }
            }
        }
    }
    impl Renderer {
        pub fn new(w: u32, h: u32, camera: Camera) -> Self {
            Self {
                camera,
                viewport: Viewport { x: 0, y: 0, w, h },
                color_attachment: ColorAttachment::new(w, h),
            }
        }
        fn draw_line(&mut self, p0: &Vec2, p1: &Vec2, color: &Vec4) {
            let rect_min = Vec2::zero();
            let rect_max = Vec2::new(
                self.color_attachment.width() as f32 - 1.0,
                self.color_attachment.height() as f32 - 1.0,
            );
            let res = cohen_sutherland::clip(&p0, &p1, &rect_min, &rect_max);
            match res {
                Some((next_p0, next_p1)) => {
                    bresenham_line::draw_line(
                        next_p0.x as i32,
                        next_p0.y as i32,
                        next_p1.x as i32,
                        next_p1.y as i32,
                        color,
                        &mut self.color_attachment,
                    )
                }
                None => {}
            }
        }
        pub fn draw_trapezoid(
            &mut self,
            trap: &mut scanline::Trapezoid,
            texture: Option<&Texture>,
        ) {
            let top = trap.top.ceil().max(0.0) as i32;
            let bottom = trap
                .bottom
                .ceil()
                .min(self.color_attachment.height() as f32 - 1.0) as i32 - 1;
            let mut y = top as f32;
            vertex::vertex_rhw_init(&mut trap.left.v1);
            vertex::vertex_rhw_init(&mut trap.left.v2);
            vertex::vertex_rhw_init(&mut trap.right.v1);
            vertex::vertex_rhw_init(&mut trap.right.v2);
            while y <= bottom as f32 {
                let mut scanline = scanline::Scanline::from_trapezoid(&trap, y);
                self.draw_scanline(&scanline, texture);
                y += 1.0;
            }
        }
        pub fn draw_scanline(
            &mut self,
            scanline: &scanline::Scanline,
            texture: Option<&Texture>,
        ) {
            let mut vertex = scanline.vertex;
            let y: u32 = scanline.y as u32;
            let mut width = scanline.width;
            let border = self.color_attachment.width() as f32;
            while width > 0.0 {
                let x = &vertex.position.x;
                let rhw = vertex.position.z;
                if *x >= 0.0 && *x < border {
                    let mut attr_local = vertex.attributes;
                    attributes_foreach(&mut attr_local, |v| v / rhw);
                    let textcoord = attr_local.vec2[1];
                    let color = attr_local.vec4[0]
                        * match texture {
                            Some(texture) => {
                                renderer::texture_sample(&texture, &textcoord)
                            }
                            None => Vec4::new(1.0, 1.0, 1.0, 1.0),
                        };
                    self.color_attachment.set(*x as u32, y, &color);
                }
                width -= 1.0;
                vertex.position += scanline.step.position;
                vertex.attributes = vertex::interp_attributes(
                    &vertex.attributes,
                    &scanline.step.attributes,
                    |v1, v2, _| v1 + v2,
                    0.0,
                );
            }
        }
    }
}
mod vertex {
    use crate::math::{lerp, Vec2, Vec3, Vec4};
    const ATTR_NUM: usize = 4;
    pub struct Attributes {
        pub float: [f32; ATTR_NUM],
        pub vec2: [Vec2; ATTR_NUM],
        pub vec3: [Vec3; ATTR_NUM],
        pub vec4: [Vec4; ATTR_NUM],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Attributes {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Attributes",
                "float",
                &self.float,
                "vec2",
                &self.vec2,
                "vec3",
                &self.vec3,
                "vec4",
                &&self.vec4,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Attributes {
        #[inline]
        fn clone(&self) -> Attributes {
            let _: ::core::clone::AssertParamIsClone<[f32; ATTR_NUM]>;
            let _: ::core::clone::AssertParamIsClone<[Vec2; ATTR_NUM]>;
            let _: ::core::clone::AssertParamIsClone<[Vec3; ATTR_NUM]>;
            let _: ::core::clone::AssertParamIsClone<[Vec4; ATTR_NUM]>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Attributes {}
    impl Default for Attributes {
        fn default() -> Self {
            Self {
                float: [0.0; ATTR_NUM],
                vec2: [Vec2::zero(); ATTR_NUM],
                vec3: [Vec3::zero(); ATTR_NUM],
                vec4: [Vec4::zero(); ATTR_NUM],
            }
        }
    }
    impl Attributes {
        pub fn set_float(&mut self, location: usize, value: f32) {
            self.float[location] = value;
        }
        pub fn set_vec2(&mut self, location: usize, value: Vec2) {
            self.vec2[location] = value;
        }
        pub fn set_vec3(&mut self, location: usize, value: Vec3) {
            self.vec3[location] = value;
        }
        pub fn set_vec4(&mut self, location: usize, value: Vec4) {
            self.vec4[location] = value;
        }
    }
    pub struct Vertex {
        pub position: Vec4,
        pub attributes: Attributes,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Vertex {
        #[inline]
        fn clone(&self) -> Vertex {
            let _: ::core::clone::AssertParamIsClone<Vec4>;
            let _: ::core::clone::AssertParamIsClone<Attributes>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Vertex {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Vertex {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Vertex",
                "position",
                &self.position,
                "attributes",
                &&self.attributes,
            )
        }
    }
    impl Vertex {
        pub fn new(position: &Vec3, attributes: Attributes) -> Self {
            Self {
                position: Vec4::from_vec3(position, 1.0),
                attributes,
            }
        }
        pub fn truncated_to_vec2(&self) -> Vec2 {
            self.position.truncated_to_vec2()
        }
    }
    pub fn vertex_rhw_init(vertex: &mut Vertex) {
        let rhw_z = 1.0 / vertex.position.z;
        vertex.position.z = rhw_z;
        attributes_foreach(&mut vertex.attributes, |v| v * rhw_z);
    }
    pub fn lerp_vertex(start: &Vertex, end: &Vertex, t: f32) -> Vertex {
        let position = start.position + (end.position - start.position) * t;
        let attributes = interp_attributes(&start.attributes, &end.attributes, lerp, t);
        Vertex { position, attributes }
    }
    pub fn interp_attributes<F>(
        attr1: &Attributes,
        attr2: &Attributes,
        f: F,
        t: f32,
    ) -> Attributes
    where
        F: Fn(f32, f32, f32) -> f32,
    {
        let mut attributes = Attributes::default();
        for index in 0..ATTR_NUM {
            attributes.set_float(index, f(attr1.float[index], attr2.float[index], t));
        }
        for index in 0..ATTR_NUM {
            let value1 = attr1.vec2[index];
            let value2 = attr2.vec2[index];
            attributes
                .set_vec2(
                    index,
                    Vec2::new(f(value1.x, value2.x, t), f(value1.y, value2.y, t)),
                );
        }
        for index in 0..ATTR_NUM {
            let value1 = attr1.vec3[index];
            let value2 = attr2.vec3[index];
            attributes
                .set_vec3(
                    index,
                    Vec3::new(
                        f(value1.x, value2.x, t),
                        f(value1.y, value2.y, t),
                        f(value1.z, value2.z, t),
                    ),
                );
        }
        for index in 0..ATTR_NUM {
            let value1 = attr1.vec4[index];
            let value2 = attr2.vec4[index];
            attributes
                .set_vec4(
                    index,
                    Vec4::new(
                        f(value1.x, value2.x, t),
                        f(value1.y, value2.y, t),
                        f(value1.z, value2.z, t),
                        f(value1.w, value2.w, t),
                    ),
                );
        }
        attributes
    }
    pub fn attributes_foreach<F>(attr: &mut Attributes, f: F)
    where
        F: Fn(f32) -> f32,
    {
        for index in 0..ATTR_NUM {
            attr.set_float(index, f(attr.float[index]));
        }
        for index in 0..ATTR_NUM {
            let value = attr.vec2[index];
            attr.set_vec2(index, Vec2::new(f(value.x), f(value.y)));
        }
        for index in 0..ATTR_NUM {
            let value = attr.vec3[index];
            attr.set_vec3(index, Vec3::new(f(value.x), f(value.y), f(value.z)));
        }
        for index in 0..ATTR_NUM {
            let value = attr.vec4[index];
            attr.set_vec4(
                index,
                Vec4::new(f(value.x), f(value.y), f(value.z), f(value.w)),
            );
        }
    }
}
pub mod texture {
    use std::collections::HashMap;
    use image::{self, GenericImageView};
    use crate::math::Vec4;
    type A = image::Rgba<u16>;
    pub struct Texture {
        image: image::DynamicImage,
        id: u32,
        name: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Texture {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Texture",
                "image",
                &self.image,
                "id",
                &self.id,
                "name",
                &&self.name,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Texture {
        #[inline]
        fn default() -> Texture {
            Texture {
                image: ::core::default::Default::default(),
                id: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
            }
        }
    }
    impl Texture {
        pub fn load(filename: &str, id: u32, name: &str) -> image::ImageResult<Texture> {
            let img = image::open(filename)?;
            Ok(Self {
                image: img,
                id,
                name: name.to_string(),
            })
        }
        fn get_pixel(&self, x: u32, y: u32) -> Vec4 {
            let pixel = self.image.get_pixel(x, y);
            let data = pixel.0;
            Vec4::new(
                data[0] as f32 / 255.0,
                data[1] as f32 / 255.0,
                data[2] as f32 / 255.0,
                data[3] as f32 / 255.0,
            )
        }
    }
    pub struct TextureStore {
        auto_incre_id: u32,
        images: HashMap<u32, Texture>,
        name_id_map: HashMap<String, u32>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TextureStore {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "TextureStore",
                "auto_incre_id",
                &self.auto_incre_id,
                "images",
                &self.images,
                "name_id_map",
                &&self.name_id_map,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for TextureStore {
        #[inline]
        fn default() -> TextureStore {
            TextureStore {
                auto_incre_id: ::core::default::Default::default(),
                images: ::core::default::Default::default(),
                name_id_map: ::core::default::Default::default(),
            }
        }
    }
    impl TextureStore {
        pub fn load(&mut self, filename: &str, name: &str) -> image::ImageResult<u32> {
            let id = self.auto_incre_id;
            self.images.insert(id, Texture::load(filename, id, name)?);
            self.name_id_map.insert(name.to_string(), id);
            self.auto_incre_id += 1;
            Ok(id)
        }
        pub fn get_by_id(&self, id: u32) -> Option<&Texture> {
            self.images.get(&id)
        }
        pub fn get_by_name(&self, name: &str) -> Option<&Texture> {
            let id = self.get_id(name)?;
            self.get_by_id(*id)
        }
        pub fn get_id(&self, name: &str) -> Option<&u32> {
            self.name_id_map.get(&name.to_string())
        }
    }
}
