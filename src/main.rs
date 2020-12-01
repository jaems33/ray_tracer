fn main() {
    let image_width = 256;
    let image_height = 256;

    let width = &image_width;
    let height = &image_height;

    let num = 255.999;

    let mut output = format!("P3\n{} {}\n255\n", width, height);

    for j in (0..image_height).rev() {
        println!("Scanlines Remaining: {}", j);
        for i in 0..image_width {
            let red = (i as f64) / (width-1) as f64;
            let green = (j as f64) / (height-1) as f64;
            let blue = 0.25;

            let ir = (num * red) as i32;
            let ig = (num * green) as i32;
            let ib = (num * blue) as i32;

            let together = format!("{} {} {} \n", ir, ig, ib);
            output.push_str(&together);
        }
    }
    std::fs::write("image.ppm", output).unwrap();
    println!("Done");
}
