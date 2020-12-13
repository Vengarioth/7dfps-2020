use rand::Rng;
use bevy::prelude::*;

pub fn get_color_palette() -> Vec<Color> {
    let mut rng = rand::thread_rng();
    let num_of_colors: i32 = 3;
    let angle: f32 = 105.0; 
    let saturation: f32 = rng.gen_range(25.0, 90.0); 
    let lightness: f32 = 90.0;
    let angle_of_separation = angle/num_of_colors as f32;
    let start_angle = rng.gen_range(0.0, 360.0-angle_of_separation*num_of_colors as f32);
    let mut palette = Vec::new();
    for i in 0..num_of_colors {
        palette.push(hsl_to_rgba_color(start_angle+angle_of_separation*i as f32, rng.gen_range(25.0, saturation), lightness));
    }
    palette
}

fn hsl_to_rgba_color(h: f32, s: f32, l: f32) -> Color {
    let r: f32;
    let g: f32;
    let b: f32;
    let a = 255.0;

    if s == 0.0 {
        return Color::rgba(l, l, l, a);
    }

    let s = s/100.0;
    let l = l/100.0;

    let c = (1.0 - (2.0*l-1.0).abs())*s;
    let new_h = h/60.0;
    let x = c*(1.0-(new_h%2.0-1.0).abs());

    if h >= 0.0 && h <= 1.0 {
        r = c;
        g = x;
        b = 0.0;
    } else if h > 1.0 && h <= 2.0 {
        r = x;
        g = c;
        b = 0.0;
    } else if h > 2.0 && h <= 3.0 {
        r = 0.0;
        g = c;
        b = x;
    } else if h > 3.0 && h <= 4.0 {
        r = 0.0;
        g = x;
        b = c;
    } else if h > 4.0 && h <= 5.0 {
        r = x;
        g = 0.0;
        b = c;
    } else if h > 5.0 && h <= 6.0 {
        r = c;
        g = 0.0;
        b = x;
    } else {
        r = 0.0;
        g = 0.0;
        b = 0.0;
    }

    let m = l - c/2.0;
    Color::rgba(r+m, g+m, b+m, a)
}