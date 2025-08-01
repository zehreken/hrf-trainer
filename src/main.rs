// main.rs
mod utils;

use crate::utils::{Textures, point_in_rect};
use macroquad::prelude::*;

const CROSS_OFFSET: f32 = 20.0;

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
    map: Draggable,
    buttons: Vec<Button>,
    is_dragging: bool,
    current_draggable: Option<Draggable>,
    button_index: Option<usize>,
    dropped_items: Vec<DroppedItem>,
    textures: Vec<Texture2D>,
}

impl GameState {
    fn new(textures: Textures) -> Self {
        let mut buttons = vec![];
        let red_button = Button::new(0, Vec2::new(0.0, 100.0), 1);
        buttons.push(red_button);
        let green_button = Button::new(1, Vec2::new(0.0, 180.0), 2);
        buttons.push(green_button);
        let colorful_button = Button::new(2, Vec2::new(0.0, 260.0), 3);
        buttons.push(colorful_button);
        let orange_button = Button::new(3, Vec2::new(0.0, 340.0), 4);
        buttons.push(orange_button);

        let map = Draggable::new(
            Rect::new(0.0, 0.0, textures.map.width(), textures.map.height()),
            0,
        );

        let textures = vec![
            textures.map,
            textures.red_buoy,
            textures.green_buoy,
            textures.colorful_buoy,
            textures.orange_buoy,
            textures.button_background,
            textures.cross,
        ];
        Self {
            score: 0,
            map,
            buttons,
            is_dragging: false,
            current_draggable: None,
            button_index: None,
            dropped_items: vec![],
            textures,
        }
    }

    fn update(&mut self) {
        let touches = touches();

        let (input_position, is_started, is_ended) = if touches.is_empty() {
            let pos = Vec2::from(mouse_position());
            let is_started = is_mouse_button_pressed(MouseButton::Left);
            let is_ended = is_mouse_button_released(MouseButton::Left);

            (pos, is_started, is_ended)
        } else {
            let touch = &touches[0];
            let pos = Vec2::from(touch.position);
            let is_started = touch.phase == TouchPhase::Started;
            let is_ended = touch.phase == TouchPhase::Ended || touch.phase == TouchPhase::Cancelled;

            (pos, is_started, is_ended)
        };

        if is_started {
            self.handle_start(input_position);
        } else if is_ended {
            self.handle_end(input_position);
        }

        if let Some(draggable) = &mut self.current_draggable {
            draggable.drag(input_position);
        } else if self.is_dragging {
            let delta = self.map.drag(input_position);
            for dropped_item in &mut self.dropped_items {
                dropped_item.update(delta);
            }
        }
    }

    fn handle_start(&mut self, input_position: Vec2) {
        if point_in_rect(input_position, self.map.rect) {
            self.is_dragging = true;
            self.map.drag_offset = Vec2::new(self.map.rect.x, self.map.rect.y) - input_position;
        }
        for (index, button) in self.buttons.iter().enumerate() {
            if point_in_rect(input_position, button.rect) {
                let size = self.textures[button.texture_id].size();
                let mut draggable = Draggable::new(
                    Rect::new(
                        input_position.x,
                        input_position.y,
                        self.textures[button.texture_id].width(),
                        self.textures[button.texture_id].height(),
                    ),
                    button.texture_id,
                );
                draggable.drag_offset = -size / 2.0;
                self.current_draggable = Some(draggable);
                self.is_dragging = true;
                self.button_index = Some(index);
                break;
            }
        }
    }

    fn handle_end(&mut self, input_position: Vec2) {
        if self.button_index.is_some()
            && point_in_rect(
                input_position,
                self.buttons[self.button_index.unwrap()].rect,
            )
        {
            self.button_index = None;
        } else if let Some(draggable) = &self.current_draggable {
            self.dropped_items
                .push(DroppedItem::new(draggable.texture_id, input_position));
        }
        self.current_draggable = None;
        self.is_dragging = false;
    }

    fn draw(&self) {
        self.map.draw(&self.textures);
        for draggable in &self.dropped_items {
            draggable.draw(&self.textures);
        }
        for button in &self.buttons {
            button.draw(&self.textures);
        }
        if let Some(draggable) = &self.current_draggable {
            draggable.draw(&self.textures);
        }
    }

    fn debug_draw(&self) {
        let info = format!(
            "Version: 0.0.1.0328\nis_dragging: {}\nhas_touch: {}\nbutton: {}",
            self.is_dragging,
            !touches().is_empty(),
            self.button_index.is_some(),
        );
        draw_multiline_text(&info, 10.0, 20.0, 20.0, None, RED);
    }
}

struct DroppedItem {
    texture_id: usize,
    position_on_map: Vec2,
}

impl DroppedItem {
    fn new(texture_id: usize, position: Vec2) -> Self {
        Self {
            texture_id,
            position_on_map: position,
        }
    }

    fn update(&mut self, position_offset: Vec2) {
        self.position_on_map += position_offset;
    }

    fn draw(&self, textures: &Vec<Texture2D>) {
        let t = &textures[self.texture_id];
        let size = t.size() * 0.25;
        draw_texture_ex(
            t,
            self.position_on_map.x - size.x / 2.0,
            self.position_on_map.y - size.y / 2.0 - t.height() / 2.0 - CROSS_OFFSET,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(size.x, size.y)),
                ..Default::default()
            },
        );
    }
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

    fn drag(&mut self, input_position: Vec2) -> Vec2 {
        let mut new_pos = input_position + self.drag_offset;
        if self.rect.w > screen_width() || self.rect.h > screen_height() {
            let min = Vec2::new(screen_width() - self.rect.w, screen_height() - self.rect.h);
            new_pos = new_pos.clamp(min, Vec2::ZERO);
        }
        let delta = new_pos - Vec2::new(self.rect.x, self.rect.y);
        self.rect.x = new_pos.x;
        self.rect.y = new_pos.y;

        delta
    }

    fn draw(&self, textures: &Vec<Texture2D>) {
        let t = &textures[self.texture_id];
        draw_texture_ex(
            t,
            self.rect.x,
            self.rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(t.size()),
                ..Default::default()
            },
        );
        let cross = &textures[6];
        draw_texture_ex(
            cross,
            self.rect.x + (t.width() - cross.width()) / 2.0,
            self.rect.y - cross.width() / 2.0 - CROSS_OFFSET,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(cross.width(), cross.height())),
                ..Default::default()
            },
        );
    }
}

struct Button {
    rect: Rect,
    texture_id: usize,
}

impl Button {
    fn new(id: u8, position: Vec2, texture_id: usize) -> Self {
        Self {
            rect: Rect::new(position.x, position.y, 60.0, 60.0),
            texture_id,
        }
    }

    fn draw(&self, textures: &Vec<Texture2D>) {
        let t = &textures[5];
        draw_texture_ex(
            t,
            self.rect.x,
            self.rect.y,
            Color::new(1.0, 1.0, 1.0, 0.5),
            DrawTextureParams {
                dest_size: Some(Vec2::new(60.0, 60.0)),
                ..Default::default()
            },
        );
        let t = &textures[self.texture_id];
        let size = Vec2::new(t.width() * 0.5, t.height() * 0.5);
        draw_texture_ex(
            t,
            self.rect.x + 30.0 - size.x / 2.0,
            self.rect.y + 30.0 - size.y / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(size.x, size.y)),
                ..Default::default()
            },
        );
    }
}
