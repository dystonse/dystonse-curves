mod conversion;
pub mod regular_dynamic;
pub mod irregular_dynamic;

use irregular_dynamic::{IrregularDynamicCurve, Tup};
use itertools::Itertools;

/**
 * Trait to access the curve's values using f32 as type for X 
 * and Y, irrespective of the types used internally.
 */
pub trait Curve
{
    fn min_x(&self) -> f32;
    fn max_x(&self) -> f32;
    fn y_at_x(&self, x: f32) -> f32;
    fn x_at_y(&self, y: f32) -> f32;
    fn get_values_as_vectors(&self) -> (Vec<f32>, Vec<f32>);
    fn get_x_values(&self) -> Vec<f32>; // TODO return iterator instead of Vec
}

/**
 * Trait to acces the curve's values using the types which are
 * used internally to store the data.
 */
pub trait TypedCurve<X, Y>
{
    fn typed_min_x(&self) -> X;
    fn typed_max_x(&self) -> X;
    fn typed_y_at_x(&self, x: X) -> Y;
    fn typed_x_at_y(&self, y: Y) -> X;
}

// calculate a weighted average between several curves
pub fn weighted_average(curves: Vec<Box<&dyn Curve>>, weights: Vec<f32>) -> IrregularDynamicCurve<f32, f32> {
    
    // make sure the number of weights and curves match:
    assert_eq!(curves.len(), weights.len(), "invalid arguments: number of curves and weights must be the same.");
    
    // correction factor to make sure the weights will add up to 1.0:
    let f = 1.0 / weights.iter().sum::<f32>();

    // gather x values from all curves:
    let x_values = curves.iter().map(|c| c.get_x_values()).kmerge().dedup();

    // make a vector of (curve, weight)-tuples: 
    let zipped : Vec<_> = curves.iter().zip(weights.iter()).collect();

    // this is where the actual interpolation happens:
    let points = x_values.map(|x| {
        let mut y = 0.0;
        for (c, w) in zipped.iter() {
            y += c.y_at_x(x) * **w;
        }
        Tup {x, y: y * f}
    }).collect();

    // make a curve from all the newly calculated points, throwing away unnecessary ones:
    let mut ret = IrregularDynamicCurve::<f32, f32>::new(points);
    ret.simplify(0.0);

    return ret;
}

/// Compute the distance if two curves, defindes as the area between the two
pub fn distance(a: &impl Curve, b: &impl Curve) -> f32 {
    // gather x values from all curves:
    let x_a = a.get_x_values();
    let x_b = b.get_x_values();
    let x_values = x_a.iter().merge(x_b.iter()).dedup();

    // for each relevant x, get the difference of the ys of both curves
    x_values.map(|x| {
        let y_a = a.y_at_x(*x);
        let y_b = b.y_at_x(*x);
        (x, y_a - y_b)
    }).tuple_windows().map(|((x1, dy1), (x2, dy2))| { // 
        let h = x2 - x1;
        let a = dy1.abs();
        let c = dy2.abs();
        if dy1 * dy2 >= 0.0 { // same sings, true trapezoid or triangle
            (a + c) * h * 0.5
        } else { // different signs, self-intersecting trapezoid
            h * 0.5 * (a*a + c*c) / (a + c)
        }
    }).sum()
}

// TODO Move tests into own file?
// TODO Test multiple consecutive points with the same value
// TODO split test functions
#[cfg(test)]
mod tests {
    use crate::{Curve, TypedCurve, distance, weighted_average};
    use crate::regular_dynamic::RegularDynamicCurve;
    use crate::conversion::LikeANumber;
    use assert_approx_eq::assert_approx_eq;
    use fixed::types::{U1F7, U1F15};
    // use half::prelude::*;

    #[test]
    fn test_all() {
        test_curve::<RegularDynamicCurve<f32,   f32>, f32,   f32>(true , 0.000001);
        test_curve::<RegularDynamicCurve< i8,   f32>,  i8,   f32>(false, 0.000001);
        test_curve::<RegularDynamicCurve<f32,  U1F7>, f32,  U1F7>(true , 0.05);
        test_curve::<RegularDynamicCurve<f32, U1F15>, f32, U1F15>(true , 0.0005);
        // test_curve::<RegularDynamicCurve<f32,   f16>, f32,   f16>(true , 0.005);
    }

    fn test_curve<T, X, Y>(test_float_x: bool, epsilon: f32) 
    where X: LikeANumber,
          Y: LikeANumber,
          T: Curve + TypedCurve<X, Y>
        {
            let c = RegularDynamicCurve::<X, Y>::new(
                10.0,
                10.0,
                vec!{0.0, 0.6, 1.0}
            );

            test_curve_typed(&c, test_float_x, epsilon);
            test_curve_untyped(&c, test_float_x, epsilon);
        }

    fn test_curve_untyped(c: & impl Curve, test_float_x: bool, epsilon: f32) 
        {
        // Test x bounds
        assert_eq!(c.min_x(), 10.0);
        assert_eq!(c.max_x(), 30.0);

        // Test x outside of bounds
        assert_eq!(c.y_at_x(0.0), 0.0);
        assert_eq!(c.y_at_x(100.0), 1.0);

        // Test x equal to the actual points
        assert_approx_eq!(c.y_at_x(10.0), 0.0, epsilon);
        assert_approx_eq!(c.y_at_x(20.0), 0.6, epsilon);
        assert_approx_eq!(c.y_at_x(30.0), 1.0, epsilon);

        // Test arbitrary "integer" x values
        assert_approx_eq!(c.y_at_x(15.0), 0.3, epsilon);
        assert_approx_eq!(c.y_at_x(25.0), 0.8, epsilon);

        if test_float_x {
            // Test arbitrary "float" x values
            assert_approx_eq!(c.y_at_x(12.5), 0.15, epsilon);
            assert_approx_eq!(c.y_at_x(17.5), 0.45, epsilon);
        }

        // Test y queries
        assert_approx_eq!(c.x_at_y(0.0), 10.0, epsilon);
        assert_approx_eq!(c.x_at_y(1.0), 30.0, epsilon);
        assert_approx_eq!(c.x_at_y(0.6), 20.0, epsilon);
        if test_float_x {
            assert_approx_eq!(c.x_at_y(0.15), 12.5, epsilon);
            assert_approx_eq!(c.x_at_y(0.45), 17.5, epsilon);
        }
    }

    fn test_curve_typed<X : LikeANumber, Y : LikeANumber>(c: & impl TypedCurve<X, Y>, test_float_x: bool, epsilon: f32) 
        {
        // Test x bounds
        assert_eq!(c.typed_min_x().make_into_f32(), 10.0);
        assert_eq!(c.typed_max_x().make_into_f32(), 30.0);

        // Test x outside of bounds
        assert_eq!(c.typed_y_at_x(X::make_from_f32(0.0)).make_into_f32(), 0.0);
        assert_eq!(c.typed_y_at_x(X::make_from_f32(100.0)).make_into_f32(), 1.0);

        // Test x equal to the actual points
        assert_approx_eq!(c.typed_y_at_x(X::make_from_f32(10.0)).make_into_f32(), 0.0, epsilon);
        assert_approx_eq!(c.typed_y_at_x(X::make_from_f32(20.0)).make_into_f32(), 0.6, epsilon);
        assert_approx_eq!(c.typed_y_at_x(X::make_from_f32(30.0)).make_into_f32(), 1.0, epsilon);

        // Test arbitrary "integer" x values
        assert_approx_eq!(c.typed_y_at_x(X::make_from_f32(15.0)).make_into_f32(), 0.3, epsilon);
        assert_approx_eq!(c.typed_y_at_x(X::make_from_f32(25.0)).make_into_f32(), 0.8, epsilon);

        if test_float_x {
            // Test arbitrary "float" x values
            assert_approx_eq!(c.typed_y_at_x(X::make_from_f32(12.5)).make_into_f32(), 0.15, epsilon);
            assert_approx_eq!(c.typed_y_at_x(X::make_from_f32(17.5)).make_into_f32(), 0.45, epsilon);
        }

        // Test y queries
        assert_approx_eq!(c.typed_x_at_y(Y::make_from_f32(0.0)).make_into_f32(), 10.0, epsilon);
        assert_approx_eq!(c.typed_x_at_y(Y::make_from_f32(1.0)).make_into_f32(), 30.0, epsilon);
        assert_approx_eq!(c.typed_x_at_y(Y::make_from_f32(0.6)).make_into_f32(), 20.0, epsilon);
        if test_float_x {
            assert_approx_eq!(c.typed_x_at_y(Y::make_from_f32(0.15)).make_into_f32(), 12.5, epsilon);
            assert_approx_eq!(c.typed_x_at_y(Y::make_from_f32(0.45)).make_into_f32(), 17.5, epsilon);
        }
    }

    #[test]
    fn test_distance() {
        let c1 = RegularDynamicCurve::<f32, f32>::new(
            10.0,
            10.0,
            vec!{0.0, 0.6, 0.6, 0.6, 0.7, 1.0}
        );

        let c2 = RegularDynamicCurve::<f32, f32>::new(
            5.0,
            3.0,
            vec!{0.0, 0.2, 0.6, 0.7, 0.7, 1.0}
        );

        let c3 = weighted_average(vec!{Box::new(&c1), Box::new(&c2)}, vec!{0.5, 0.5});

        assert_approx_eq!(distance(&c1, &c1), 0.0);
        assert_ne!(distance(&c1, &c2), 0.0);
        assert_approx_eq!(distance(&c1, &c2), distance(&c2, &c1));

        // c3 is exactly between c1 and c2, so both should have the same distance from c3
        assert_approx_eq!(distance(&c1, &c3), distance(&c2, &c3));

        // direct distance from c1 to c2 should be the same as using the
        // detour via c3, because c3 lies exactly in the middle of both
        assert_approx_eq!(distance(&c1, &c2),  distance(&c1, &c3) + distance(&c2, &c3));

        // all non-zero distance should be positive
        assert!(distance(&c1, &c2) > 0.0);
        assert!(distance(&c2, &c1) > 0.0);
        assert!(distance(&c1, &c3) > 0.0);
        assert!(distance(&c3, &c1) > 0.0);
        assert!(distance(&c3, &c2) > 0.0);
        assert!(distance(&c2, &c3) > 0.0);
    }
}
