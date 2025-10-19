// utils.rs
use macroquad::prelude::*;

pub struct Textures {
    pub map: Texture2D,
    pub button_background: Texture2D,
    pub cross: Texture2D,
    pub green_buoy: Texture2D,
    pub red_buoy: Texture2D,
    pub colorful_buoy: Texture2D,
    pub orange_buoy: Texture2D,
    pub gull: Texture2D,
}

impl Textures {
    pub async fn load() -> Result<Self, String> {
        // The paths must be relative to the html file, not the wasm file
        let map = load_texture("assets/arstaviken_map.png")
            .await
            .map_err(|_| "Failed to load arstaviken_map.png")?;
        let button_background = load_texture("assets/button.png")
            .await
            .map_err(|_| "Failed to load button.png")?;
        let cross = load_texture("assets/cross.png")
            .await
            .map_err(|_| "Failed to load cross.png")?;
        let green_buoy = load_texture("assets/green_20x80.png")
            .await
            .map_err(|_| "Failed to load green_20x80.png")?;
        let red_buoy = load_texture("assets/red_20x80.png")
            .await
            .map_err(|_| "Failed to load red_20x80.png")?;
        let colorful_buoy = load_texture("assets/yellow_blue_black_20x80.png")
            .await
            .map_err(|_| "Failed to load yellow_blue_black_20x80.png")?;
        let orange_buoy = load_texture("assets/orange_40x40.png")
            .await
            .map_err(|_| "Failed to load orange_40x40.png")?;
        let gull = load_texture("assets/gull.png")
            .await
            .map_err(|_| "Failed to load gull.png")?;

        Ok(Self {
            map,
            button_background,
            cross,
            green_buoy,
            red_buoy,
            colorful_buoy,
            orange_buoy,
            gull,
        })
    }
}

pub fn point_in_rect(point: Vec2, rect: Rect) -> bool {
    rect.contains(point)
}
