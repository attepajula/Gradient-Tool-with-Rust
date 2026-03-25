use serde::{Deserialize, Serialize};

use crate::color::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientStop {
    pub position: f32,
    pub hex: String,
}

/// Generate `steps` evenly-spaced gradient stops by interpolating across `colors`.
///
/// Colors are treated as evenly-distributed anchor points along the 0.0–1.0 range.
pub fn generate_stops(colors: &[Color], steps: usize) -> Vec<GradientStop> {
    if colors.is_empty() {
        return vec![];
    }
    if colors.len() == 1 || steps <= 1 {
        return vec![GradientStop {
            position: 0.0,
            hex: colors[0].to_hex(),
        }];
    }

    let segments = colors.len() - 1;

    (0..steps)
        .map(|i| {
            let t = i as f32 / (steps - 1) as f32; // 0.0 ..= 1.0
            let scaled = t * segments as f32;
            let seg_idx = (scaled as usize).min(segments - 1);
            let seg_t = scaled - seg_idx as f32;
            let color = colors[seg_idx].lerp(colors[seg_idx + 1], seg_t);
            GradientStop {
                position: t,
                hex: color.to_hex(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_colors_endpoints_match() {
        let colors = vec![Color::new(0, 0, 0), Color::new(255, 255, 255)];
        let stops = generate_stops(&colors, 5);
        assert_eq!(stops.first().unwrap().hex, "#000000");
        assert_eq!(stops.last().unwrap().hex, "#ffffff");
        assert_eq!(stops.len(), 5);
    }

    #[test]
    fn single_color_returns_one_stop() {
        let colors = vec![Color::new(255, 0, 128)];
        let stops = generate_stops(&colors, 10);
        assert_eq!(stops.len(), 1);
        assert_eq!(stops[0].position, 0.0);
    }

    #[test]
    fn empty_colors_returns_empty() {
        assert!(generate_stops(&[], 5).is_empty());
    }
}
