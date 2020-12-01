mod lib;
mod color;
mod ray;

fn hit_sphere(center: &lib::Point3, radius: f64, r: &ray::Ray) -> bool {
    let oc = r.origin() - center.clone();
    let a = lib::dot(&r.direction(), &r.direction());
    let b = 2.0 * lib::dot(&oc, &r.direction());
    let c = lib::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_color(r: ray::Ray) -> lib::Color {
    if hit_sphere(&lib::Point3::new(0.0, 0.0, -1.0), 0.5, &r) {
        return lib::Color::new(1.0, 0.0, 0.0)
    }
    let unit_direction = lib::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0-t) * lib::Color::new(1.0, 1.0, 1.0) + t * lib::Color::new(0.5, 0.7, 1.0)
}

fn main() {

    /* Image */
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    /* Camera */
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = lib::Point3::new(0.0, 0.0, 0.0);
    let horizontal = lib::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = lib::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: lib::Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - lib::Vec3::new(0.0, 0.0, focal_length);

    let mut output = format!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        println!("Scanlines Remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width-1) as f64;
            let v = j as f64 / (image_height-1) as f64;
            let r = ray::Ray::new(&origin, &(lower_left_corner + (u * horizontal) + (v * vertical - origin)));
            let pixel_color = ray_color(r);
            let together = color::write_color(&pixel_color);
            output.push_str(&together);
        }
    }
    std::fs::write("image.ppm", output).unwrap();
    println!("Done");
}
