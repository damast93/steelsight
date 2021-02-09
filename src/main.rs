// Main file for bin crate 

extern crate steelsight;

use steelsight::*;
use steelsight::geometry::*;
use steelsight::camera::*;

fn write_color(color: Color) {
    let (r, g, b) = color.to_rgb_bytes();
    println!("{} {} {}", r, g, b);
}

fn ray_color(world: &impl Geometry, ray: Ray) -> Color {
    if let Some(hit) = world.hit(ray, 0.0, f64::INFINITY) {
        0.5 * Color::from_rgb(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0)
    } else {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * unit_direction.y + 1.0;
        let a = Color::from_rgb(1.0, 1.0, 1.0);
        let b = Color::from_rgb(0.5, 0.7, 1.0);
        (1.0 - t) * a + t * b
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height = ((image_width as float) / aspect_ratio) as i32;

    // Camera
    let camera_args = CameraArgs {
        origin: vec3(0.0, 0.0, 0.0),
        viewport_height: 2.0,
        focal_length: 1.0,
        aspect_ratio,
    };
    let camera = Camera::new(camera_args);

    // Make world
    let mut world = GeometryList::new();
    world.push(Sphere { center: vec3(0.0,0.0,-1.0), radius: 0.5 });
    world.push(Sphere { center: vec3(0.0,-100.5,-1.0), radius: 100.0 });

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\nScanlines remaining: {}", j);
        for i in 0..image_width {
            let s = (i as float) / ((image_width - 1) as float);
            let t = (j as float) / ((image_height - 1) as float);

            let r = camera.get_ray(s, t);
            let pixel_color = ray_color(&world, r);
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone");
}
