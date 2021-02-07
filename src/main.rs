mod vec3;
mod color;
mod ray;

use vec3::*;
use color::*;
use ray::*;

fn write_color(color : Color) {
    let (r,g,b) = color.to_rgb_bytes();
    println!("{} {} {}", r, g, b);
}

fn ray_color(r : Ray) -> Color {
    let unit_direction = r.direction.unit();
    let t = 0.5 * unit_direction.y + 1.0;
    let a = Color::from_rgb(1.0, 1.0, 1.0);
    let b = Color::from_rgb(0.5, 0.7, 1.0);
    (1.0-t)*a + t*b
}

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width : i32 = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = vec3(0.0, 0.0, 0.0);
    let horizontal = vec3(viewport_width, 0.0, 0.0);
    let vertical = vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vec3(0.0, 0.0, focal_length);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\nScanlines remaining: {}", j);
        for i in 0..image_width {
            let s = (i as f64) / ((image_width-1) as f64);
            let t = (j as f64) / ((image_height-1) as f64);
            
            let r = Ray::through_points(origin, lower_left_corner + s*horizontal + t*vertical);

            let pixel_color = ray_color(r);
            write_color(pixel_color);
        }
    }
    
    eprintln!("\nDone");
}