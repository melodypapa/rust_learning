use std::env;
use std::path::Path;

fn main() {
    let image_path = env::args().next().unwrap();
    let path = Path::new(&image_path);
    let img = image::open(path).unwrap();
    let rotated = img.rotate90();
    rotated.save(path).unwrap();

    println!("{:?}",path);
}
