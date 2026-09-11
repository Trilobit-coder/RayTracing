use std::io::Result;
use std::sync::Arc;

use crate::hittable::{
    Block, BvhNode, ConstantMedium, HittableList, Quad, RotateY, Sphere, Translate,
};
use crate::optical::material::Material;
use crate::optical::texture::Texture;
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
    /// Cornell box with smoke
    CornellSmoke,
    /// Final cover of book2
    FinalScene,
    /// Final cover of book2 with high quality
    FinalSceneHQ,
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
        Scene::CornellSmoke => cornell_smoke(),
        Scene::FinalScene => final_scene(400, 250, 4),
        Scene::FinalSceneHQ => final_scene(800, 10000, 40),
    }
}

fn bounding_sphere() -> Result<()> {
    let mut world = HittableList::new();

    let checker = Texture::checker(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

    let ground_material = Material::lambertian(checker);
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
                let sphere_material: Material;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let center_to = center + Vec3::new(0., random_f32_range(0., 0.5), 0.);
                    sphere_material = Material::lambertian(albedo);
                    let moving_sphere = Sphere::new(center, 0.2, sphere_material).motion(center_to);
                    world.add(Box::new(moving_sphere));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = random_f32_range(0., 0.5);
                    sphere_material = Material::metal(albedo, fuzz);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Material::dielectric(1.5);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Material::dielectric(1.5);
    let s0 = Sphere::new(Point3::new(0., 1., 0.), 1.0, material1);
    world.add(Box::new(s0));

    let material2 = Material::lambertian(Color::new(0.4, 0.2, 0.1));
    let s1 = Sphere::new(Point3::new(-4., 1., 0.), 1.0, material2);
    world.add(Box::new(s1));

    let material3 = Material::metal(Color::new(0.7, 0.6, 0.5), 0.0);
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

    let checker = Texture::checker(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

    let s0 = Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        Material::lambertian(checker.clone()),
    );
    world.add(Box::new(s0));
    let s1 = Sphere::new(Point3::new(0., 10., 0.), 10., Material::lambertian(checker));
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
    let earth_texture = Texture::image("earthmap.jpg");
    let earth_surface = Material::lambertian(earth_texture);
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
    let pertext = Texture::noise(4.);
    let earth_surface = Material::lambertian(pertext);

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
        Material::lambertian(Color::new(1.0, 0.2, 0.2)),
    );
    let back_green = Quad::new(
        Point3::new(-2., -2., 0.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 4., 0.),
        Material::lambertian(Color::new(0.2, 1.0, 0.2)),
    );
    let right_blue = Quad::new(
        Point3::new(3., -2., 1.),
        Vec3::new(0., 0., 4.),
        Vec3::new(0., 4., 0.),
        Material::lambertian(Color::new(0.2, 0.2, 1.0)),
    );
    let upper_orange = Quad::new(
        Point3::new(-2., 3., 1.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., 4.),
        Material::lambertian(Color::new(1.0, 0.5, 0.0)),
    );
    let lower_teal = Quad::new(
        Point3::new(-2., -3., 5.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., -4.),
        Material::lambertian(Color::new(0.2, 0.8, 0.8)),
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

    let pertext = Texture::noise(4.);
    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Material::lambertian(pertext.clone()),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Material::lambertian(pertext),
    )));

    let difflight = Material::diffuse_light(Color::new(4., 4., 4.));
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

    let red = Material::lambertian(Color::new(0.65, 0.05, 0.05));
    let white = Material::lambertian(Color::new(0.73, 0.73, 0.73));
    let green = Material::lambertian(Color::new(0.12, 0.45, 0.15));
    let light = Material::diffuse_light(Color::new(15., 15., 15.));

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
    let box1 = Arc::new(RotateY::new(box1, 15.));
    let box1 = Box::new(Translate::new(box1, Vec3::new(265., 0., 295.)));
    world.add(box1);

    let box2 = Arc::new(Block::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white,
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.));
    let box2 = Box::new(Translate::new(box2, Vec3::new(130., 0., 65.)));
    world.add(box2);

    let world = HittableList::from(Box::new(BvhNode::from_list(world)));

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

fn cornell_smoke() -> Result<()> {
    let mut world = HittableList::new();

    let red = Material::lambertian(Color::new(0.65, 0.05, 0.05));
    let white = Material::lambertian(Color::new(0.73, 0.73, 0.73));
    let green = Material::lambertian(Color::new(0.12, 0.45, 0.15));
    let light = Material::diffuse_light(Color::new(7., 7., 7.));

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
        Point3::new(113., 554., 127.),
        Vec3::new(330., 0., 0.),
        Vec3::new(0., 0., 305.),
        light,
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0., 555., 0.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 0., 555.),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0., 0., 0.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 0., 555.),
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
    let box1 = Arc::new(RotateY::new(box1, 15.));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265., 0., 295.)));

    let box2 = Arc::new(Block::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white,
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));

    world.add(Box::new(ConstantMedium::from_color(
        box1,
        0.01,
        Color::new(0., 0., 0.),
    )));
    world.add(Box::new(ConstantMedium::from_color(
        box2,
        0.01,
        Color::new(1., 1., 1.),
    )));

    let world = HittableList::from(Box::new(BvhNode::from_list(world)));

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

fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32) -> Result<()> {
    let mut boxes1 = HittableList::new();
    let ground = Material::lambertian(Color::new(0.48, 0.83, 0.53));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f32_range(1., 101.);
            let z1 = z0 + w;

            boxes1.add(Box::new(Block::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = HittableList::new();

    world.add(Box::new(BvhNode::from_list(boxes1)));

    let light = Material::diffuse_light(Color::new(7., 7., 7.));
    world.add(Box::new(Quad::new(
        Point3::new(123., 554., 147.),
        Vec3::new(300., 0., 0.),
        Vec3::new(0., 0., 265.),
        light,
    )));

    let center1 = Point3::new(400., 400., 200.);
    let center2 = center1 + Vec3::new(30., 0., 0.);
    let sphere_material = Material::lambertian(Color::new(0.7, 0.3, 0.1));
    world.add(Box::new(
        Sphere::new(center1, 50., sphere_material).motion(center2),
    ));

    world.add(Box::new(Sphere::new(
        Point3::new(260., 150., 45.),
        50.,
        Material::dielectric(1.5),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 150., 145.),
        50.,
        Material::metal(Color::new(0.8, 0.8, 0.9), 1.0),
    )));

    let boundary = Sphere::new(
        Point3::new(360., 150., 145.),
        70.,
        Material::dielectric(1.5),
    );
    world.add(Box::new(boundary.clone()));
    world.add(Box::new(ConstantMedium::from_color(
        Arc::new(boundary),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::new(
        Point3::new(0., 0., 0.),
        5000.,
        Material::dielectric(1.5),
    ));
    world.add(Box::new(ConstantMedium::from_color(
        boundary,
        0.0001,
        Color::new(1., 1., 1.),
    )));

    let emat = Material::lambertian(Texture::image("earthmap.jpg"));
    world.add(Box::new(Sphere::new(
        Point3::new(400., 200., 400.),
        100.,
        emat,
    )));
    let pertext = Texture::noise(0.2);
    world.add(Box::new(Sphere::new(
        Point3::new(220., 280., 300.),
        80.,
        Material::lambertian(pertext),
    )));

    let mut boxes2 = HittableList::new();
    let white = Material::lambertian(Color::new(0.73, 0.73, 0.73));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Box::new(Sphere::new(
            Point3::random_range(0., 165.),
            10.,
            white.clone(),
        )));
    }

    world.add(Box::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BvhNode::from_list(boxes2)), 15.)),
        Vec3::new(-100., 270., 395.),
    )));

    let world = HittableList::from(Box::new(BvhNode::from_list(world)));

    let camera_config = camera::Config::new()
        .image(1.0, image_width)
        .quality(samples_per_pixel, max_depth)
        .view(
            40.,
            Point3::new(478., 278., -600.),
            Point3::new(278., 278., 0.),
            Vec3::new(0., 1., 0.),
        )
        .bgcolor(Color::zero());

    let camera = Camera::new(camera_config);

    camera.render(&world)
}
