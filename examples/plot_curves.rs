use dystonse_curves::regular_dynamic::RegularDynamicCurve;
use dystonse_curves::Curve;
use gnuplot::{Figure};
    
fn main() {
    //comment in whatever you want to test.

    //test_plot(); //simple plot with one curve
    test_multi_curve(); //three curves in one plot
}

// this plots a simple example curve with gnuplot (should open a new gnuplot window automatically)
fn test_plot() {

    let c = RegularDynamicCurve::<f32, f32>::new(
        3,
        10.0,
        10.0,
        vec!{0.0, 0.6, 1.0}
    );

    c.plot_curve_with_gnuplot();
}

fn test_multi_curve() {
    let c = RegularDynamicCurve::<f32, f32>::new(
        3,
        10.0,
        10.0,
        vec!{0.0, 0.6, 1.0}
    );

    let d = RegularDynamicCurve::<f32, f32>::new(
        3,
        10.0,
        10.0,
        vec!{0.7, 0.8, 0.9}
    );

    let e = RegularDynamicCurve::<f32, f32>::new(
        5,
        5.0,
        10.0,
        vec!{0.1, 0.39, 0.45, 0.7, 1.0}
    );

    let v = vec!{c, d, e};

    multi_curve_plot(v);
}

fn multi_curve_plot(curves: Vec<RegularDynamicCurve<f32, f32>>) {
    let mut fg = Figure::new();
    let mut axes = fg.axes2d();
    for c in curves {
        let vecs = c.get_values_as_vectors();
        let x = vecs.0;
        let y = vecs.1;
        axes.lines_points(&x, &y, &[]);
    }
    fg.show();

}
