// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Constants for the `f32` single-precision floating point type.
//!
//! *[See also the `f32` primitive type](primitive@f32).*
//!
//! Mathematically significant numbers are provided in the `consts` sub-module.
//!
//! For the constants defined directly in this module
//! (as distinct from those defined in the `consts` sub-module),
//! new code should instead use the associated constants
//! defined directly on the `f32` type.

#![allow(missing_docs)]

#[cfg(feature = "unit_test")]
mod tests;

use crate::intrinsics;
use crate::sys::cmath;

#[allow(deprecated, deprecated_in_future)]
pub use core::f32::{
    consts, DIGITS, EPSILON, INFINITY, MANTISSA_DIGITS, MAX, MAX_10_EXP, MAX_EXP, MIN, MIN_10_EXP,
    MIN_EXP, MIN_POSITIVE, NAN, NEG_INFINITY, RADIX,
};

impl f32 {
    /// Returns the largest integer less than or equal to `self`.
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub const fn floor(self) -> f32 {
        core::f32::math::floor(self)
    }

    /// Returns the smallest integer greater than or equal to `self`.
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub const fn ceil(self) -> f32 {
        core::f32::math::ceil(self)
    }

    /// Returns the nearest integer to `self`, rounding half-way cases away from `0.0`.
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub const fn round(self) -> f32 {
        core::f32::math::round(self)
    }

    /// Returns the nearest integer, rounding ties to even.
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub const fn round_ties_even(self) -> f32 {
        core::f32::math::round_ties_even(self)
    }

    /// Returns the integer part of `self`. Truncates towards zero.
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub const fn trunc(self) -> f32 {
        core::f32::math::trunc(self)
    }

    /// Returns the fractional part of `self`.
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub const fn fract(self) -> f32 {
        core::f32::math::fract(self)
    }

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding error.
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn mul_add(self, a: f32, b: f32) -> f32 {
        core::f32::math::mul_add(self, a, b)
    }

    /// Calculates Euclidean division.
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn div_euclid(self, rhs: f32) -> f32 {
        core::f32::math::div_euclid(self, rhs)
    }

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn rem_euclid(self, rhs: f32) -> f32 {
        core::f32::math::rem_euclid(self, rhs)
    }

    /// Raises a number to an integer power.
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn powi(self, n: i32) -> f32 {
        core::f32::math::powi(self, n)
    }

    /// Returns the square root of a number.
    ///
    /// Returns NaN if `self` is a negative number other than `-0.0`.
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn sqrt(self) -> f32 {
        intrinsics::sqrtf32(self)
    }

    /// Raises a number to a floating point power.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 2.0_f32;
    /// let abs_difference = (x.powf(2.0) - (x * x)).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn powf(self, n: f32) -> f32 {
        intrinsics::powf32(self, n)
    }
    /// Returns `e^(self)`, (the exponential function).
    ///
    /// # Examples
    ///
    /// ```
    /// let one = 1.0f32;
    /// // e^1
    /// let e = one.exp();
    ///
    /// // ln(e) - 1 == 0
    /// let abs_difference = (e.ln() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn exp(self) -> f32 {
        intrinsics::expf32(self)
    }

    /// Returns `2^(self)`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 2.0f32;
    ///
    /// // 2^2 - 4 == 0
    /// let abs_difference = (f.exp2() - 4.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn exp2(self) -> f32 {
        intrinsics::exp2f32(self)
    }

    /// Returns the natural logarithm of the number.
    ///
    /// # Examples
    ///
    /// ```
    /// let one = 1.0f32;
    /// // e^1
    /// let e = one.exp();
    ///
    /// // ln(e) - 1 == 0
    /// let abs_difference = (e.ln() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn ln(self) -> f32 {
        intrinsics::logf32(self)
    }

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// The result might not be correctly rounded owing to implementation details;
    /// `self.log2()` can produce more accurate results for base 2, and
    /// `self.log10()` can produce more accurate results for base 10.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = 5.0f32;
    ///
    /// // log5(5) - 1 == 0
    /// let abs_difference = (five.log(5.0) - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn log(self, base: f32) -> f32 {
        self.ln() / base.ln()
    }

    /// Returns the base 2 logarithm of the number.
    ///
    /// # Examples
    ///
    /// ```
    /// let two = 2.0f32;
    ///
    /// // log2(2) - 1 == 0
    /// let abs_difference = (two.log2() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn log2(self) -> f32 {
        intrinsics::log2f32(self)
    }

    /// Returns the base 10 logarithm of the number.
    ///
    /// # Examples
    ///
    /// ```
    /// let ten = 10.0f32;
    ///
    /// // log10(10) - 1 == 0
    /// let abs_difference = (ten.log10() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn log10(self) -> f32 {
        intrinsics::log10f32(self)
    }

    /// The positive difference of two numbers.
    ///
    /// * If `self <= other`: `0.0`
    /// * Else: `self - other`
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 3.0f32;
    /// let y = -3.0f32;
    ///
    /// let abs_difference_x = (x.abs_sub(1.0) - 2.0).abs();
    /// let abs_difference_y = (y.abs_sub(1.0) - 0.0).abs();
    ///
    /// assert!(abs_difference_x <= f32::EPSILON);
    /// assert!(abs_difference_y <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    #[deprecated(
        since = "1.10.0",
        note = "you probably meant `(self - other).abs()`: \
                this operation is `(self - other).max(0.0)` \
                except that `abs_sub` also propagates NaNs (also \
                known as `fdimf` in C). If you truly need the positive \
                difference, consider using that expression or the C function \
                `fdimf`, depending on how you wish to handle NaN (please consider \
                filing an issue describing your use-case too)."
    )]
    pub fn abs_sub(self, other: f32) -> f32 {
        unsafe { cmath::fdimf(self, other) }
    }

    /// Returns the cube root of a number.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 8.0f32;
    ///
    /// // x^(1/3) - 2 == 0
    /// let abs_difference = (x.cbrt() - 2.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn cbrt(self) -> f32 {
        unsafe { cmath::cbrtf(self) }
    }

    /// Compute the distance between the origin and a point (`x`, `y`) on the
    /// Euclidean plane. Equivalently, compute the length of the hypotenuse of a
    /// right-angle triangle with other sides having length `x.abs()` and
    /// `y.abs()`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 2.0f32;
    /// let y = 3.0f32;
    ///
    /// // sqrt(x^2 + y^2)
    /// let abs_difference = (x.hypot(y) - (x.powi(2) + y.powi(2)).sqrt()).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn hypot(self, other: f32) -> f32 {
        unsafe { cmath::hypotf(self, other) }
    }

    /// Computes the sine of a number (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// let x = std::f32::consts::FRAC_PI_2;
    ///
    /// let abs_difference = (x.sin() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn sin(self) -> f32 {
        intrinsics::sinf32(self)
    }

    /// Computes the cosine of a number (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 2.0 * std::f32::consts::PI;
    ///
    /// let abs_difference = (x.cos() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn cos(self) -> f32 {
        intrinsics::cosf32(self)
    }

    /// Computes the tangent of a number (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// let x = std::f32::consts::FRAC_PI_4;
    /// let abs_difference = (x.tan() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn tan(self) -> f32 {
        unsafe { cmath::tanf(self) }
    }

    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2] or NaN if the number is outside the range
    /// [-1, 1].
    ///
    /// # Examples
    ///
    /// ```
    /// let f = std::f32::consts::FRAC_PI_2;
    ///
    /// // asin(sin(pi/2))
    /// let abs_difference = (f.sin().asin() - std::f32::consts::FRAC_PI_2).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn asin(self) -> f32 {
        unsafe { cmath::asinf(self) }
    }

    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi] or NaN if the number is outside the range
    /// [-1, 1].
    ///
    /// # Examples
    ///
    /// ```
    /// let f = std::f32::consts::FRAC_PI_4;
    ///
    /// // acos(cos(pi/4))
    /// let abs_difference = (f.cos().acos() - std::f32::consts::FRAC_PI_4).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn acos(self) -> f32 {
        unsafe { cmath::acosf(self) }
    }

    /// Computes the arctangent of a number. Return value is in radians in the
    /// range [-pi/2, pi/2];
    ///
    /// # Examples
    ///
    /// ```
    /// let f = 1.0f32;
    ///
    /// // atan(tan(1))
    /// let abs_difference = (f.tan().atan() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn atan(self) -> f32 {
        unsafe { cmath::atanf(self) }
    }

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    ///
    /// # Examples
    ///
    /// ```
    /// // Positive angles measured counter-clockwise
    /// // from positive x axis
    /// // -pi/4 radians (45 deg clockwise)
    /// let x1 = 3.0f32;
    /// let y1 = -3.0f32;
    ///
    /// // 3pi/4 radians (135 deg counter-clockwise)
    /// let x2 = -3.0f32;
    /// let y2 = 3.0f32;
    ///
    /// let abs_difference_1 = (y1.atan2(x1) - (-std::f32::consts::FRAC_PI_4)).abs();
    /// let abs_difference_2 = (y2.atan2(x2) - (3.0 * std::f32::consts::FRAC_PI_4)).abs();
    ///
    /// assert!(abs_difference_1 <= f32::EPSILON);
    /// assert!(abs_difference_2 <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn atan2(self, other: f32) -> f32 {
        unsafe { cmath::atan2f(self, other) }
    }

    /// Simultaneously computes the sine and cosine of the number, `x`. Returns
    /// `(sin(x), cos(x))`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = std::f32::consts::FRAC_PI_4;
    /// let f = x.sin_cos();
    ///
    /// let abs_difference_0 = (f.0 - x.sin()).abs();
    /// let abs_difference_1 = (f.1 - x.cos()).abs();
    ///
    /// assert!(abs_difference_0 <= f32::EPSILON);
    /// assert!(abs_difference_1 <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn sin_cos(self) -> (f32, f32) {
        (self.sin(), self.cos())
    }

    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 1e-8_f32;
    ///
    /// // for very small x, e^x is approximately 1 + x + x^2 / 2
    /// let approx = x + x * x / 2.0;
    /// let abs_difference = (x.exp_m1() - approx).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn exp_m1(self) -> f32 {
        unsafe { cmath::expm1f(self) }
    }

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 1e-8_f32;
    ///
    /// // for very small x, ln(1 + x) is approximately x - x^2 / 2
    /// let approx = x - x * x / 2.0;
    /// let abs_difference = (x.ln_1p() - approx).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn ln_1p(self) -> f32 {
        unsafe { cmath::log1pf(self) }
    }

    /// Hyperbolic sine function.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f32::consts::E;
    /// let x = 1.0f32;
    ///
    /// let f = x.sinh();
    /// // Solving sinh() at 1 gives `(e^2-1)/(2e)`
    /// let g = ((e * e) - 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn sinh(self) -> f32 {
        unsafe { cmath::sinhf(self) }
    }

    /// Hyperbolic cosine function.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f32::consts::E;
    /// let x = 1.0f32;
    /// let f = x.cosh();
    /// // Solving cosh() at 1 gives this result
    /// let g = ((e * e) + 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// // Same result
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn cosh(self) -> f32 {
        unsafe { cmath::coshf(self) }
    }

    /// Hyperbolic tangent function.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f32::consts::E;
    /// let x = 1.0f32;
    ///
    /// let f = x.tanh();
    /// // Solving tanh() at 1 gives `(1 - e^(-2))/(1 + e^(-2))`
    /// let g = (1.0 - e.powi(-2)) / (1.0 + e.powi(-2));
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn tanh(self) -> f32 {
        unsafe { cmath::tanhf(self) }
    }

    /// Inverse hyperbolic sine function.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 1.0f32;
    /// let f = x.sinh().asinh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn asinh(self) -> f32 {
        let ax = self.abs();
        let ix = 1.0 / ax;
        (ax + (ax / (Self::hypot(1.0, ix) + ix))).ln_1p().copysign(self)
    }

    /// Inverse hyperbolic cosine function.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 1.0f32;
    /// let f = x.cosh().acosh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn acosh(self) -> f32 {
        if self < 1.0 {
            Self::NAN
        } else {
            (self + ((self - 1.0).sqrt() * (self + 1.0).sqrt())).ln()
        }
    }

    /// Inverse hyperbolic tangent function.
    ///
    /// # Examples
    ///
    /// ```
    /// let e = std::f32::consts::E;
    /// let f = e.tanh().atanh();
    ///
    /// let abs_difference = (f - e).abs();
    ///
    /// assert!(abs_difference <= 1e-5);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn atanh(self) -> f32 {
        0.5 * ((2.0 * self) / (1.0 - self)).ln_1p()
    }

    /// Gamma function.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(float_gamma)]
    /// let x = 5.0f32;
    ///
    /// let abs_difference = (x.gamma() - 24.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn gamma(self) -> f32 {
        unsafe { cmath::tgammaf(self) }
    }

    /// Natural logarithm of the absolute value of the gamma function
    ///
    /// The integer part of the tuple indicates the sign of the gamma function.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(float_gamma)]
    /// let x = 2.0f32;
    ///
    /// let abs_difference = (x.ln_gamma().0 - 0.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[rustc_allow_incoherent_impl]
    #[inline]
    pub fn ln_gamma(self) -> (f32, i32) {
        let mut signgamp: i32 = 0;
        let x = unsafe { cmath::lgammaf_r(self, &mut signgamp) };
        (x, signgamp)
    }
}
