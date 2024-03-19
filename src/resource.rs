use std::fs::{self, DirEntry};
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
    pub tilesheet: Vec<TilesheetData>,
    pub items: Vec<TextureData>,
    pub players: Vec<TextureData>,
}

impl TextureAllocation {
    pub fn new(
        atlases: &mut [AtlasSet],
        renderer: &GpuRenderer,
    ) -> Result<Self, AscendingError> {
        let is_file = |path: &DirEntry| {
            if let Ok(meta) = path.metadata() {
                if meta.is_file() {
                    return true;
                }
            }

            false
        };

        // This is how we load a image into a atlas/Texture. It returns the location of the image
        // within the texture. its x, y, w, h.  Texture loads the file. group_uploads sends it to the Texture
        // renderer is used to upload it to the GPU when done.
        let mut textures = Vec::with_capacity(4);
        for (name, path) in [
            ("bg.png", "images/gui/bg.png"),
            ("horizontal_arrow.png", "images/gui/horizontal_arrow.png"),
            ("vertical_arrow.png", "images/gui/vertical_arrow.png"),
            ("button_icon.png", "images/gui/button_icon.png"),
        ] {
            textures.push(TextureData {
                name: name.to_string(),
                allocation: Texture::from_file(path)?
                    .upload(&mut atlases[0], renderer)
                    .ok_or_else(|| OtherError::new("failed to upload image"))?,
            })
        }

        let (mut tilesheet, mut items, mut players) = (
            Vec::with_capacity(32),
            Vec::with_capacity(32),
            Vec::with_capacity(32),
        );

        if let Ok(mut paths) = fs::read_dir("./images/tiles/") {
            while let Some(path) = paths.next().and_then(|p| p.ok()) {
                if is_file(&path) {
                    tilesheet.push(TilesheetData {
                        name: path.path().display().to_string(),
                        tile: Texture::from_file(path.path())?
                            .new_tilesheet(
                                &mut atlases[1],
                                renderer,
                                TILE_SIZE as u32,
                            )
                            .ok_or_else(|| {
                                OtherError::new("failed to upload tiles")
                            })?,
                    });
                }
            }
        }

        if let Ok(mut paths) = fs::read_dir("./images/items/") {
            while let Some(path) = paths.next().and_then(|p| p.ok()) {
                if is_file(&path) {
                    items.push(TextureData {
                        name: path.path().display().to_string(),
                        allocation: Texture::from_file(path.path())?
                            .upload(&mut atlases[0], renderer)
                            .ok_or_else(|| {
                                OtherError::new("failed to upload image")
                            })?,
                    });
                }
            }
        }

        if let Ok(mut paths) = fs::read_dir("./images/player/") {
            while let Some(path) = paths.next().and_then(|p| p.ok()) {
                if is_file(&path) {
                    players.push(TextureData {
                        name: path.path().display().to_string(),
                        allocation: Texture::from_file(path.path())?
                            .upload(&mut atlases[0], renderer)
                            .ok_or_else(|| {
                                OtherError::new("failed to upload image")
                            })?,
                    });
                }
            }
        }

        // Complete! We can now pass the result
        Ok(Self {
            menu_bg: textures.remove(0),
            horizontal_arrow: textures.remove(0),
            vertical_arrow: textures.remove(0),
            button_icon: textures.remove(0),
            tilesheet,
            items,
            players,
        })
    }
}
