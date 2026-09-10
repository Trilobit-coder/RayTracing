use std::io::Result;
use std::sync::Arc;

use crate::hittable::{Block, BvhNode, HittableList, Quad, RotationY, Sphere, Translate};
use crate::optical::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::optical::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::optical::{Camera, camera};
use crate::utilities::Point3;
use crate::utilities::random::{random_f32, random_f32_range};
use crate::utilities::{Color, Vec3};

/// enum for sample scenes
pub enum Scene {
    /// A scene with bounding spheres, test for basic optics
    BoundingSphere,
    /// A scene with spheres having checker texture
    CheckerSphere,
    /// A globe with image mapping
    Earth,
    /// A sphere with perlin noise
    PerlinSphere,
    /// Five quads
    Quads,
    /// Simple light source
    Light,
    /// Cornell box
    CornellBox,
}

/// build and render a scene based on the scene option
pub fn run_scene(option: Scene) -> Result<()> {
    match option {
        Scene::BoundingSphere => bounding_sphere(),
        Scene::CheckerSphere => checkered_sphere(),
        Scene::Earth => earth(),
        Scene::PerlinSphere => perlin_sphere(),
        Scene::Quads => quads(),
        Scene::Light => simple_light(),
        Scene::CornellBox => cornell_box(),
    }
}

fn bounding_sphere() -> Result<()> {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::from_texture(checker));
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
    let s0 = Sphere::new(Point3::new(0., 1., 0.), 1.0, material1);
    world.add(Box::new(s0));

    let material2: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let s1 = Sphere::new(Point3::new(-4., 1., 0.), 1.0, material2);
    world.add(Box::new(s1));

    let material3: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let s2 = Sphere::new(Point3::new(4., 1., 0.), 1.0, material3);
    world.add(Box::new(s2));

    world = HittableList::from(Box::new(BvhNode::from_list(world)));

    let camera_config = camera::Config::new()
        .image(16.0 / 9.0, 800)
        .view(
            20.0,
            Point3::new(13., 2., 3.),
            Point3::zero(),
            Vec3::new(0., 1., 0.),
        )
        .focus(0.6, 10.)
        .quality(100, 50);

    let camera = Camera::new(camera_config);

    camera.render(&world)
}

fn checkered_sphere() -> Result<()> {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let s0 = Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        Arc::new(Lambertian::from_texture(checker.clone())),
    );
    world.add(Box::new(s0));
    let s1 = Sphere::new(
        Point3::new(0., 10., 0.),
        10.,
        Arc::new(Lambertian::from_texture(checker)),
    );
    world.add(Box::new(s1));

    let camera_config = camera::Config::new()
        .image(16.0 / 9.0, 400)
        .quality(100, 50)
        .view(
            20.,
            Point3::new(13., 2., 3.),
            Point3::new(0., 0., 0.),
            Vec3::new(0., 1., 0.),
        )
        .focus(0., 10.);

    let camera = Camera::new(camera_config);

    camera.render(&world)
}

fn earth() -> Result<()> {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
    let globe = Sphere::new(Point3::new(0., 0., 0.), 2., earth_surface);

    let camera_config = camera::Config::new()
        .image(16.0 / 9.0, 800)
        .quality(100, 50)
        .view(
            20.,
            Point3::new(12., 0., 0.),
            Point3::new(0., 0., 0.),
            Vec3::new(0., 1., 0.),
        )
        .focus(0., 10.);

    let camera = Camera::new(camera_config);

    camera.render(&HittableList::from(Box::new(globe)))
}

fn perlin_sphere() -> Result<()> {
    let pertext = Arc::new(NoiseTexture::new(4.));
    let earth_surface = Arc::new(Lambertian::from_texture(pertext));

    let ground = Sphere::new(Point3::new(0., -1000., 0.), 1000., earth_surface.clone());
    let s0 = Sphere::new(Point3::new(0., 2., 0.), 2., earth_surface);

    let mut world = HittableList::new();
    world.add(Box::new(ground));
    world.add(Box::new(s0));

    let camera_config = camera::Config::new()
        .image(16.0 / 9.0, 1200)
        .quality(200, 50)
        .view(
            20.,
            Point3::new(12., 2., 3.),
            Point3::new(0., 0., 0.),
            Vec3::new(0., 1., 0.),
        )
        .focus(0., 10.);

    let camera = Camera::new(camera_config);

    camera.render(&world)
}

fn quads() -> Result<()> {
    let mut world = HittableList::new();

    // Quads
    let left_red = Quad::new(
        Point3::new(-3., -2., 5.),
        Vec3::new(0., 0., -4.),
        Vec3::new(0., 4., 0.),
        Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2))),
    );
    let back_green = Quad::new(
        Point3::new(-2., -2., 0.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 4., 0.),
        Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2))),
    );
    let right_blue = Quad::new(
        Point3::new(3., -2., 1.),
        Vec3::new(0., 0., 4.),
        Vec3::new(0., 4., 0.),
        Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0))),
    );
    let upper_orange = Quad::new(
        Point3::new(-2., 3., 1.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., 4.),
        Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0))),
    );
    let lower_teal = Quad::new(
        Point3::new(-2., -3., 5.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., -4.),
        Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8))),
    );

    world.add(Box::new(left_red));
    world.add(Box::new(back_green));
    world.add(Box::new(right_blue));
    world.add(Box::new(upper_orange));
    world.add(Box::new(lower_teal));

    let camera_config = camera::Config::new()
        .image(16.0 / 9.0, 800)
        .quality(200, 50)
        .view(
            80.,
            Point3::new(0., 0., 9.),
            Point3::new(0., 0., 0.),
            Vec3::new(0., 1., 0.),
        )
        .focus(0., 10.);

    let camera = Camera::new(camera_config);

    camera.render(&world)
}

fn simple_light() -> Result<()> {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Arc::new(Lambertian::from_texture(pertext)),
    )));

    let difflight = Arc::new(DiffuseLight::from_color(Color::new(4., 4., 4.)));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 7., 0.),
        2.,
        difflight.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(3., 1., -2.),
        Vec3::new(2., 0., 0.),
        Vec3::new(0., 2., 0.),
        difflight,
    )));

    let camera_config = camera::Config::new()
        .image(16.0 / 9.0, 800)
        .quality(200, 50)
        .view(
            20.,
            Point3::new(26., 3., 6.),
            Point3::new(0., 2., 0.),
            Vec3::new(0., 1., 0.),
        )
        .focus(0., 10.)
        .bgcolor(Color::zero());

    let camera = Camera::new(camera_config);

    camera.render(&world)
}

fn cornell_box() -> Result<()> {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15., 15., 15.)));

    world.add(Box::new(Quad::new(
        Point3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        green,
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        red,
    )));
    world.add(Box::new(Quad::new(
        Point3::new(343., 554., 332.),
        Vec3::new(-130., 0., 0.),
        Vec3::new(0., 0., -105.),
        light,
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0., 0., 0.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 0., 555.),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(555., 555., 555.),
        Vec3::new(-555., 0., 0.),
        Vec3::new(0., 0., -555.),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0., 0., 555.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        white.clone(),
    )));

    let box1 = Arc::new(Block::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        white.clone(),
    ));
    let box1 = Arc::new(RotationY::new(box1, 15.));
    let box1 = Box::new(Translate::new(box1, Vec3::new(265., 0., 295.)));
    world.add(box1);

    let box2 = Arc::new(Block::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white,
    ));
    let box2 = Arc::new(RotationY::new(box2, -18.));
    let box2 = Box::new(Translate::new(box2, Vec3::new(130., 0., 65.)));
    world.add(box2);

    let camera_config = camera::Config::new()
        .image(1.0, 600)
        .quality(200, 50)
        .view(
            40.,
            Point3::new(278., 278., -800.),
            Point3::new(278., 278., 0.),
            Vec3::new(0., 1., 0.),
        )
        .bgcolor(Color::zero());

    let camera = Camera::new(camera_config);

    camera.render(&world)
}
