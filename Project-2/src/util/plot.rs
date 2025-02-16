use plotters::prelude::*;
use crate::genetic::initialize_population::Point;

pub fn plot_points(patients: &Vec<Point>) {
    let root = BitMapBackend::new("example.png", (1000, 1000))
            .into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("X-Y graph", ("sans-serif", 50))
        .margin(15)
        .build_cartesian_2d(-100.0..100.0, -100.0..100.0).unwrap();

    chart.configure_mesh().draw().unwrap();

    for patient in patients {
        chart
            .draw_series(
                std::iter::once(Circle::new((patient.x as f64, patient.y as f64), 3, &BLUE))
            ).unwrap();
    }
    root.present().unwrap();
}
