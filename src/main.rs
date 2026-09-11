use std::process;

use ray_tracer::scene::{Scene, run_scene};

fn main() {
    if let Err(e) = run_scene(Scene::FinalScene) {
        eprintln!("Error writing PPM file: {}", e);
        process::exit(1);
    }
}
