use crate::conversion::ConvertF32;
use crate::{Curve, TypedCurve};

use std::ops::{Add, Sub, Mul, Div};
use std::cmp::{PartialOrd};
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

impl<X, Y> Curve for RegularDynamicCurve<X, Y>
where X: ConvertF32 + Copy + Sub<X, Output = X> + Add<X, Output = X> + Div<X, Output = X> + Mul<X, Output = X> + PartialOrd,
Y: ConvertF32 + Copy
{
    fn new(n: usize, s: f32, x0: f32, y: Vec<f32>) -> Self {
        return Self{
            n,
            s: X::make_from_f32(s),
            x0: X::make_from_f32(x0),
            y: y.iter().map(|yp| Y::make_from_f32(*yp)).collect()
        };
    }

    fn min_x(&self) -> f32 {
        return self.x0.make_into_f32();
    }

    fn max_x(&self) -> f32
    {
        let len = self.s.make_into_f32() * ((self.n - 1) as f32);
        return self.x0.make_into_f32() + len;
    }

    fn y_at_x(&self, x: f32) -> f32 {
        if x <= self.min_x() {
            return self.y[0].make_into_f32();
        }
        if x >= self.max_x() {
            return self.y[self.n - 1].make_into_f32();
        }

        let i = (x - self.min_x()) / self.s.make_into_f32();
       
        let i_min = i.floor() as usize;
        let i_max = i.ceil() as usize;

        if i_max == i_min {
            return self.y[i_min].make_into_f32();
        }

        let a = i.fract();
        return self.y[i_min].make_into_f32() * (1.0 - a) + 
               self.y[i_max].make_into_f32() * a;
    }

    /**
     * TODO when multiple consecutive points have the given Y value, the first X value will be returned. 
     * We could as well return the last one, or the center, or whatever.
     */
    fn x_at_y(&self, y: f32) -> f32 {
        assert!(y >= 0.0);
        assert!(y <= 1.0);

        if y == 0.0 {
            return self.min_x();
        }

        if y == 1.0 {
            return self.max_x();
        }

        for i in 0..self.n {
            let v_r = self.y[i].make_into_f32();
            if v_r == y {
                return self.min_x() + i as f32 * self.s.make_into_f32();
            }
            if v_r > y {
                assert!(i > 0);
                let v_l = self.y[i - 1].make_into_f32();
                let a = (y - v_l) / (v_r - v_l);
                return self.min_x() + ((i - 1) as f32 + a) * self.s.make_into_f32();
            }
        }

        panic!("Did not find y = {}", y);
    }
}

impl<X, Y> TypedCurve<X, Y> for RegularDynamicCurve<X, Y>
    where X: ConvertF32 + Copy + Sub<X, Output = X> + Add<X, Output = X> + Div<X, Output = X> + Mul<X, Output = X> + PartialOrd,
          Y: ConvertF32 + Copy
{
    fn typed_new(n: usize, s: X, x0: X, y: Vec<Y>) -> Self {
        return Self{
            n,s,x0,y
        };
    }

    fn typed_min_x(&self) -> X {
        return self.x0;
    }

    fn typed_max_x(&self) -> X
    {
        let len : X = self.s * X::make_from_f32((self.n - 1) as f32);
        return self.x0 + len;
    }

    fn typed_y_at_x(&self, x: X) -> Y {
        if x <= self.x0 {
            return self.y[0];
        }
        if x >= self.typed_max_x() {
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
    fn typed_x_at_y(&self, y: Y) -> X {
        let yf = y.make_into_f32();
        assert!(yf >= 0.0);
        assert!(yf <= 1.0);

        if yf == 0.0 {
            return self.typed_min_x();
        }

        if yf == 1.0 {
            return self.typed_max_x();
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