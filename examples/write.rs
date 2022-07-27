use std::fs::File;
use std::path::Path;

fn main() {
    let data = "Hello world";
    let image = jabcode::write_jabcode(data.as_bytes(), &jabcode::WriteOptions::default()).unwrap();

    let fout = &mut File::create(&Path::new("out.png")).unwrap();
    image.write_to(fout, image::ImageFormat::Png).unwrap();
}
