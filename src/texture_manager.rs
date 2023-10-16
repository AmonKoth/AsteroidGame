// use sdl2::image::LoadTexture;
// use sdl2::render::{Texture, TextureCreator};

// use std::collections::HashMap;

// pub struct TextureManager<'a> {
//     textures: HashMap<String, &'a Texture<'a>>,
// }

// impl<'a> TextureManager<'a> {
//     pub fn new() -> Self {
//         TextureManager {
//             textures: HashMap::new(),
//         }
//     }

//     pub fn load_texture(
//         &mut self,
//         key: &str,
//         path: &str,
//         texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
//     ) -> Result<(), String> {
//         let texture = texture_creator.load_texture(path)?;
//         self.textures.insert(key.to_string(), texture);
//         Ok(())
//     }
//     pub fn get_texture(&self, key: &str) -> Option<&Texture<'a>> {
//         self.textures.get(key).map(|t| t as &sdl2::render::Texture)
//     }
// }
