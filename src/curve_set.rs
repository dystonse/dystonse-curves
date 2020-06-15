use crate::conversion::LikeANumber;
use crate::Curve;
use crate::weighted_average;
use crate::irregular_dynamic::IrregularDynamicCurve;
use simple_error::{SimpleError, bail};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CurveSet<T, C> where 
    T: LikeANumber,
    C: Curve
{
    pub curves: Vec<(T,C)>
}

impl<T, C> CurveSet<T, C> where 
    T: LikeANumber,
    C: Curve
{
    pub fn new() -> Self {
        return Self {
            curves: vec!{}
        };
    }

    pub fn min_x(&self) -> f32 {
        return self.curves.first().unwrap().0.make_into_f32();
    }

    pub fn max_x(&self) -> f32 {
        return self.curves.last().unwrap().0.make_into_f32();
    }

    fn binary_search_by_x(&self, x: f32, start: usize, end: usize) -> (usize, IrregularDynamicCurve<f32, f32>) {
        if start + 1 == end {
            let (lx, lc) = &self.curves[start];
            let (rx, rc) = &self.curves[end];
            let a = (x - lx.make_into_f32()) / (rx.make_into_f32() - lx.make_into_f32());
            return (start, weighted_average(vec!{Box::new(lc), Box::new(rc)}, vec!{(1.0 - a), a}));
        } else {
            let mid = (start + end) / 2;
            if x < self.curves[mid].0.make_into_f32() {
                return self.binary_search_by_x(x, start, mid);
            } else {
                return self.binary_search_by_x(x, mid, end);
            }
        }
    }

    /// Returns the curve that would correspond to the given x value. If x is out of 
    /// bounds, it uses the two nearest cuves to provide an extrapolation.
    /// Otherise, two curves may be interpolated to generate the result.
    /// TODO this extrapolation is completely untested and is - in the best case - a
    /// bug which turned into a feature
    pub fn curve_at_x_with_extrapolation(&self, x: f32) -> IrregularDynamicCurve<f32, f32> {
        return self.binary_search_by_x(x, 0, self.curves.len() - 1).1;
    }

    /// Returns the curve that would correspond to the given x value. If x is out of 
    /// bounds, it returns the curve which is at the bounds. Otherise, two curves may be 
    /// interpolated to generate the result.
    pub fn curve_at_x_with_continuation(&self, x: f32) -> IrregularDynamicCurve<f32, f32> {
        if x <= self.min_x() {
            return weighted_average(vec!{Box::new(&self.curves.first().unwrap().1)}, vec!{1.0});
        }
        if x >= self.max_x() {
            return weighted_average(vec!{Box::new(&self.curves.last().unwrap().1)}, vec!{1.0});
        }
        return self.binary_search_by_x(x, 0, self.curves.len() - 1).1;
    }

    /// Returns the curve that would correspond to the given x value. If x is out of 
    /// bounds, it panics. Otherise, two curves may be interpolated to generate
    /// the result.
    pub fn curve_at_x(&self, x: f32) -> Result<IrregularDynamicCurve<f32, f32>, SimpleError> {
        if x <= self.min_x() {
            bail!("X below minimum.");
        }
        if x >= self.max_x() {
            bail!("X above maximum.");
        }
        return Ok(self.binary_search_by_x(x, 0, self.curves.len() - 1).1);
    }

    pub fn add_curve(&mut self, x: T, curve: C) {
        if self.curves.is_empty() || x.make_into_f32() <= self.min_x() {
            self.curves.insert(0, (x, curve));
            return;
        }
        if x.make_into_f32() >= self.max_x() {
            self.curves.push((x, curve));
            return;
        }

        for i in 0..self.curves.len() {
            if self.curves[i].0 == x {
                panic!("Duplicate x value: {}", x.make_into_f32());
            }

            if x > self.curves[i].0 && x < self.curves[i + 1].0 {
                self.curves.insert(i + 1, (x, curve));
                return;
            }
        }
    }
}