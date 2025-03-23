use std::f64::consts::PI;
use std::fs;

use serde_json::from_str;

use plotters::prelude::*;
use plotters::coord::types::RangedCoordf64;
use plotters::style::RGBColor;

use crate::structs::config::Config;
use crate::structs::io::{read_from_json, Info};
use crate::util::print::print_best_solution;

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
    let root = BitMapBackend::new(&file_name, (1200, 1200)).into_drawing_area();
    root.fill(&WHITE).unwrap();

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

    let padding_x = (max_x - min_x) * 0.08;
    let padding_y = (max_y - min_y) * 0.08;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("{} - Nurse Routing Solution", info.instance_name),
            ("sans-serif", 40).into_font(),
        )
        .margin(50)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            (min_x - padding_x) as f64..(max_x + padding_x) as f64,
            (min_y - padding_y) as f64..(max_y + padding_y) as f64,
        )
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("X pos")
        .y_desc("Y pos")
        .axis_desc_style(("sans-serif", 20))
        .draw()
        .unwrap();

    // Plot patient points
    for nurse in individual {
        for patient in nurse {
            let p_info = info.patients[*patient as usize - 1];
            let _ = chart
                .draw_series(std::iter::once(Circle::new(
                    (p_info.x_coord as f64, p_info.y_coord as f64),
                    5,
                    &BLUE.mix(0.7),
                )));
        }
    }

    let colors = generate_colors();

    let depot_x = info.depot.x_coord;
    let depot_y = info.depot.y_coord;

    // Plot routes with arrows and colors
    for (i, nurse) in individual.iter().enumerate() {
        if nurse.is_empty() {
            continue;
        }
        let mut prev_patient = (depot_x, depot_y);
        for patient in nurse {
            let p_info = info.patients[*patient as usize - 1];
            draw_arrow(
                &mut chart,
                (prev_patient.0, prev_patient.1),
                (p_info.x_coord, p_info.y_coord),
                &colors[i % colors.len()],
            )
                .unwrap();
            prev_patient = (p_info.x_coord, p_info.y_coord);
        }
        draw_arrow(
            &mut chart,
            (prev_patient.0, prev_patient.1),
            (depot_x, depot_y),
            &colors[i % colors.len()],
        )
            .unwrap();
    }

    root.present().unwrap();
}
pub fn plot_best_individual() {
    let info = read_from_json(&Config::new("config/config.yaml")).unwrap();
    let config = Config::new("./config/config.yaml");
    let folder_path = "./individuals/".to_string() + &*config.file_name + "/";
    fs::create_dir_all(&folder_path).unwrap();

    // Find the file with the smallest numerical name
    let mut min_file: f32 = f32::INFINITY;
    let mut parsed_vec: Vec<Vec<i32>> = Vec::new();
    if fs::read_dir(&folder_path).unwrap().next().is_none() {
        return;
    }

    for entry in fs::read_dir(&folder_path).unwrap() {
        let entry = entry.unwrap();
        let filename = entry.file_name();
        let file_num= filename.to_str().unwrap();
        let file_num: f32 = file_num.parse::<f32>().unwrap();

        if file_num <  min_file {
            min_file = file_num;
            let fp = format!("{}/{}", folder_path, &filename.to_str().unwrap());
            let file_content = fs::read_to_string(&fp).unwrap();
            parsed_vec = from_str(&file_content).unwrap();
        }
    }

    plot_points(&parsed_vec, &info);
    print_best_solution(parsed_vec, &info);
    println!("Made a plot for: {}", &*config.file_name)
}

fn generate_colors() -> Vec<RGBColor> {
    vec![
        RGBColor { 0: 255, 1: 0, 2: 0 },       // Red
        RGBColor { 0: 0, 1: 255, 2: 0 },       // Green
        RGBColor { 0: 0, 1: 0, 2: 255 },       // Blue
        RGBColor { 0: 255, 1: 255, 2: 0 },     // Yellow
        RGBColor { 0: 255, 1: 0, 2: 255 },     // Ma1enta
        RGBColor { 0: 0, 1: 255, 2: 255 },     // Cyan
        RGBColor { 0: 128, 1: 0, 2: 0 },       // Maroon
        RGBColor { 0: 0, 1: 128, 2: 0 },       // Dark Green
        RGBColor { 0: 0, 1: 0, 2: 128 },       // Navy
        RGBColor { 0: 128, 1: 128, 2: 0 },     // Olive
        RGBColor { 0: 128, 1: 0, 2: 128 },     // Purple
        RGBColor { 0: 0, 1: 128, 2: 128 },     // Teal
        RGBColor { 0: 192, 1: 192, 2: 192 },   // Silver
        RGBColor { 0: 128, 1: 128, 2: 128 },   // Gray
        RGBColor { 0: 255, 1: 165, 2: 0 },     // Oran1e
        RGBColor { 0: 255, 1: 192, 2: 203 },   // Pink
        RGBColor { 0: 165, 1: 42, 2: 42 },     // Brown
        RGBColor { 0: 0, 1: 255, 2: 127 },     // Sprin1 Green
        RGBColor { 0: 70, 1: 130, 2: 180 },    // Steel Blue
        RGBColor { 0: 255, 1: 99, 2: 71 },     // Tomato
        RGBColor { 0: 147, 1: 112, 2: 219 },   // Medium Purple
        RGBColor { 0: 255, 1: 215, 2: 0 },     // Gold
        RGBColor { 0: 0, 1: 128, 2: 128 },     // Dark Cyan
        RGBColor { 0: 255, 1: 140, 2: 0 },     // Dark Oran1e
        RGBColor { 0: 75, 1: 0, 2: 130 },      // Indi1o
    ]
}