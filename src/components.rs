use raylib::prelude::{RaylibTexture2D, Rectangle, Texture2D, Vector2};

use crate::sprite_sheet::SpriteSheet;
#[allow(dead_code)]
pub struct GameComponents<'a> {
    pub tex: &'a Texture2D,
    pub lower_horizontal_wall_rect: Rectangle,
    pub upper_horizontal_wall_rect: Rectangle,
    pub left_vertical_wall_rect: Rectangle,
    pub right_vertical_wall_rect: Rectangle,
    pub left_upper_corner_rect: Rectangle,
    pub right_upper_corner_rect: Rectangle,
    pub left_bottom_corner_rect: Rectangle,
    pub right_bottom_corner_rect: Rectangle,

    pub inner_left_upper_corner_rect: Rectangle,
    pub inner_right_upper_corner_rect: Rectangle,
    pub inner_left_bottom_corner_rect: Rectangle,
    pub inner_right_bottom_corner_rect: Rectangle,
    pub portal_rect: Rectangle,

    pub enemy_rect: Rectangle,
    pub floor_rects: Vec<Rectangle>,
    pub player_rect: Rectangle,
    pub origin: Vector2,
    pub rotation: f32,
    pub vfactor: isize,
    pub debug: bool,
    pub active_turn: bool,
    pub screen_size: Vector2,
    pub midpoint: Vector2,
}

impl<'a> GameComponents<'a> {
    pub fn new(tex: &'a Texture2D, screen_size: Vector2) -> Self {
        let sheet = SpriteSheet::new(
            Vector2::new(tex.width() as f32, tex.height() as f32),
            Vector2::new(16.0, 16.0),
            1,
        );
        let lower_horizontal_wall_rect = sheet.index_to_rect(24);
        let upper_horizontal_wall_rect = sheet.index_to_rect(2);
        let left_vertical_wall_rect = sheet.index_to_rect(12);
        let right_vertical_wall_rect = sheet.index_to_rect(14);

        let left_upper_corner_rect = sheet.index_to_rect(4);
        let right_upper_corner_rect = sheet.index_to_rect(5);
        let left_bottom_corner_rect = sheet.index_to_rect(15);
        let right_bottom_corner_rect = sheet.index_to_rect(16);

        let inner_left_upper_corner_rect = sheet.index_to_rect(1);
        let inner_right_upper_corner_rect = sheet.index_to_rect(3);
        let inner_left_bottom_corner_rect = sheet.index_to_rect(23);
        let inner_right_bottom_corner_rect = sheet.index_to_rect(25);
        let portal_rect = sheet.index_to_rect(9);

        let enemy_rect = sheet.index_to_rect(111);
        let floor_rects = vec![
            sheet.index_to_rect(0),
            sheet.index_to_rect(11),
            sheet.index_to_rect(22),
        ];
        let player_rect = sheet.index_to_rect(88);
        let origin = Vector2::new(0.0, 0.0);
        let rotation = 0.0;
        let vfactor = 32;
        let debug = false;
        let active_turn = false;
        let midpoint = Vector2::new(screen_size.x / 2.0, screen_size.y / 2.0);
        Self {
            tex,

            portal_rect,
            lower_horizontal_wall_rect,
            upper_horizontal_wall_rect,
            left_vertical_wall_rect,
            right_vertical_wall_rect,

            left_upper_corner_rect,
            right_upper_corner_rect,
            left_bottom_corner_rect,
            right_bottom_corner_rect,

            inner_left_upper_corner_rect,
            inner_right_upper_corner_rect,
            inner_left_bottom_corner_rect,
            inner_right_bottom_corner_rect,

            enemy_rect,
            floor_rects,
            player_rect,
            origin,
            rotation,
            vfactor,
            debug,
            active_turn,
            screen_size,
            midpoint,
        }
    }
}
