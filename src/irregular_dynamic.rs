use crate::conversion::LikeANumber;
use crate::{Curve, TypedCurve};

pub struct Tup<X, Y> {
    x: X,
    y: Y,
}

/**
 * A curve that has a dynamic length and data points at regular distances.
 */
pub struct IrregularDynamicCurve<X, Y>
where
    X: LikeANumber,
    Y: LikeANumber,
{
    points: Vec<Tup<X, Y>>,
}

impl<X, Y> IrregularDynamicCurve<X, Y>
where
    X: LikeANumber,
    Y: LikeANumber,
{
    fn binary_search_by_x(&self, x: f32, start: usize, end: usize) -> f32 {
        if start + 1 == end {
            let l = &self.points[start];
            let r = &self.points[end];
            let a = (x - l.x.make_into_f32()) / (r.x.make_into_f32() - l.x.make_into_f32());
            return l.y.make_into_f32() * (1.0 - a) + r.y.make_into_f32() * a;
        } else {
            let mid = (start + end) / 2;
            if x < self.points[mid].x.make_into_f32() {
                return self.binary_search_by_x(x, start, mid);
            } else {
                return self.binary_search_by_x(x, mid, end);
            }
        }
    }

    fn binary_search_by_y(&self, y: f32, start: usize, end: usize) -> f32 {
        if start + 1 == end {
            let l = &self.points[start];
            let r = &self.points[end];
            let a = (y - l.y.make_into_f32()) / (r.y.make_into_f32() - l.y.make_into_f32());
            return l.x.make_into_f32() * (1.0 - a) + r.x.make_into_f32() * a;
        } else {
            let mid = (start + end) / 2;
            if y < self.points[mid].y.make_into_f32() {
                return self.binary_search_by_y(y, start, mid);
            } else {
                return self.binary_search_by_y(y, mid, end);
            }
        }
    }

    pub fn new(points: Vec<Tup<X, Y>>) -> Self {
        return IrregularDynamicCurve { points };
    }
}

impl<X, Y> Curve for IrregularDynamicCurve<X, Y>
where
    X: LikeANumber,
    Y: LikeANumber,
{
    fn min_x(&self) -> f32 {
        return self.points.first().unwrap().x.make_into_f32();
    }

    fn max_x(&self) -> f32 {
        return self.points.last().unwrap().x.make_into_f32();
    }

    fn y_at_x(&self, x: f32) -> f32 {
        if x <= self.min_x() {
            return 0.0;
        }
        if x >= self.max_x() {
            return 1.0;
        }
        return self.binary_search_by_x(x, 0, self.points.len());
    }

    fn x_at_y(&self, y: f32) -> f32 {
        if y == 0.0 {
            return self.min_x();
        }
        if y == 1.0 {
            return self.max_x();
        }
        return self.binary_search_by_y(y, 0, self.points.len());
    }
}

#[cfg(test)]
mod tests {
    use crate::irregular_dynamic::{IrregularDynamicCurve, Tup};
    use crate::{Curve};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_irregular() {
        let epsilon = 0.0001;

        let points = vec![
            Tup { x: 12.0, y: 0.0 },
            Tup { x: 13.0, y: 0.0 },
            Tup { x: 14.0, y: 0.4 },
            Tup { x: 20.0, y: 0.4 },
            Tup { x: 30.0, y: 0.7 },
            Tup { x: 40.0, y: 1.0 },
        ];
        let c = IrregularDynamicCurve::<f32, f32>::new(points);

        // Test x bounds
        assert_eq!(c.min_x(), 12.0);
        assert_eq!(c.max_x(), 40.0);

        // Test x outside of bounds
        assert_eq!(c.y_at_x(0.0), 0.0);
        assert_eq!(c.y_at_x(100.0), 1.0);

        // Test x equal to the actual points
        assert_approx_eq!(c.y_at_x(12.0), 0.0, epsilon);
        assert_approx_eq!(c.y_at_x(13.0), 0.0, epsilon);
        assert_approx_eq!(c.y_at_x(14.0), 0.4, epsilon);
        assert_approx_eq!(c.y_at_x(40.0), 1.0, epsilon);

        // Test arbitrary "integer" x values
        assert_approx_eq!(c.y_at_x(25.0), 0.55, epsilon);
        assert_approx_eq!(c.y_at_x(35.0), 0.85, epsilon);
    
        // Test arbitrary "float" x values
        assert_approx_eq!(c.y_at_x(13.5), 0.2, epsilon);
        assert_approx_eq!(c.y_at_x(15.5), 0.4, epsilon);
    
        // Test y queries
        assert_approx_eq!(c.x_at_y(0.0), 12.0, epsilon);
        assert_approx_eq!(c.x_at_y(1.0), 40.0, epsilon);
        assert!(c.x_at_y(0.4) >= 14.0);
        assert!(c.x_at_y(0.4) <= 20.0);
        assert_approx_eq!(c.x_at_y(0.7), 30.0, epsilon);
        
        assert_approx_eq!(c.x_at_y(0.2), 13.5, epsilon);
    }
}
