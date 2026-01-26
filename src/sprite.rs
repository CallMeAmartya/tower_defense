use macroquad::prelude::*;

/// A sprite sheet that divides a texture into a grid of frames.
pub struct SpriteSheet {
    pub texture: Texture2D,
    pub frame_width: f32,
    pub frame_height: f32,
    pub columns: usize,
    pub rows: usize,
}

impl SpriteSheet {
    /// Create a new sprite sheet by dividing the texture into a grid.
    pub fn new(texture: Texture2D, columns: usize, rows: usize) -> Self {
        texture.set_filter(FilterMode::Nearest);
        let frame_width = texture.width() / columns as f32;
        let frame_height = texture.height() / rows as f32;
        Self {
            texture,
            frame_width,
            frame_height,
            columns,
            rows,
        }
    }

    /// Draw a specific frame from the sprite sheet, centered at (x, y).
    /// `dest_height` controls the rendered size; width is computed from the frame's aspect ratio.
    /// `flip_x` mirrors the sprite horizontally.
    pub fn draw_frame(
        &self,
        col: usize,
        row: usize,
        x: f32,
        y: f32,
        dest_height: f32,
        flip_x: bool,
    ) {
        let aspect = self.frame_width / self.frame_height;
        let dest_width = dest_height * aspect;
        let source = Rect::new(
            col as f32 * self.frame_width,
            row as f32 * self.frame_height,
            self.frame_width,
            self.frame_height,
        );
        draw_texture_ex(
            &self.texture,
            x - dest_width / 2.0,
            y - dest_height / 2.0,
            WHITE,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(Vec2::new(dest_width, dest_height)),
                flip_x,
                ..Default::default()
            },
        );
    }

    /// Total number of frames in this sprite sheet.
    pub fn frame_count(&self) -> usize {
        self.columns * self.rows
    }
}
