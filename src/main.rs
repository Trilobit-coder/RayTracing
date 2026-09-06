use std::io::Result;
use std::process;
use std::sync::Arc;

use ray_tracer::hittable::{BvhNode, HittableList, Sphere};
use ray_tracer::optical::material::{Dielectric, Lambertian, Material, Metal};
use ray_tracer::optical::{Camera, camera};
use ray_tracer::utilities::Point3;
use ray_tracer::utilities::random::{random_f32, random_f32_range};
use ray_tracer::utilities::{Color, Vec3};

fn run() -> Result<()> {
    let mut world = HittableList::new();

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground = Sphere::new(Point3::new(0., -1000., 0.), 1000., ground_material);
    world.add(Box::new(ground));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();
            let center = Point3::new(
                a as f32 + 0.9 * random_f32(),
                0.2,
                b as f32 + 0.9 * random_f32(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let center_to = center + Vec3::new(0., random_f32_range(0., 0.5), 0.);
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    let moving_sphere = Sphere::new(center, 0.2, sphere_material).motion(center_to);
                    world.add(Box::new(moving_sphere));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = random_f32_range(0., 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let s_1 = Sphere::new(Point3::new(0., 1., 0.), 1.0, material1);
    world.add(Box::new(s_1));

    let material2: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let s_2 = Sphere::new(Point3::new(-4., 1., 0.), 1.0, material2);
    world.add(Box::new(s_2));

    let material3: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let s_3 = Sphere::new(Point3::new(4., 1., 0.), 1.0, material3);
    world.add(Box::new(s_3));

    world = HittableList::from(Box::new(BvhNode::from_list(world)));

    let camera_config = camera::Config::new()
        .image(16.0 / 9.0, 400)
        .view(
            20.0,
            Point3::new(13., 2., 3.),
            Point3::zero(),
            Vec3::new(0., 1., 0.),
        )
        .focus(0.6, 10.)
        .quality(500, 50);

    let camera = Camera::new(camera_config);

    camera.render(&world)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error writing PPM file: {}", e);
        process::exit(1);
    }
}
