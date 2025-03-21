use image::GenericImageView;
use log::debug;

// innerHieght分の高さでtrimして、画像の下の余白をカットする関数
pub fn trim_extra_space(image_data: &[u8], inner_height: f64) -> Result<Vec<u8>, String> {
    debug!("trim_extra_space");

    let image =
        image::load_from_memory(image_data).map_err(|e| format!("Failed to load image: {}", e))?;
    let (width, height) = image.dimensions();
    let height = height as f64;

    if height <= inner_height {
        return Err("Image height is smaller than navigation bar height, cannot crop.".to_string());
    }

    let cropped_image = image
        .view(0, 0, width, (inner_height * 3.0) as u32)
        .to_image();

    let mut output = std::io::Cursor::new(Vec::new());
    cropped_image
        .write_to(&mut output, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to save cropped image: {}", e))?;

    Ok(output.into_inner())
}

// 最後のスクロールで、被った部分をカットする関数
pub fn cut_scroll_overlap(
    image_data: &[u8],
    scroll_overlap_height: f64,
) -> Result<Vec<u8>, String> {
    debug!("cut_scroll_overlap");
    debug!("scroll_overlap_height: {} px", scroll_overlap_height);

    let image =
        image::load_from_memory(image_data).map_err(|e| format!("Failed to load image: {}", e))?;
    let (width, height) = image.dimensions();
    // let height = height as f64;

    let scroll_overlap_height = (scroll_overlap_height * 3.0) as u32;
    if height <= scroll_overlap_height {
        return Err("Image height is smaller than crop height, cannot crop.".to_string());
    }

    let new_height = height - scroll_overlap_height;
    let cropped_image = image
        .view(0, scroll_overlap_height, width, new_height)
        .to_image();

    let mut output = std::io::Cursor::new(Vec::new());
    cropped_image
        .write_to(&mut output, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to save cropped image: {}", e))?;

    Ok(output.into_inner())
}
