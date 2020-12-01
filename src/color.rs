use super::lib::{Color};

pub fn write_color(color: &Color) -> String {
    let adjust: f64 = 255.999;
    format!("{} {} {} \n", (color.x() * adjust) as i32, (color.y() * adjust) as i32, (color.z() * adjust) as i32)
}