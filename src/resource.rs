use std::cmp::Ordering;
use std::fs::{self, DirEntry, ReadDir};
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

        for path in get_dir_files("./images/tiles/") {
            tilesheet.push(TilesheetData {
                name: path.path().display().to_string(),
                tile: Texture::from_file(path.path())?
                    .new_tilesheet(&mut atlases[1], renderer, TILE_SIZE as u32)
                    .ok_or_else(|| OtherError::new("failed to upload tiles"))?,
            });
        }

        for path in get_dir_files("./images/items/") {
            items.push(TextureData {
                name: path.path().display().to_string(),
                allocation: Texture::from_file(path.path())?
                    .upload(&mut atlases[0], renderer)
                    .ok_or_else(|| OtherError::new("failed to upload image"))?,
            });
        }

        for path in get_dir_files("./images/player/") {
            players.push(TextureData {
                name: path.path().display().to_string(),
                allocation: Texture::from_file(path.path())?
                    .upload(&mut atlases[0], renderer)
                    .ok_or_else(|| OtherError::new("failed to upload image"))?,
            });
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

pub fn get_dir_files(path: &str) -> Vec<DirEntry> {
    let mut files = Vec::new();

    let is_file = |path: &DirEntry| {
        if let Ok(meta) = path.metadata() {
            if meta.is_file() {
                return true;
            }
        }

        false
    };

    if let Ok(mut paths) = fs::read_dir(path) {
        while let Some(path) = paths.next().and_then(|p| p.ok()) {
            if is_file(&path) {
                files.push(path);
            }
        }
    }

    files.sort_by(|a, b| {
        natural_cmp(
            a.file_name().to_str().unwrap_or_default(),
            b.file_name().to_str().unwrap_or_default(),
        )
    });
    files
}

pub fn natural_cmp(a: &str, b: &str) -> Ordering {
    // Walk through two the strings with two markers.
    let (mut chs1, mut chs2) = (a.chars(), b.chars());
    let (mut ch1, mut ch2) = (chs1.next(), chs2.next());
    let mut space1 = String::with_capacity(32);
    let mut space2 = String::with_capacity(32);

    loop {
        if ch1.is_none() || ch2.is_none() {
            break;
        }

        //Some buffers we can build up characters in for each chunk.
        space1.clear();
        space2.clear();
        let mut is_digit1 = false;
        let mut is_digit2 = false;

        // Walk through all following characters that are digits or
        // characters in BOTH strings starting at the appropriate marker.
        // Collect char arrays.
        while ch1.is_some() {
            let char = ch1.unwrap();
            is_digit1 = char.is_ascii_digit();

            space1.push(ch1.unwrap());
            ch1 = chs1.next();

            if let Some(char) = ch1 {
                if char.is_ascii_digit() != is_digit1 {
                    break;
                }
            }
        }

        while ch2.is_some() {
            let char = ch2.unwrap();
            is_digit2 = char.is_ascii_digit();

            space2.push(ch2.unwrap());
            ch2 = chs2.next();

            if let Some(char) = ch2 {
                if char.is_ascii_digit() != is_digit2 {
                    break;
                }
            }
        }

        // If we have collected numbers, compare them numerically.
        // Otherwise, if we have strings, compare them alphabetically.
        let mut result = Ordering::Equal;

        if !space1.is_empty() && !space2.is_empty() {
            if is_digit1 && is_digit2 {
                let num1: i64 = space1.parse().unwrap_or(0);
                let num2: i64 = space2.parse().unwrap_or(0);
                result = num1.cmp(&num2);
            } else {
                result = space1.cmp(&space2);
            }
        }

        if result != Ordering::Equal {
            return result;
        }
    }

    a.len().cmp(&b.len())
}
