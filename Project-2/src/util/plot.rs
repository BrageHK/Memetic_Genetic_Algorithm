use std::f64::consts::PI;
use std::fs;
use serde_json::from_str;
use plotters::prelude::*;
use plotters::coord::types::RangedCoordf64;
use crate::structs::config::Config;
use crate::structs::io::{read_from_json, Info};
use plotters::style::RGBColor;

fn draw_arrow(
    chart: &mut ChartContext<BitMapBackend,
    Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    (x1, y1): (i32, i32),
    (x2, y2): (i32, i32),
    color: &RGBColor
    ) -> Result<(), Box<dyn std::error::Error>> {
    let angle = ((y2 - y1) as f64).atan2((x2 - x1) as f64);
    let arrowhead_length = 1.0;

    // Arrowhead points
    let left_x = x2 as f64 - arrowhead_length * (angle + PI / 6.0).cos();
    let left_y = y2 as f64 - arrowhead_length * (angle + PI / 6.0).sin();
    let right_x = x2 as f64 - arrowhead_length * (angle - PI / 6.0).cos();
    let right_y = y2 as f64 - arrowhead_length * (angle - PI / 6.0).sin();

    chart.draw_series(LineSeries::new(vec![(x1 as f64, y1 as f64), (x2 as f64, y2 as f64)], color)).unwrap();
    chart.draw_series(LineSeries::new(vec![(x2 as f64, y2 as f64), (left_x, left_y)], color)).unwrap();
    chart.draw_series(LineSeries::new(vec![(x2 as f64, y2 as f64), (right_x, right_y)], color)).unwrap();

    Ok(())
}

pub fn plot_points(individual: &Vec<Vec<i32>>, info: &Info) {
    let file_name = "plots/".to_owned() + &*info.instance_name.clone() + ".png";
    let root = BitMapBackend::new(&file_name, (1000, 1000))
            .into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Compute min/max X and Y values dynamically
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for nurse in individual {
        for &patient in nurse {
            let p_info = &info.patients[patient as usize - 1];
            min_x = min_x.min(p_info.x_coord as f32);
            max_x = max_x.max(p_info.x_coord as f32);
            min_y = min_y.min(p_info.y_coord as f32);
            max_y = max_y.max(p_info.y_coord as f32);
        }
    }

    // Add padding to the range to make it look better
    let padding_x = (max_x - min_x) * 0.1;
    let padding_y = (max_y - min_y) * 0.1;

    let mut chart = ChartBuilder::on(&root)
        .caption(info.instance_name.to_string()+" Graph", ("sans-serif", 50))
        .margin(30) // Increase margin for centering
        .build_cartesian_2d(
            (min_x - padding_x) as f64..(max_x + padding_x) as f64,
            (min_y - padding_y) as f64..(max_y + padding_y) as f64,
        )
        .unwrap();
    chart.configure_mesh().draw().unwrap();

    for nurse in individual {
        for patient in nurse{
            let p_info = info.patients[*patient as usize-1];
            chart
                .draw_series(
                    std::iter::once(Circle::new((p_info.x_coord as f64, p_info.y_coord as f64), 3, &BLUE))
                ).unwrap();
        }
    }

    let colors: Vec<RGBColor> = generate_colors(info.nbr_nurses as usize);

    let depot_x = info.depot.x_coord;
    let depot_y = info.depot.y_coord;
    for (i, nurse) in individual.iter().enumerate() {
        if nurse.is_empty() {
            continue;
        }
        let mut prev_patient = (depot_x, depot_y);
        for patient in nurse {
            let p_info = info.patients[*patient as usize-1];
            draw_arrow(&mut chart, (prev_patient.0, prev_patient.1), (p_info.x_coord, p_info.y_coord), &colors[i]).unwrap();
            prev_patient = (p_info.x_coord, p_info.y_coord);
        }
        draw_arrow(&mut chart, (prev_patient.0, prev_patient.1), (depot_x, depot_y), &colors[i]).unwrap();
    }

    root.present().unwrap();
}

pub fn plot_best_individual() {
    let info = read_from_json(&Config::new("config/config.yaml")).unwrap();
    let folder_path = "./individuals"; // Change to your actual folder path

    // Find the file with the smallest numerical name
    let mut min_file: f32 = f32::INFINITY;
    let mut parsed_vec: Vec<Vec<i32>> = Vec::new();

    for entry in fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        let filename = entry.file_name();
        let file_num: f32 = filename.clone().into_string().unwrap().parse().unwrap();

        if file_num <  min_file {
            min_file = file_num;
            let fp = format!("{}/{}", folder_path, &filename.to_str().unwrap());
            let file_content = fs::read_to_string(&fp).unwrap();
            parsed_vec = from_str(&file_content).unwrap();
        }
    }

    plot_points(&parsed_vec, &info);
}
fn generate_colors(n: usize) -> Vec<RGBColor> {
    let golden_ratio_conjugate = 0.61803398875; // Helps spread colors better
    let mut hue = 0.0;

    (0..n)
        .map(|_| {
            hue = (hue + golden_ratio_conjugate * 360.0) % 360.0; // Avoid clustering
            hsl_to_rgb(hue, 0.85, 0.55) // Higher saturation & balanced brightness
        })
        .collect()
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> RGBColor {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    RGBColor(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}