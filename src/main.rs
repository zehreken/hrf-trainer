use macroquad::prelude::*;

#[macroquad::main("hrf-trainer")]
async fn main() {
    let map_texture = load_texture("assets/arstaviken_map.png").await.unwrap();

    loop {
        clear_background(BLUE);

        // Draw the map texture at position (0, 0)
        // draw_texture(&map_texture, 0.0, 0.0, WHITE);

        // Optional: scale the texture to fit screen
        let aspect = map_texture.width() / map_texture.height();
        draw_texture_ex(
            &map_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_height() * aspect, screen_height())),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
