// main.rs
mod utils;

use crate::utils::{Textures, point_in_rect};
use macroquad::prelude::*;

#[macroquad::main("hrf-trainer")]
async fn main() {
    let textures = match Textures::load().await {
        Ok(textures) => textures,
        Err(error) => loop {
            clear_background(BLACK);
            draw_text(&error, 20.0, 20.0, 20.0, RED);
            next_frame().await;
        },
    };

    let mut game_state = GameState::new(textures);

    loop {
        clear_background(BLUE);

        game_state.update();

        let t = &game_state.textures.map;
        draw_texture_ex(
            &t,
            game_state.map_rect.x,
            game_state.map_rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(game_state.map_rect.w, game_state.map_rect.h)),
                ..Default::default()
            },
        );

        game_state.debug_draw();

        next_frame().await;
    }
}

struct GameState {
    score: u32,
    map_rect: Rect,
    is_dragging: bool,
    drag_offset: Vec2,
    textures: Textures,
}

impl GameState {
    fn new(textures: Textures) -> Self {
        Self {
            score: 0,
            map_rect: Rect::new(0.0, 0.0, textures.map.width(), textures.map.height()),
            is_dragging: false,
            drag_offset: Vec2::ZERO,
            textures,
        }
    }

    fn update(&mut self) {
        let touches = touches();
        let mut input_position = Vec2::ZERO;

        if touches.is_empty() {
            // handle mouse input
            input_position = Vec2::from(mouse_position());
            if is_mouse_button_pressed(MouseButton::Left) {
                if point_in_rect(input_position, self.map_rect) {
                    self.is_dragging = true;
                    self.drag_offset = Vec2::new(self.map_rect.x, self.map_rect.y) - input_position;
                }
            } else if is_mouse_button_released(MouseButton::Left) {
                self.is_dragging = false;
            }
        } else {
            // handle touch input
            let touch = &touches[0];
            input_position = touch.position;
            if touch.phase == TouchPhase::Started {
                self.is_dragging = true;
                self.drag_offset = Vec2::new(self.map_rect.x, self.map_rect.y) - input_position;
            } else if touch.phase == TouchPhase::Ended || touch.phase == TouchPhase::Cancelled {
                self.is_dragging = false;
            }
        }

        if self.is_dragging {
            let mut new_pos = input_position + self.drag_offset;
            let min = Vec2::new(
                screen_width() - self.map_rect.w,
                screen_height() - self.map_rect.h,
            );
            new_pos = new_pos.clamp(min, Vec2::ZERO);
            self.map_rect.x = new_pos.x;
            self.map_rect.y = new_pos.y;
        }
    }

    fn debug_draw(&self) {
        let info = format!(
            "Version: 0.0.1.1033\nis_dragging: {}\nhas_touch: {}",
            self.is_dragging,
            !touches().is_empty()
        );
        draw_multiline_text(&info, 10.0, 20.0, 20.0, None, RED);
    }
}

struct Item {
    id: u32,
}
