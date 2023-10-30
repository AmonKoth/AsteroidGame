use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use std::collections::HashMap;

pub struct TextureManager<'a> {
    loader: &'a TextureCreator<WindowContext>,
    tex_map: HashMap<String, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new(loader: &'a TextureCreator<WindowContext>) -> Self {
        TextureManager {
            loader: loader,
            tex_map: HashMap::new(),
        }
    }

    pub fn load_texture(
        &mut self,
        key: &String,
        path: &String,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<(), String> {
        let texture = texture_creator.load_texture(path)?;
        self.tex_map.insert(key.to_string(), texture);
        Ok(())
    }
    pub fn get_texture(&self, key: &String) -> Option<&Texture<'a>> {
        self.tex_map.get(key)
    }
}
