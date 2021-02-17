// Main file for bin crate 

extern crate steelsight;

use steelsight::*;
use steelsight::geometry::*;
use steelsight::camera::*;
use steelsight::materials::*;

use std::rc::Rc;

fn random_scene() -> impl Geometry {
    let mut rng = rand_pcg::Pcg32::seed_from_u64(42);

    let mut world = GeometryList::new();

    let ground_material = Rc::new(Lambertian { albedo: Color::from_rgb(0.5, 0.5, 0.5) });
    
    world.push(Sphere {
        center: vec3(0.0, -1000.0, 0.0), 
        radius: 1000.0, 
        material: ground_material
    });

    for a in -11..11 {
        for b in -11..11 {
            let mat_choice: float = rng.gen();

            let center = vec3(
                (a as float) + 0.9*rng.gen::<float>(),
                0.2,
                (b as float) + 0.9*rng.gen::<float>());

            if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if mat_choice < 0.8 {
                    // diffuse
                    let albedo = random::color(0.0, 1.0, &mut rng) * random::color(0.0, 1.0, &mut rng);
                    let material = Rc::new(Lambertian { albedo });
                    world.push(Sphere {
                        center, radius: 0.2, material
                    })
                } else if mat_choice < 0.95 {
                    // metal
                    let albedo = random::color(0.5, 1.0, &mut rng);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let material = Rc::new(Metal { albedo, fuzz });
                    world.push(Sphere {
                        center, radius: 0.2, material
                    })
                } else {
                    // glass
                    let material = Rc::new(Dielectric { eta: 1.5 });
                    world.push(Sphere {
                        center, radius: 0.2, material
                    })
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric { eta: 1.5 });
    world.push(Sphere {
        center: vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1
    });

    let material2 = Rc::new(Lambertian { 
        albedo: Color::from_rgb(0.4, 0.2, 0.1)
    });
    world.push(Sphere {
        center: vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2
    });

    let material3 = Rc::new(Metal {
        albedo: Color::from_rgb(0.7, 0.6, 0.5),
        fuzz: 0.0
    });
    world.push(Sphere {
        center: vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3
    });

    world
}

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
        // Background
        let unit_direction = ray.direction.unit();
        let t = 0.5 * unit_direction.y + 1.0;
        let a = Color::from_rgb(1.0, 1.0, 1.0);
        let b = Color::from_rgb(0.5, 0.7, 1.0);
        (1.0 - t) * a + t * b
    }
}

fn main() {
    let mut rng: Random = thread_rng();

    let samples_per_pixel = 500;
    let max_depth = 24;

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width: i32 = 1200;
    let image_height = ((image_width as float) / aspect_ratio) as i32;

    // Make world
    let world = random_scene();
    
    // Camera
    let lookfrom = vec3(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let camera_args = CameraArgs {
        vup : vec3(0.0, 1.0, 0.0),
        vfow_degrees: 20.0,
        aperture: 0.1,
        lookfrom,
        lookat,
        focus_dist,
        aspect_ratio
    };
    let camera = Camera::new(camera_args);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\nScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut total_color = Color::from_rgb(0.0, 0.0, 0.0);

            for _sample in 0..samples_per_pixel {
                let s = (i as float + rng.gen::<float>()) / ((image_width - 1) as float);
                let t = (j as float + rng.gen::<float>()) / ((image_height - 1) as float);

                let r = camera.get_ray(s, t, &mut rng);
                total_color = total_color + ray_color(&world, r, max_depth, &mut rng);
            }

            let avg_color = (1.0 / samples_per_pixel as float) * total_color;
            write_color(avg_color);
        }
    }
    eprintln!("\nDone");
}
