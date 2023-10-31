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

    pub fn load_texture(&mut self, key: &String, path: &String) -> Result<(), String> {
        let texture = self.loader.load_texture(path)?;
        self.tex_map.insert(key.to_string(), texture);
        Ok(())
    }
    pub fn get_texture(&self, key: &String) -> Result<&Texture<'a>, String> {
        match self.tex_map.get(key) {
            None => {
                let error_msg = format!("Texture {} cannot be found", key);
                Err(error_msg).into()
            }
            Some(texture) => Ok(texture),
        }
    }
}
