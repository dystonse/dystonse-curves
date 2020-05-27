use crate::conversion::LikeANumber;
use crate::{Curve, TypedCurve};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Tup<X, Y> 
where X: Debug, Y: Debug
{
    pub x: X,
    pub y: Y,
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

    pub fn new(mut points: Vec<Tup<X, Y>>) -> Self {
        points.sort_by(|p1, p2| p1.x.make_into_f32().partial_cmp(&p2.x.make_into_f32()).unwrap());
        let value = IrregularDynamicCurve { points };
        value.check();
        return value;
    }

    fn check(&self) {
        assert_eq!(self.points.first().unwrap().y.make_into_f32(), 0.0, "First point does not define y = 0.");
        assert_eq!(self.points.last().unwrap().y.make_into_f32(), 1.0, "Last point does not define y = 1.");
        for i in 0..self.points.len() - 1 {
            let l = &self.points[i];
            let r = &self.points[i + 1];
            assert!(l.x < r.x, "Unsorted x values or duplicate x value.");
            assert!(l.y <= r.y, "Y does not increase montonously for increasing x.");
        }
    }

    pub fn add_point(&mut self, x: f32, y: f32) {
        let xt = X::make_from_f32(x);
        let yt = Y::make_from_f32(y);
        for i in 0..self.points.len() {
            if self.points[i].x == xt {
                panic!("Duplicate x value: {}", x);
            }

            if xt > self.points[i].x && xt < self.points[i + 1].x {
                if yt < self.points[i].y || yt > self.points[i + 1].y {
                    panic!("New point {},{} breaks monotony.", x, y);
                }
                self.points.insert(i + 1, Tup {x: xt, y: yt});
                return;
            }
        }
    }

    pub fn len(&self) -> usize {
        return self.points.len();
    }

    pub fn simplify(&mut self, tol: f32) {
        let mut delete_x : Vec<X> = Vec::new();
        self.simplify_rec(tol, 0, self.len() - 1, &mut delete_x);
        self.points.retain(|p| !delete_x.contains(&p.x));
    }

    fn simplify_rec(&mut self, tol: f32, start: usize, end: usize, delete_x: &mut Vec<X>) {
        if end - start < 2 { // keep all 1 or 2 points
            return;
        }
        let mut max_d = -1.0;
        let mut max_d_i = 0;

        let s = Self::tuple_to_f32(&self.points[start]);
        let e = Self::tuple_to_f32(&self.points[end]);

        let n = (e.1 - s.1, s.0 - e.0); // normal vector to se, transposed and one coordinate inverted
        for i in start+1 .. end {
            let d = self.distance(start, end, i, n);
            if d > max_d {
                max_d = d;
                max_d_i = i;
            }
        }

        if max_d <= tol { // discard all points in between
            for i in start +1 .. end {
                delete_x.push(self.points[i].x);
            }
        } else {
            self.simplify_rec(tol, start, max_d_i, delete_x);
            self.simplify_rec(tol, max_d_i, end, delete_x);
        }
    }

    fn tuple_to_f32(tup : &Tup<X, Y>) -> (f32, f32) {
        return (tup.x.make_into_f32(), tup.y.make_into_f32());
    }

    fn distance(&self, start: usize, _end: usize, i: usize, n: (f32, f32)) -> f32 {
        // compute the distance of p to the line throug s and e, where n is a normal vector of that line.
        // Formular adapted from https://www.mathelounge.de/521534/vektorenrechnung-abstand-zwischen-punkt-und-geraden-in-2d
        let s = Self::tuple_to_f32(&self.points[start]);
        let p = Self::tuple_to_f32(&self.points[i]);
        let s_minus_p = (p.0 - s.0, p.1 - s.1);
        return ((s_minus_p.0 * n.0 + s_minus_p.1 * n.1) / (n.0 * n.0 + n.1 * n.1).sqrt()).abs();       
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


    fn get_values_as_vectors(&self) -> (Vec<f32>, Vec<f32>) {
        let mut x : Vec<f32> = Vec::new();
        let mut y : Vec<f32> = Vec::new();

        for p in &self.points {
            x.push(p.x.make_into_f32());
            y.push(p.y.make_into_f32());
        }
        
        return (x,y);
    } 
}

#[cfg(test)]
mod tests {
    use crate::irregular_dynamic::{IrregularDynamicCurve, Tup};
    use crate::{Curve};
    use assert_approx_eq::assert_approx_eq;
    use gnuplot::{Figure, Caption, Color};

    extern crate rand;

    use rand::Rng;


    #[test]
    fn test_irregular() {
        let epsilon = 0.0001;

        let points = vec![
            Tup { x: 12.0, y: 0.0 },
            Tup { x: 14.0, y: 0.4 },
            Tup { x: 16.0, y: 0.4 }, // this point is redundant
            Tup { x: 20.0, y: 0.4 },
            Tup { x: 30.0, y: 0.7 },
            Tup { x: 13.0, y: 0.0 }, // This point is out-of-order within the Vec, but in-order regaring x and y
            Tup { x: 40.0, y: 1.0 },
        ];
        let mut c = IrregularDynamicCurve::<f32, f32>::new(points);

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

        c.add_point(35.0, 0.9);
        assert_approx_eq!(c.y_at_x(35.0), 0.9, epsilon);
        assert_approx_eq!(c.y_at_x(32.5), 0.8, epsilon);

        // let mut fg = Figure::new();
        // let axes = fg.axes2d();
        
        assert_eq!(c.len(), 8);
        let c_plot = c.get_values_as_vectors();
        // axes.lines_points(&c_plot.0, &c_plot.1, &[Caption("C original"), Color("grey")]);

        c.simplify(0.0);
        assert_eq!(c.len(), 7); // should only remove the redundant point
        // TODO if the curve begins with multuple 0.0 values or ends with mutluple 0.1 
        
        let c_plot = c.get_values_as_vectors();
        // axes.lines_points(&c_plot.0, &c_plot.1, &[Caption("C pseudo-simplified"), Color("black")]);

        c.simplify(0.1);
        assert!(c.len() < 7); // should remove at least one more point

        let c_plot = c.get_values_as_vectors();
        // axes.lines_points(&c_plot.0, &c_plot.1, &[Caption("C simplified"), Color("red")]);

        // fg.show();
    }

    #[test]
    fn test_many_points() {
        let points = vec![
            Tup { x: 0.0, y: 0.0 },
            Tup { x: 100.0, y: 1.0 },
        ];
        let mut c = IrregularDynamicCurve::<f32, f32>::new(points);

        let mut rng = rand::thread_rng();

        let mut y = 0.0;
        for i in 1..100 {
            y += rng.gen_range(0.0, 0.018);
            c.add_point(i as f32, y);
        }
        
        let mut fg = Figure::new();
        let axes = fg.axes2d();
        
        let c_plot = c.get_values_as_vectors();
        axes.lines_points(&c_plot.0, &c_plot.1, &[Caption("C original"), Color("grey")]);

        c.simplify(0.01);
        
        let c_plot = c.get_values_as_vectors();
        axes.lines_points(&c_plot.0, &c_plot.1, &[Caption("C simplified"), Color("red")]);

        c.simplify(0.01);
        c.simplify(0.01);
        c.simplify(0.01);
        
        let c_plot = c.get_values_as_vectors();
        axes.lines_points(&c_plot.0, &c_plot.1, &[Caption("C over-simplified"), Color("green")]);

        fg.show();
    }
}
