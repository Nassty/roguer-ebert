use raylib::prelude::{Rectangle, Vector2};

pub struct SpriteSheet {
    tex_size: Vector2,
    tile_size: Vector2,
    offset: i32,
}

impl SpriteSheet {
    pub fn new(tex_size: Vector2, tile_size: Vector2, offset: i32) -> Self {
        Self {
            tex_size,
            tile_size,
            offset,
        }
    }

    #[allow(dead_code)]
    fn total_tiles(&self) -> i32 {
        let tiles_per_row = self.tex_size.x / (self.tile_size.x + self.offset as f32);
        let rows = self.tex_size.y / (self.tile_size.y + self.offset as f32);
        (tiles_per_row * rows) as i32
    }

    pub fn coords_to_rect(&self, x: i32, y: i32) -> Rectangle {
        let index = y * self.tex_size.x as i32 + x;
        self.index_to_rect(index)
    }
    pub fn index_to_rect(&self, index: i32) -> Rectangle {
        let tiles_per_row = self.tex_size.x / (self.tile_size.x + self.offset as f32);
        let row = index / tiles_per_row as i32;
        let col = index % tiles_per_row as i32;

        let x0 = col * (self.tile_size.x as i32 + self.offset);
        let y0 = row * (self.tile_size.y as i32 + self.offset);
        Rectangle::new(x0 as f32, y0 as f32, self.tile_size.x, self.tile_size.y)
    }
}
