use image::{ImageOutputFormat, RgbImage};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use crate::color::Color;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Paradigm {
    #[default]
    Linear,
    Diagonal,
    Radial,
    Reflected,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Warp {
    #[default]
    None,
    EaseIn,
    EaseOut,
    EaseInOut,
    Wave,
    Zigzag,
}

/// Control points in normalized image coordinates (0.0–1.0).
/// Point A = gradient start / radial center.
/// Point B = gradient end / radial edge.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GradientPoints {
    pub ax: f32,
    pub ay: f32,
    pub bx: f32,
    pub by: f32,
}

impl GradientPoints {
    pub fn default_for(paradigm: Paradigm) -> Self {
        match paradigm {
            Paradigm::Linear | Paradigm::Reflected => Self { ax: 0.0, ay: 0.5, bx: 1.0, by: 0.5 },
            Paradigm::Diagonal => Self { ax: 0.0, ay: 0.0, bx: 1.0, by: 1.0 },
            Paradigm::Radial   => Self { ax: 0.5, ay: 0.5, bx: 1.0, by: 0.5 },
        }
    }
}

pub fn render_jpeg(
    stops: &[(f32, Color)],
    width: u32,
    height: u32,
    quality: u8,
    paradigm: Paradigm,
    warp: Warp,
    points: GradientPoints,
) -> Vec<u8> {
    let mut img = RgbImage::new(width, height);

    let ax = points.ax * width as f32;
    let ay = points.ay * height as f32;
    let bx = points.bx * width as f32;
    let by = points.by * height as f32;
    let dx = bx - ax;
    let dy = by - ay;
    let len2 = (dx * dx + dy * dy).max(f32::EPSILON);
    let radius = len2.sqrt();

    for y in 0..height {
        for x in 0..width {
            let px = x as f32 - ax;
            let py = y as f32 - ay;

            let raw_t: f32 = match paradigm {
                Paradigm::Linear | Paradigm::Diagonal => {
                    (px * dx + py * dy) / len2
                }
                Paradigm::Radial => {
                    (px * px + py * py).sqrt() / radius
                }
                Paradigm::Reflected => {
                    let t = (px * dx + py * dy) / len2;
                    1.0 - (2.0 * t - 1.0).abs()
                }
            };

            let t = apply_warp(raw_t.clamp(0.0, 1.0), warp);
            let color = sample_stops(stops, t);
            img.put_pixel(x, y, image::Rgb([color.r, color.g, color.b]));
        }
    }

    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageOutputFormat::Jpeg(quality))
        .expect("JPEG encoding should not fail for a valid RgbImage");
    buf.into_inner()
}

fn apply_warp(t: f32, warp: Warp) -> f32 {
    match warp {
        Warp::None => t,
        Warp::EaseIn => t * t * t,
        Warp::EaseOut => 1.0 - (1.0 - t).powi(3),
        Warp::EaseInOut => t * t * (3.0 - 2.0 * t),
        Warp::Wave => (t * std::f32::consts::TAU * 2.0).sin() * 0.5 + 0.5,
        Warp::Zigzag => {
            let t = (t * 4.0) % 2.0;
            if t < 1.0 { t } else { 2.0 - t }
        }
    }
}

fn sample_stops(stops: &[(f32, Color)], t: f32) -> Color {
    if stops.is_empty() { return Color::new(0, 0, 0); }
    if stops.len() == 1 { return stops[0].1; }
    if t <= stops[0].0 { return stops[0].1; }
    if t >= stops[stops.len() - 1].0 { return stops[stops.len() - 1].1; }
    let upper = stops.partition_point(|(pos, _)| *pos <= t);
    let (p1, c1) = stops[upper - 1];
    let (p2, c2) = stops[upper];
    c1.lerp(c2, (t - p1) / (p2 - p1).max(f32::EPSILON))
}
