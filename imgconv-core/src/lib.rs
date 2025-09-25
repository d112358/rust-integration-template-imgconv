use std::path::Path;
use image::ImageFormat;

/// Convert an input image file to an output format based on extension
pub fn convert_image(input: &Path, output: &Path) -> Result<(), String> {
    // Load image
    let img = image::open(input).map_err(|e| format!("Failed to open image: {}", e))?;

    // Infer format from output file extension
    let format = match output.extension().and_then(|e| e.to_str()) {
        Some("png") => ImageFormat::Png,
        Some("jpg") | Some("jpeg") => ImageFormat::Jpeg,
        _ => return Err("Unsupported output format".into()),
    };

    // Save image
    img.save_with_format(output, format)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(())
}

/// Example function to demonstrate usage
pub fn example() {
    let input = Path::new("data/pug_57.jpg");
    let output = Path::new("data/pug_57_converted_example.png");
    match convert_image(input, output) {
        Ok(_) => println!("Converted {:?} -> {:?}", input, output),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    #[test]
    fn test_convert_image() {
        let input = PathBuf::from("../data/pug_57.jpg");
        let output = PathBuf::from("../data/pug_57_converted_test.png");
        // Ensure output file does not exist before test
        let _ = fs::remove_file(&output);
        let result = convert_image(&input, &output);
        assert!(result.is_ok());
        assert!(output.exists());
    }
}