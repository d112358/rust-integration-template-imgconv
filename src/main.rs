use std::path::Path;
use imgconv_core::convert_image;

fn main() {
    let input = Path::new("data/pug_57.jpg");
    let output = Path::new("data/pug_57_converted_main.png");
    match convert_image(input, output) {
        Ok(_) => println!("Converted {:?} -> {:?}", input, output),
        Err(e) => eprintln!("Error: {}", e),
    }
}
