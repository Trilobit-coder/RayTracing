//! Renders a named scene to `./data/out.ppm`.
//!
//! Usage: `cargo run --release --example render -- <scene>`
//! where `<scene>` is one of: bounding, checker, earth, perlin, quads,
//! light, cornell, smoke, final, hq. Defaults to `final`.

use std::process;

use ray_tracer::scene::{Scene, run_scene};

fn main() {
    let name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "final".to_string());

    let scene = match name.as_str() {
        "bounding" => Scene::BoundingSphere,
        "checker" => Scene::CheckerSphere,
        "earth" => Scene::Earth,
        "perlin" => Scene::PerlinSphere,
        "quads" => Scene::Quads,
        "light" => Scene::Light,
        "cornell" => Scene::CornellBox,
        "smoke" => Scene::CornellSmoke,
        "final" => Scene::FinalScene,
        "hq" => Scene::FinalSceneHQ,
        other => {
            eprintln!("unknown scene '{other}'");
            process::exit(1);
        }
    };

    if let Err(e) = run_scene(scene) {
        eprintln!("Error writing PPM file: {}", e);
        process::exit(1);
    }
}
