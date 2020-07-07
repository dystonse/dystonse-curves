use crate::conversion::LikeANumber;
use crate::{Curve, TypedCurve};
use gnuplot::{Figure, Caption, Color};
use serde::{Serialize, Deserialize};
use crate::tree::{LeafData, SerdeFormat};
use std::fmt::{Debug, Display, Formatter};

/**
 * A curve that has a dynamic length and data points at regular distances.
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegularDynamicCurve<X, Y> 
where X: LikeANumber, Y: LikeANumber {
    s: X,
    x0: X,
    y: Vec<Y>
}

impl<X, Y> RegularDynamicCurve<X, Y>
where X: LikeANumber, Y: LikeANumber
{
    pub fn new( s: f32, x0: f32, y: Vec<f32>) -> Self {
        let value = Self{
            s: X::make_from_f32(s),
            x0: X::make_from_f32(x0),
            y: y.iter().map(|yp| Y::make_from_f32(*yp)).collect()
        };
        value.check();
        return value;
    }

    pub fn typed_new(s: X, x0: X, y: Vec<Y>) -> Self {
        return Self{
            s,x0,y
        };
    }

    // generates a graph of this curve and shows it in a gnuplot window
    pub fn plot_curve_with_gnuplot(&self) {
        let mut x = Vec::<f32>::new();
        for i in 0..self.y.len() {
            x.push(self.x0.make_into_f32()+(i as f32)*self.s.make_into_f32());
        }
        let y: Vec<f32> = self.y.iter().map(|yi| yi.make_into_f32()).collect();
        let mut fg = Figure::new();
        fg.axes2d()
        .lines_points(&x, &y, &[Caption("A line"), Color("black")]);
        match fg.show() {
            Ok(_) => {},
            Err(e) => {println!("Error: {}", e);}
        }
    }

    fn check(&self) {
        assert_eq!(self.y.first().unwrap().make_into_f32(), 0.0, "First point does not define y = 0.");
        assert_eq!(self.y.last().unwrap().make_into_f32(), 1.0, "Last point does not define y = 1.");
        for i in 0..self.y.len() - 1 {
            let l = &self.y[i];
            let r = &self.y[i + 1];
            assert!(l <= r, "Y does not increase montonously for increasing x.");
        }
    }
}

impl<X, Y> Curve for RegularDynamicCurve<X, Y>
where X: LikeANumber, Y: LikeANumber
{
    fn min_x(&self) -> f32 {
        return self.x0.make_into_f32();
    }

    fn max_x(&self) -> f32
    {
        let len = self.s.make_into_f32() * ((self.y.len() - 1) as f32);
        return self.x0.make_into_f32() + len;
    }

    fn y_at_x(&self, x: f32) -> f32 {
        if x <= self.min_x() {
            return self.y[0].make_into_f32();
        }
        if x >= self.max_x() {
            return self.y[self.y.len() - 1].make_into_f32();
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

        for i in 0..self.y.len() {
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

    // getter for x and y values as vectors, to be used e.g. for plotting multiple curves
    fn get_values_as_vectors(&self) -> (Vec<f32>, Vec<f32>){
        let mut x = Vec::<f32>::new();
        for i in 0..self.y.len() {
            x.push(self.x0.make_into_f32()+(i as f32)*self.s.make_into_f32());
        }
        let y: Vec<f32> = self.y.iter().map(|yi| yi.make_into_f32()).collect();
        return (x, y);
    }

    fn get_x_values(&self) -> Vec<f32> {
        let mut vec: Vec<f32> = Vec::with_capacity(self.y.len());
        let x0 = self.x0.make_into_f32();
        let s = self.s.make_into_f32();
        for i in 0..self.y.len() {
            vec.push(x0 + s * i as f32);
        }
        // TODO maybe use ranges like this: (0..10).step(3);
        // but is this actually efficient, and does it work for floats?
        return vec;
    }

    fn serialize_compact(&self) -> Vec<u8> {
        panic!("Not implemented for this type.");
    }

    fn serialize_compact_limited(&self, _max_bytes: usize) -> Vec<u8> {
        panic!("Not implemented for this type.");
    }
}

impl<X, Y> TypedCurve<X, Y> for RegularDynamicCurve<X, Y>
where X: LikeANumber, Y: LikeANumber
{
    fn typed_min_x(&self) -> X {
        return self.x0;
    }

    fn typed_max_x(&self) -> X
    {
        let len : X = self.s * X::make_from_f32((self.y.len() - 1) as f32);
        return self.x0 + len;
    }

    fn typed_y_at_x(&self, x: X) -> Y {
        if x <= self.x0 {
            return self.y[0];
        }
        if x >= self.typed_max_x() {
            return self.y[self.y.len() - 1];
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

        for i in 0..self.y.len() {
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

impl<X, Y> LeafData for RegularDynamicCurve<X, Y> 
where X: LikeANumber, Y: LikeANumber 
{
    fn get_ext(format: &SerdeFormat) -> &str {
        match format {
            SerdeFormat::Json => "json",
            SerdeFormat::MessagePack => "rcrv"
        }
    }
}

impl<X, Y> Display for RegularDynamicCurve<X, Y> where X: LikeANumber, Y: LikeANumber
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RegularDynamicCurve (min={:>5}, 5%={:>5}, med={:>5}, 95%={:>5}, max={:>5})", 
        self.x_at_y(0.0) as i32, self.x_at_y(0.05) as i32, self.x_at_y(0.5) as i32, self.x_at_y(0.95) as i32, self.x_at_y(1.0) as i32)
    }
}