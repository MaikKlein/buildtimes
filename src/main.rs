extern crate gltf;

use gltf::v1::Gltf;

fn main() {
    let gltf = Gltf::open("").expect("Error loading glTF asset");
}
