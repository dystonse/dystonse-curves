use std::ops::{Add, Sub, Mul, Div};
use std::cmp::{PartialOrd};
use fixed::types::U1F7;
use fixed::traits::{LossyFrom};
use half::prelude::*;

/** 
 * This is an implementation of https://xkcd.com/927/ for 
 * converting number-like values to/from f32.
 * 
 * Many different types behave like floating point numbers,
 * but they don't support all the operators that f32 has,
 * and their conversion methods are different. Due to 
 * ophanage rules, we can't just implement one of the 
 * exisitng conversion traits for all those types - we
 * have to invent our own.
*/
pub trait ConvertF32 {
    fn make_into_f32(self) -> f32;
    fn make_from_f32(value: f32) -> Self;
}

/** Trivial "conversion" from f32 to f32. */
impl ConvertF32 for f32 {
    fn make_into_f32(self) -> f32 {
        return self;
    }

    fn make_from_f32(value: f32) -> Self {
        return value;
    }
}

impl ConvertF32 for U1F7 {
    fn make_into_f32(self) -> f32 {
        return f32::lossy_from(self);
    }

    fn make_from_f32(value: f32) -> Self {
        return U1F7::from_num(value);
    }
}

impl ConvertF32 for f16 {
    fn make_into_f32(self) -> f32 {
        return self.to_f32();
    }

    fn make_from_f32(value: f32) -> Self {
        return f16::from_f32(value);
    }
}

impl ConvertF32 for i8 {
    fn make_into_f32(self) -> f32 {
        return self.into();
    }

    fn make_from_f32(value: f32) -> Self {
        return value as i8;
    }
}


pub trait Curve<X, Y>
{
    fn min_x(&self) -> X;
    fn max_x(&self) -> X;
    fn y_at_x(&self, x: X) -> Y;
    fn x_at_y(&self, y: Y) -> X;
}

/**
 * A curve that has a dynamic length and data points at regular distances.
 */
pub struct RegularDynamicCurve<X, Y> 
    where X: ConvertF32,
          Y: ConvertF32 {
    n: usize,
    s: X,
    x0: X,
    y: Vec<Y>
}

impl<X, Y> Curve<X, Y> for RegularDynamicCurve<X, Y>
    where X: ConvertF32 + Copy + Sub<X, Output = X> + Add<X, Output = X> + Div<X, Output = X> + Mul<X, Output = X> + PartialOrd,
          Y: ConvertF32 + Copy
{
    fn min_x(&self) -> X {
        return self.x0;
    }

    fn max_x(&self) -> X
    {
        let len : X = self.s * X::make_from_f32((self.n - 1) as f32);
        return self.x0 + len;
    }

    fn y_at_x(&self, x: X) -> Y {
        if x <= self.x0 {
            return self.y[0];
        }
        if x >= self.max_x() {
            return self.y[self.n - 1];
        }

        let i = X::make_into_f32(x - self.x0) / X::make_into_f32(self.s);
       
        let i_min = i.floor() as usize;
        let i_max = i.ceil() as usize;

        if i_max == i_min {
            return self.y[i_min];
        }

        let a = i.fract();
        return Y::make_from_f32(self.y[i_min].make_into_f32() * (1.0 - a) + 
                                self.y[i_max].make_into_f32() * a);
    }

    /**
     * TODO when multiple consecutive points have the given Y value, the first X value will be returned. 
     * We could as well return the last one, or the center, or whatever.
     */
    fn x_at_y(&self, y: Y) -> X {
        let yf = y.make_into_f32();
        assert!(yf >= 0.0);
        assert!(yf <= 1.0);

        if yf == 0.0 {
            return self.min_x();
        }

        if yf == 1.0 {
            return self.max_x();
        }

        for i in 0..self.n {
            let v_r = self.y[i].make_into_f32();
            if v_r == yf {
                return X::make_from_f32(self.x0.make_into_f32() + i as f32 * self.s.make_into_f32());
            }
            if v_r > yf {
                assert!(i > 0);
                let v_l = self.y[i - 1].make_into_f32();
                let a = (yf - v_l) / (v_r - v_l);
                return X::make_from_f32(self.x0.make_into_f32() + ((i - 1) as f32 + a) * self.s.make_into_f32());
            }
        }

        panic!("Did not find y = {}", yf);
    }
}

// TODO Move tests into own file?
// TODO Make test function generic
// TODO Test multiple consecutive points with the same value
// TODO split test functions
#[cfg(test)]
mod tests {
    use crate::{Curve, RegularDynamicCurve, ConvertF32};
    use assert_approx_eq::assert_approx_eq;
    use fixed::types::U1F7;
    use half::prelude::*;

    #[test]
    fn test_float_x() {
        let c : RegularDynamicCurve<f32, f32> = RegularDynamicCurve {
            n: 3,
            x0: 10.0,
            s: 10.0,
            y: vec!{0.0, 0.6, 1.0}
        };

        // Test x bounds
        assert_eq!(c.min_x(), 10.0);
        assert_eq!(c.max_x(), 30.0);

        // Test x outside of bounds
        assert_eq!(c.y_at_x(0.0), 0.0);
        assert_eq!(c.y_at_x(100.0), 1.0);

        // Test x equal to the actual points
        assert_eq!(c.y_at_x(10.0), 0.0);
        assert_eq!(c.y_at_x(20.0), 0.6);
        assert_eq!(c.y_at_x(30.0), 1.0);

        // Test arbitrary "integer" x values
        assert_eq!(c.y_at_x(15.0), 0.3);
        assert_eq!(c.y_at_x(25.0), 0.8);

        // Test arbitrary "float" x values
        assert_approx_eq!(c.y_at_x(12.5), 0.15);
        assert_approx_eq!(c.y_at_x(17.5), 0.45);

        // Test y queries
        assert_eq!(c.x_at_y(0.0), 10.0);
        assert_eq!(c.x_at_y(1.0), 30.0);
        assert_eq!(c.x_at_y(0.6), 20.0);
        assert_eq!(c.x_at_y(0.15), 12.5);
        assert_eq!(c.x_at_y(0.45), 17.5);
    }

    #[test]
    fn test_int_x() {
        let c : RegularDynamicCurve<i8, f32> = RegularDynamicCurve {
            n: 3,
            x0: 10,
            s: 10,
            y: vec!{0.0, 0.6, 1.0}
        };

        // Test x bounds
        assert_eq!(c.min_x(), 10);
        assert_eq!(c.max_x(), 30);

        // Test x outside of bounds
        assert_eq!(c.y_at_x(0), 0.0);
        assert_eq!(c.y_at_x(100), 1.0);

        // Test x equal to the actual points
        assert_eq!(c.y_at_x(10), 0.0);
        assert_eq!(c.y_at_x(20), 0.6);
        assert_eq!(c.y_at_x(30), 1.0);

        // Test arbitrary "integer" x values
        assert_eq!(c.y_at_x(15), 0.3);
        assert_eq!(c.y_at_x(25), 0.8);

        // Test arbitrary "float" x values
        assert_approx_eq!(c.y_at_x(12), 0.15, 0.031);
        assert_approx_eq!(c.y_at_x(17), 0.45, 0.031);

        // Test y queries
        assert_eq!(c.x_at_y(0.0), 10);
        assert_eq!(c.x_at_y(1.0), 30);
        assert_eq!(c.x_at_y(0.6), 20);
        assert_eq!(c.x_at_y(0.15), 12);
        assert_eq!(c.x_at_y(0.45), 17);
    }

    #[test]
    fn test_fixed_y() {
        let c : RegularDynamicCurve<f32, U1F7> = RegularDynamicCurve {
            n: 3,
            x0: 10.0,
            s: 10.0,
            y: vec!{U1F7::from_num(0.0), 
                    U1F7::from_num(0.6), 
                    U1F7::from_num(1.0)}
        };

        // Test x outside of bounds
        assert_approx_eq!(c.y_at_x(0.0).make_into_f32(), 0.0, 0.03);
        assert_approx_eq!(c.y_at_x(100.0).make_into_f32(), 1.0, 0.03);

        // Test x equal to the actual points
        assert_approx_eq!(c.y_at_x(10.0).make_into_f32(), 0.0, 0.03);
        assert_approx_eq!(c.y_at_x(20.0).make_into_f32(), 0.6, 0.03);
        assert_approx_eq!(c.y_at_x(30.0).make_into_f32(), 1.0, 0.03);

        // Test arbitrary "integer" x values
        assert_approx_eq!(c.y_at_x(15.0).make_into_f32(), 0.3, 0.03);
        assert_approx_eq!(c.y_at_x(25.0).make_into_f32(), 0.8, 0.03);

        // Test arbitrary "float" x values
        assert_approx_eq!(c.y_at_x(12.5).make_into_f32(), 0.15, 0.03);
        assert_approx_eq!(c.y_at_x(17.5).make_into_f32(), 0.45, 0.03);
    }

    #[test]
    fn test_half_y() {
        let c : RegularDynamicCurve<f32, f16> = RegularDynamicCurve {
            n: 3,
            x0: 10.0,
            s: 10.0,
            y: vec!{f16::from_f32(0.0), 
                    f16::from_f32(0.6), 
                    f16::from_f32(1.0)}
        };

        // Test x outside of bounds
        assert_approx_eq!(c.y_at_x(0.0).make_into_f32(), 0.0, 0.003);
        assert_approx_eq!(c.y_at_x(100.0).make_into_f32(), 1.0, 0.003);

        // Test x equal to the actual points
        assert_approx_eq!(c.y_at_x(10.0).make_into_f32(), 0.0, 0.003);
        assert_approx_eq!(c.y_at_x(20.0).make_into_f32(), 0.6, 0.003);
        assert_approx_eq!(c.y_at_x(30.0).make_into_f32(), 1.0, 0.003);

        // Test arbitrary "integer" x values
        assert_approx_eq!(c.y_at_x(15.0).make_into_f32(), 0.3, 0.003);
        assert_approx_eq!(c.y_at_x(25.0).make_into_f32(), 0.8, 0.003);

        // Test arbitrary "float" x values
        assert_approx_eq!(c.y_at_x(12.5).make_into_f32(), 0.15, 0.003);
        assert_approx_eq!(c.y_at_x(17.5).make_into_f32(), 0.45, 0.003);
    }
}
