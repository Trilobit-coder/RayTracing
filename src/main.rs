use std::process;

use ray_tracer::scene::{Scene, run_scene};

fn main() {
    if let Err(e) = run_scene(Scene::CornellBox) {
        eprintln!("Error writing PPM file: {}", e);
        process::exit(1);
    }
}
