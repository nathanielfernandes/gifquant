use std::path::PathBuf;

use image::RgbaImage;

pub fn read_image(path: &str) -> Result<RgbaImage, &'static str> {
    let image = image::open(path).map_err(|_| "Failed to open image")?;
    let image = image.into_rgba8();
    Ok(image)
}

pub fn read_images(paths: &[String]) -> Result<Vec<RgbaImage>, &'static str> {
    let mut images = Vec::with_capacity(paths.len());
    for path in paths {
        let image = read_image(path)?;
        images.push(image);
    }
    Ok(images)
}

pub fn get_ord(path: &str) -> u32 {
    let filename = path
        .split('/')
        .last()
        .expect("Failed to get filename")
        .to_string();
    let ord = filename
        .split('.')
        .next()
        .expect("Failed to get number from filename");
    let ord = ord
        .chars()
        .rev()
        .take_while(|c| c.is_digit(10))
        .collect::<String>();
    let ord = ord
        .chars()
        .rev()
        .collect::<String>()
        .parse::<u32>()
        .expect("Failed to parse number from filename");
    ord
}

pub fn read_dir(path: &PathBuf) -> Vec<String> {
    let image_paths = std::fs::read_dir(path).expect("Failed to read images directory");
    let mut images = Vec::new();
    for image_path in image_paths {
        let image_path = image_path.expect("Failed to read image path").path();
        let image_path = image_path
            .to_str()
            .expect("Failed to convert image path to str")
            .to_string();
        images.push(image_path);
    }

    // sort by number at the end of the filename (ex <something><number>.png)
    images.sort_by(|a, b| get_ord(a).cmp(&get_ord(b)));

    images
}
