extern crate gltf;

use gltf::v1::Gltf;
use std::path::Path;

fn main() {

    let gltf = Gltf::open(Path::new("")).expect("Error loading glTF asset");
}
