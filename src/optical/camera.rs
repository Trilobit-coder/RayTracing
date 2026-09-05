use std::{
    fs::File,
    io::{BufWriter, Result, Write},
};

use rayon::prelude::*;

use crate::{
    hittable::Hittable,
    optical::optics::ray_color,
    utilities::{Color, Point3, Ray, Vec3, color, random::random_f32},
};

/// Camera configuration parameters.
pub struct Config {
    /// Image aspect ratio.
    pub aspect_ratio: f32,
    /// Rendered image width in pixel count.
    pub image_width: u32,
    /// Count of random samples for each pixel.
    pub samples_per_pixel: u32,
    /// Maximum number of ray bounces into scene.
    pub max_depth: u32,

    /// Vertical view angle (field of view).
    pub vfov: f32,
    /// Point camera is looking from.
    pub lookfrom: Point3,
    /// Point camera is looking at.
    pub lookat: Point3,
    /// Camera-relative "up" direction.
    pub vup: Vec3,

    /// Variation angle of rays through each pixel.
    pub defocus_angle: f32,
    /// Distance from camera lookfrom point to plane of perfect focus.
    pub focus_dist: f32,
}

impl Config {
    /// Creates a camera configuration with default values.
    pub fn new() -> Config {
        Config::default()
    }

    /// Sets the image aspect ratio and width.
    pub fn image(mut self, aspect_ratio: f32, image_width: u32) -> Self {
        self.aspect_ratio = aspect_ratio;
        self.image_width = image_width;

        self
    }

    /// Sets samples per pixel and max ray depth.
    pub fn quality(mut self, samples_per_pixel: u32, max_depth: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self.max_depth = max_depth;

        self
    }

    /// Sets the view angle, camera position, target, and up direction.
    pub fn view(mut self, vfov: f32, lookfrom: Point3, lookat: Point3, vup: Vec3) -> Self {
        self.vfov = vfov;
        self.lookfrom = lookfrom;
        self.lookat = lookat;
        self.vup = vup;

        self
    }

    /// Sets the defocus angle and focus distance.
    pub fn focus(mut self, defocus_angle: f32, focus_dist: f32) -> Self {
        self.defocus_angle = defocus_angle;
        self.focus_dist = focus_dist;

        self
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 100,
            max_depth: 50,
            vfov: 90.0,
            lookfrom: Point3::zero(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }
}

/// A camera that renders a scene to a PPM file.
pub struct Camera {
    image_width: u32,       // Rendered image width in pixel count
    image_height: u32,      // Rendered image height
    samples_per_pixel: u32, // Count of random samples for each pixel
    max_depth: u32,         // Maximum number of ray bounces into scene

    center: Point3,           // Camera center
    pixel00_loc: Point3,      // Location of pixel 0, 0
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
    defocus_angle: f32,       // Variation angle of rays through each pixel
    defocus_disk_u: Vec3,     // Defocus disk horizontal radius
    defocus_disk_v: Vec3,     // Defocus disk vertical radius
    pixel_samples_scale: f32, // Color scale factor for a sum of pixel samples
}

impl Camera {
    /// Creates a camera from the given configuration.
    pub fn new(config: Config) -> Camera {
        let image_width = config.image_width;
        let image_height = (image_width as f32 / config.aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let pixel_samples_scale = 1.0 / config.samples_per_pixel as f32;

        let center = config.lookfrom;

        // Determine viewport dimensions.
        let theta = f32::to_radians(config.vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h * config.focus_dist;
        let viewport_width = viewport_height * ((image_width) as f32 / (image_height) as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = Vec3::unit_vector(&(config.lookfrom - config.lookat));
        let u = Vec3::unit_vector(&Vec3::cross(&config.vup, &w));
        let v = Vec3::cross(&w, &u);

        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * v * -1.0; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - (config.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_angle = config.defocus_angle;
        let defocus_radius = config.focus_dist * f32::tan(f32::to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            image_height,
            samples_per_pixel: config.samples_per_pixel,
            max_depth: config.max_depth,

            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            pixel_samples_scale,
        }
    }

    /// Renders the world and writes the result to `./data/out.ppm`.
    pub fn render<T: Hittable>(&self, world: &T) -> Result<()> {
        std::fs::create_dir_all("./data")?;

        let ppm_file = File::create("./data/out.ppm")?;
        let mut out = BufWriter::new(ppm_file);

        writeln!(out, "P3\n{} {}\n255", self.image_width, self.image_height)?;

        let pixels: Vec<Color> = (0..self.image_height)
            .into_par_iter()
            .flat_map_iter(|j| (0..self.image_width).map(move |i| (i, j)))
            .map(|(i, j)| {
                let mut pixel_color = Color::zero();
                for _ in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += ray_color(&r, self.max_depth, world);
                }
                pixel_color * self.pixel_samples_scale
            })
            .collect();

        for pixel_color in pixels {
            color::write_color(&mut out, &pixel_color)?;
        }

        println!("\rDone.");
        Ok(())
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        // with a random spacetime to simulate motion blur

        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f32 + offset.x()) * self.pixel_delta_u)
            + ((j as f32 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_f32();

        Ray::new(ray_origin, ray_direction).set_time(ray_time)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_f32() - 0.5, random_f32() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }
}
