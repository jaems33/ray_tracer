mod lib;
mod color;
mod ray;
mod utils;
mod hittable;
mod hittable_list;

fn hit_sphere(center: &lib::Point3, radius: f64, r: &ray::Ray) -> f64 {
    let oc: lib::Vec3 = r.origin() - center.clone();
    let a = r.direction().length_squared();
    let half_b = lib::dot(&oc, &r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    match discriminant < 0.0 {
        true => {
            -1.0
        }
        false => {
            (-half_b - discriminant.sqrt()) / a
        }
    }
}


fn ray_color(r: ray::Ray) -> lib::Color {
    let t = hit_sphere(&lib::Point3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let N = lib::unit_vector(&(r.at(t) - lib::Vec3::new(0.0, 0.0, -1.0)));
        return 0.5 * lib::Color::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
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
