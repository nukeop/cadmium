use std::fs;

use image::Rgb;

#[derive(Debug)]
pub struct Palette {
    colors: Vec<Rgb<u8>>,
}

trait Color {
    fn distance(&self, other: &Self) -> f32;
}

impl Color for Rgb<u8> {
    fn distance(&self, other: &Self) -> f32 {
        let r = (self[0] >> 2 << 2) as f32 - other[0] as f32;
        let g = (self[1] >> 2 << 2) as f32 - other[1] as f32;
        let b = (self[2] >> 2 << 2) as f32 - other[2] as f32;
        (r * r + g * g + b * b).sqrt()
    }
}

impl Palette {
    pub fn from_file(file_path: &str) -> Palette {
        let palette_contents =
            fs::read_to_string(file_path).expect("Something went wrong reading the file");
        let palette_colors: Vec<&str> = palette_contents.split("\n").collect();

        let hex_values = palette_colors
            .iter()
            .filter_map(|c| -> Option<Rgb<u8>> {
                if c.len() == 0 || c.chars().nth(0).unwrap() == '/' {
                    return None;
                } else {
                    return Some(Rgb([
                        u8::from_str_radix(&c[1..3], 16).unwrap(),
                        u8::from_str_radix(&c[3..5], 16).unwrap(),
                        u8::from_str_radix(&c[5..7], 16).unwrap(),
                    ]));
                };
            })
            .collect();

        Palette { colors: hex_values }
    }

    pub fn get_closest_color(&self, color: &Rgb<u8>) -> Rgb<u8> {
        let mut closest_color = self.colors[0];
        let mut closest_distance = color.distance(&closest_color);
        for palette_color in self.colors.iter() {
            let distance = color.distance(palette_color);
            if distance < closest_distance {
                closest_color = *palette_color;
                closest_distance = distance;
            }
        }
        closest_color
    }
}
