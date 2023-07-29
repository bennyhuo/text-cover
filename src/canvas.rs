use csscolorparser::Color;
use image::Rgba;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

use crate::input::Input;
use crate::param::ImageCanvas;

pub trait Draw {
    fn fill_color(&mut self, color: Color);

    fn draw(&mut self, input: &Input, rect: Rect);
}

impl Draw for ImageCanvas {
    fn fill_color(&mut self, color: Color) {
        draw_filled_rect_mut(
            self,
            Rect::at(0, 0).of_size(self.width(), self.height()),
            Rgba(color.to_rgba8()),
        );
    }

    fn draw(&mut self, input: &Input, rect: Rect) {
        input.draw(self, rect);
    }
}
