// Main file for bin crate 

extern crate steelsight;

use steelsight::*;
use steelsight::geometry::*;
use steelsight::camera::*;

use rand::prelude::*;

fn write_color(color: Color) {
    let (r,g,b) = color.to_rgb();

    // Gamma-correct
    let r = r.sqrt();
    let g = g.sqrt();
    let b = b.sqrt();

    let (rr,gg,bb) =
        ((256.0 * clamp(r, 0.0, 0.999)) as u8,
         (256.0 * clamp(g, 0.0, 0.999)) as u8,
         (256.0 * clamp(b, 0.0, 0.999)) as u8);
    println!("{} {} {}", rr, gg, bb);
}

fn ray_color<R: Rng + ?Sized>(rng : &mut R, world: &impl Geometry, ray: Ray, depth: i32) -> Color {

    if depth <= 0 { return Color::from_rgb(0.0, 0.0, 0.0) }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        
        let scatter_target = hit.p + hit.normal + random::unit_vector(rng);
        let scatter_ray = Ray::through_points(hit.p, scatter_target);
        0.5 * ray_color(rng, world, scatter_ray, depth-1)


    } else {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * unit_direction.y + 1.0;
        let a = Color::from_rgb(1.0, 1.0, 1.0);
        let b = Color::from_rgb(0.5, 0.7, 1.0);
        (1.0 - t) * a + t * b
    }
}

fn main() {
    let mut rng = thread_rng();

    let samples_per_pixel = 100;
    let max_depth = 50;

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
            let mut total_color = Color::from_rgb(0.0, 0.0, 0.0);

            for _sample in 0..samples_per_pixel {
                let s = (i as float + rng.gen::<float>()) / ((image_width - 1) as float);
                let t = (j as float + rng.gen::<float>()) / ((image_height - 1) as float);

                let r = camera.get_ray(s, t);
                total_color = total_color + ray_color(&mut rng, &world, r, max_depth);
            }

            let avg_color = (1.0 / samples_per_pixel as float) * total_color;
            write_color(avg_color);
        }
    }
    eprintln!("\nDone");
}
