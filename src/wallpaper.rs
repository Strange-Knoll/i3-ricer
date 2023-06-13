use std::io::BufReader;

use egui::{Rect, TextureHandle, ColorImage};
use image::DynamicImage;


fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let raw_image = image::io::Reader::open(path).ok();
    if raw_image.is_none() {
        print!("Could not open image: {}", path.display());
        return Ok(egui::ColorImage::from_rgba_unmultiplied(
            [0, 0],
            &[]
        ));
    }
    let image = match raw_image.unwrap().decode() {
        Ok(img) => {img},
        Err(e) => {
            println!("Could not decode Raw Image: {}", e);
            DynamicImage::default()
        },
    };
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

#[derive(Clone)]
pub struct Wallpaper{
    pub path:String,
    pub texture:Option<TextureHandle>,
}

impl Wallpaper{
    pub fn new(path:String) -> Self{
        Self{
            path:path,
            texture:None,
        }
    }
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx().load_texture(
                "/home/gravel/Pictures/dirty_wallpaper/blue.png",
                load_image_from_path(std::path::Path::new(&self.path)).unwrap(), 
                Default::default()
            )
        });
        let center = (texture.size_vec2()/2.0).to_pos2();
        let rect = Rect::from_center_size(center, ui.ctx().screen_rect().size());
        // Show the image:
        ui.image(texture, rect.size());
    }
}