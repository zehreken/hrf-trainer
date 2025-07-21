// main.rs
mod utils;

use crate::utils::{Textures, point_in_rect};
use macroquad::{prelude::*, texture};

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

        game_state.draw();

        game_state.debug_draw();

        next_frame().await;
    }
}

struct GameState {
    score: u32,
    buttons: Vec<Button>,
    is_dragging: bool,
    draggable_index: usize,
    draggables: Vec<Draggable>,
    textures: Vec<Texture2D>,
}

impl GameState {
    fn new(textures: Textures) -> Self {
        let mut buttons = vec![];
        let red_button = Button::new(0, Vec2::new(0.0, 100.0), 1);
        buttons.push(red_button);

        let mut draggables = vec![];
        let red_buoy = Draggable::new(
            Rect::new(
                0.0,
                0.0,
                textures.red_buoy.width(),
                textures.red_buoy.height(),
            ),
            1,
        );
        draggables.push(red_buoy);
        let green_buoy = Draggable::new(
            Rect::new(
                0.0,
                0.0,
                textures.green_buoy.width(),
                textures.green_buoy.height(),
            ),
            2,
        );
        draggables.push(green_buoy);
        let colorful_buoy = Draggable::new(
            Rect::new(
                0.0,
                0.0,
                textures.colorful_buoy.width(),
                textures.colorful_buoy.height(),
            ),
            3,
        );
        draggables.push(colorful_buoy);
        let orange_buoy = Draggable::new(
            Rect::new(
                0.0,
                0.0,
                textures.orange_buoy.width(),
                textures.orange_buoy.height(),
            ),
            4,
        );
        draggables.push(orange_buoy);
        let map = Draggable::new(
            Rect::new(0.0, 0.0, textures.map.width(), textures.map.height()),
            0,
        );
        draggables.push(map);

        let textures = vec![
            textures.map,
            textures.red_buoy,
            textures.green_buoy,
            textures.colorful_buoy,
            textures.orange_buoy,
        ];
        Self {
            score: 0,
            buttons,
            is_dragging: false,
            draggable_index: 0,
            draggables,
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
                for (index, draggable) in self.draggables.iter_mut().enumerate() {
                    if point_in_rect(input_position, draggable.rect) {
                        self.is_dragging = true;
                        self.draggable_index = index;
                        draggable.drag_offset =
                            Vec2::new(draggable.rect.x, draggable.rect.y) - input_position;
                        break;
                    }
                }
            } else if is_mouse_button_released(MouseButton::Left) {
                self.is_dragging = false;
                self.draggable_index = 0;
            }
        } else {
            // handle touch input
            let touch = &touches[0];
            input_position = touch.position;
            if touch.phase == TouchPhase::Started {
                for (index, draggable) in self.draggables.iter_mut().enumerate() {
                    if point_in_rect(input_position, draggable.rect) {
                        self.is_dragging = true;
                        self.draggable_index = index;
                        draggable.drag_offset =
                            Vec2::new(draggable.rect.x, draggable.rect.y) - input_position;
                        break;
                    }
                }
            } else if touch.phase == TouchPhase::Ended || touch.phase == TouchPhase::Cancelled {
                self.is_dragging = false;
                self.draggable_index = 0;
            }
        }

        if self.is_dragging {
            let draggable = &mut self.draggables[self.draggable_index];
            draggable.drag(input_position);
        }
    }

    fn draw(&self) {
        for draggable in self.draggables.iter().rev() {
            draggable.draw(&self);
        }
        for button in &self.buttons {
            button.draw(&self.textures);
        }
    }

    fn debug_draw(&self) {
        let info = format!(
            "Version: 0.0.1.0328\nis_dragging: {}\nhas_touch: {}\ndraggable: {}",
            self.is_dragging,
            !touches().is_empty(),
            self.draggable_index,
        );
        draw_multiline_text(&info, 10.0, 20.0, 20.0, None, RED);
    }
}

struct Item {
    id: u32,
}

struct Draggable {
    rect: Rect,
    texture_id: usize,
    drag_offset: Vec2,
}

impl Draggable {
    fn new(rect: Rect, texture_id: usize) -> Self {
        Self {
            rect,
            texture_id,
            drag_offset: Vec2::ZERO,
        }
    }

    fn drag(&mut self, input_position: Vec2) {
        let mut new_pos = input_position + self.drag_offset;
        if self.rect.w > screen_width() || self.rect.h > screen_height() {
            let min = Vec2::new(screen_width() - self.rect.w, screen_height() - self.rect.h);
            new_pos = new_pos.clamp(min, Vec2::ZERO);
        }
        self.rect.x = new_pos.x;
        self.rect.y = new_pos.y;
    }

    fn draw(&self, game_state: &GameState) {
        let t = &game_state.textures[self.texture_id];
        draw_texture_ex(
            t,
            self.rect.x,
            self.rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.rect.w, self.rect.h)),
                ..Default::default()
            },
        );
    }
}

struct Button {
    id: u8,
    rect: Rect,
    texture_id: usize,
}

impl Button {
    fn new(id: u8, position: Vec2, texture_id: usize) -> Self {
        Self {
            id,
            rect: Rect::new(position.x, position.y, 60.0, 60.0),
            texture_id,
        }
    }

    fn draw(&self, textures: &Vec<Texture2D>) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GRAY);
        let t = &textures[self.texture_id];
        draw_texture_ex(
            t,
            self.rect.x + 20.0,
            self.rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(15.0, 60.0)),
                ..Default::default()
            },
        );
    }
}
