use dystonse_curves::regular_dynamic::RegularDynamicCurve;
    
fn main() {
    test_plot();
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
