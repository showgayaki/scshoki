use image::GenericImageView;


// 画像の下を navigation_bar_height 分カットする関数
pub fn crop_image(image_data: &[u8], navigation_bar_height: f64) -> Result<Vec<u8>, String> {
    println!("crop_image");
    println!("navigation_bar_height: {}", navigation_bar_height);
    let image = image::load_from_memory(image_data).map_err(|e| format!("Failed to load image: {}", e))?;
    let (width, height) = image.dimensions();

    let width = width as f64;
    let height = height as f64;

    if height <= navigation_bar_height {
        return Err("Image height is smaller than navigation bar height, cannot crop.".to_string());
    }

    let cropped_height = (height - navigation_bar_height) as u32;
    let cropped_image = image.view(0, 0, width as u32, cropped_height).to_image();

    let mut output = std::io::Cursor::new(Vec::new());
    cropped_image.write_to(&mut output, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to save cropped image: {}", e))?;

    Ok(output.into_inner())
}


// 最後のスクロールで、被った部分をカットする関数
pub fn crop_bottom(image_data: &[u8], crop_height: f64) -> Result<Vec<u8>, String> {
    let image = image::load_from_memory(image_data).map_err(|e| format!("Failed to load image: {}", e))?;
    let (width, height) = image.dimensions();
    let height = height as f64;

    if height <= crop_height {
        return Err("Image height is smaller than crop height, cannot crop.".to_string());
    }

    let new_height = (height - crop_height) as u32;
    let cropped_image = image.view(0, crop_height as u32, width, new_height).to_image();

    let mut output = std::io::Cursor::new(Vec::new());
    cropped_image.write_to(&mut output, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to save cropped image: {}", e))?;

    Ok(output.into_inner())
}
