// Main file for bin crate 

extern crate steelsight;

use steelsight::*;
use steelsight::geometry::*;
use steelsight::camera::*;
use steelsight::materials::*;

fn write_color(color: Color) {
    let (r,g,b) = color.to_rgb();

    // Gamma correction 
    let r = r.sqrt();
    let g = g.sqrt();
    let b = b.sqrt();

    let (rr,gg,bb) =
        ((256.0 * clamp(r, 0.0, 0.999)) as u8,
         (256.0 * clamp(g, 0.0, 0.999)) as u8,
         (256.0 * clamp(b, 0.0, 0.999)) as u8);
    println!("{} {} {}", rr, gg, bb);
}

fn ray_color(world: &impl Geometry, ray: Ray, depth: i32, rng : &mut Random) -> Color {

    if depth <= 0 { 
        return Color::from_rgb(0.0, 0.0, 0.0)
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        
        if let Some(scatter) = hit.material.scatter(ray, &hit, rng) {
            scatter.attenuation * ray_color(world, scatter.scattered_ray, depth - 1, rng)
        } else {
            Color::from_rgb(0.0, 0.0, 0.0)
        }


    } else {

        let unit_direction = ray.direction.unit();
        let t = 0.5 * unit_direction.y + 1.0;
        let a = Color::from_rgb(1.0, 1.0, 1.0);
        let b = Color::from_rgb(0.5, 0.7, 1.0);
        (1.0 - t) * a + t * b

    }
}

fn main() {
    let mut rng: Random = thread_rng();

    let samples_per_pixel = 100;
    let max_depth = 12;

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

    // Materials
    let material_ground = Lambertian { albedo: Color::from_rgb(0.8, 0.8, 0.0) };
    let material_center = Lambertian { albedo: Color::from_rgb(0.1, 0.2, 0.5) };
    let material_left = Dielectric { eta: 1.5 };
    let material_right = Metal { albedo: Color::from_rgb(0.8, 0.6, 0.2), fuzz: 1.0 };

    // Make world
    let mut world = GeometryList::new();
    world.push(Sphere { center: vec3(0.0,-100.5,-1.0), radius: 100.0, material: &material_ground });
    world.push(Sphere { center: vec3(0.0,0.0,-1.0), radius: 0.5, material: &material_center });
    // Negative radius means surface normals point inwards!
    world.push(Sphere { center: vec3(-1.0,0.0,-1.0), radius: -0.4, material: &material_left }); 
    world.push(Sphere { center: vec3(1.0,0.0,-1.0), radius: 0.5, material: &material_right });

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
                total_color = total_color + ray_color(&world, r, max_depth, &mut rng);
            }

            let avg_color = (1.0 / samples_per_pixel as float) * total_color;
            write_color(avg_color);
        }
    }
    eprintln!("\nDone");
}
