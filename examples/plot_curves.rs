use dystonse_curves::regular_dynamic::RegularDynamicCurve;
use dystonse_curves::irregular_dynamic::*;
use dystonse_curves::{Curve, weighted_average};
use gnuplot::{Figure};
    
fn main() {
    //comment in whatever you want to test.

    //test_plot(); //simple plot with one curve
    //test_multi_curve(); //three curves in one plot
    test_weighted_average(); //two curves and a weighted average between them
}

// this plots a simple example curve with gnuplot (should open a new gnuplot window automatically)
fn test_plot() {

    let c = RegularDynamicCurve::<f32, f32>::new(
        10.0,
        10.0,
        vec!{0.0, 0.6, 1.0}
    );

    c.plot_curve_with_gnuplot();
}

// this plots three curves (two regular, one irregular) in one plot
fn test_multi_curve() {
    let c = RegularDynamicCurve::<f32, f32>::new(
        10.0,
        10.0,
        vec!{0.0, 0.6, 1.0}
    );

    let e = RegularDynamicCurve::<f32, f32>::new(
        5.0,
        10.0,
        vec!{0.0, 0.39, 0.45, 0.7, 1.0}
    );
    
    let f = IrregularDynamicCurve::<f32, f32>::new(
            vec![
                Tup { x: 5.0, y: 0.0},
                Tup { x: 10.0, y: 0.15},
                Tup { x: 12.0, y: 0.2},
                Tup { x: 17.0, y: 0.3},
                Tup { x: 25.0, y: 0.6},
                Tup { x: 30.0, y: 1.0},   
            ]
    );

    let v : Vec<Box<dyn Curve>> = vec!{
        Box::new(f), 
        Box::new(c), 
        Box::new(e), 
    };

    multi_curve_plot(v);
}

// this plots two curves and their weighted average
fn test_weighted_average() {

    let d = RegularDynamicCurve::<f32, f32>::new(
        10.0,
        10.0,
        vec!{0.0, 0.7, 0.8, 0.9, 1.0}
    );
    
    let f = IrregularDynamicCurve::<f32, f32>::new(
            vec![
                Tup { x: 5.0, y: 0.0},
                Tup { x: 10.0, y: 0.15},
                Tup { x: 12.0, y: 0.2},
                Tup { x: 17.0, y: 0.3},
                Tup { x: 25.0, y: 0.6},
                Tup { x: 30.0, y: 1.0},   
            ]
    );

    let df = weighted_average(&d, 0.7, &f, 0.3);
    //let ce = weighted_average(&c, 0.9, &e, 0.1);

    let v : Vec<Box<dyn Curve>> = vec!{
        Box::new(d), 
        Box::new(f), 
        Box::new(df), 
        //Box::new(c), 
        //Box::new(e), 
        //Box::new(ce)
    };

    multi_curve_plot(v);
}

fn multi_curve_plot(curves: Vec<Box<dyn Curve>>) {
    let mut fg = Figure::new();
    let axes = fg.axes2d();
    for c in curves {
        let vecs = c.get_values_as_vectors();
        let x = vecs.0;
        let y = vecs.1;
        axes.lines_points(&x, &y, &[]);
    }
    fg.show();
}