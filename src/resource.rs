use std::path::Path;

use graphics::*;

use crate::TILE_SIZE;

pub struct TextureData {
    pub name: String,
    pub allocation: usize,
}

pub struct TilesheetData {
    pub name: String,
    pub tile: TileSheet,
}

pub struct TextureAllocation {
    pub menu_bg: TextureData,
    pub horizontal_arrow: TextureData,
    pub vertical_arrow: TextureData,
    pub button_icon: TextureData,
    pub player: TextureData,
    pub tilesheet: Vec<TilesheetData>,
}

impl TextureAllocation {
    pub fn new(
        atlases: &mut Vec<AtlasSet>,
        renderer: &GpuRenderer,
    ) -> Result<Self, AscendingError> {
        // This is how we load a image into a atlas/Texture. It returns the location of the image
        // within the texture. its x, y, w, h.  Texture loads the file. group_uploads sends it to the Texture
        // renderer is used to upload it to the GPU when done.
        let menu_bg = TextureData {
            name: "bg.png".to_string(),
            allocation: Texture::from_file("images/gui/bg.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let horizontal_arrow = TextureData {
            name: "horizontal_arrow.png".to_string(),
            allocation: Texture::from_file("images/gui/horizontal_arrow.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let vertical_arrow = TextureData {
            name: "vertical_arrow.png".to_string(),
            allocation: Texture::from_file("images/gui/vertical_arrow.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let button_icon = TextureData {
            name: "button_icon.png".to_string(),
            allocation: Texture::from_file("images/gui/button_icon.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let player = TextureData {
            name: "player.png".to_string(),
            allocation: Texture::from_file("images/player.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let mut tilesheet = Vec::new();
        let mut count = 0;
        let mut path_found = true;
        while path_found {
            let path = format!("./images/tiles/tile_{}.png", count);
            if Path::new(&path).exists() {
                let res = TilesheetData {
                    name: format!("tile_{}.png", count),
                    tile: Texture::from_file(format!(
                        "images/tiles/tile_{}.png",
                        count
                    ))?
                    .new_tilesheet(&mut atlases[1], &renderer, TILE_SIZE as u32)
                    .ok_or_else(|| OtherError::new("failed to upload tiles"))?,
                };

                tilesheet.push(res);

                count += 1;
            } else {
                path_found = false;
            }
        }

        // Complete! We can now pass the result
        Ok(Self {
            menu_bg,
            horizontal_arrow,
            vertical_arrow,
            button_icon,
            player,
            tilesheet,
        })
    }
}
