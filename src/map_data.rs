use std::{collections::HashMap, sync::LazyLock};

use macroquad::math::Vec2;

pub static ARSTAVIKEN_DATA: LazyLock<HashMap<usize, Vec<Vec2>>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(1, vec![Vec2::new(0.0, 0.0)]);
    map.insert(2, vec![Vec2::new(0.0, 0.0)]);
    map.insert(3, vec![Vec2::new(0.0, 0.0)]);
    map.insert(4, vec![Vec2::new(0.0, 0.0)]);

    map
});
