use macroquad::prelude::*;

#[macroquad::main("hrf-trainer")]
async fn main() {
    // The path must be relative to the html, not the wasm file
    let map_texture = load_texture("assets/arstaviken_map.png").await;

    loop {
        clear_background(BLUE);

        // Draw the map texture at position (0, 0)
        // draw_texture(&map_texture, 0.0, 0.0, WHITE);

        // Optional: scale the texture to fit screen
        if let Ok(ref t) = map_texture {
            let aspect = t.width() / t.height();
            draw_texture_ex(
                &t,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(screen_height() * aspect, screen_height())),
                    ..Default::default()
                },
            );
        } else {
            draw_text("ERROR: Texture not found", 20.0, 20.0, 20.0, RED);
        }

        next_frame().await;
    }
}
