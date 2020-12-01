mod lib;
mod color;

fn main() {
    let width = 256;
    let height = 256;
    let mut output = format!("P3\n{} {}\n255\n", width, height);

    for j in (0..height).rev() {
        println!("Scanlines Remaining: {}", j);
        for i in 0..width {
            let color = lib::Color::new((i as f64) / (width-1) as f64, (j as f64) / (height-1) as f64 , 0.25);
            let together = color::write_color(&color);
            output.push_str(&together);
        }
    }
    std::fs::write("image.ppm", output).unwrap();
    println!("Done");
}
