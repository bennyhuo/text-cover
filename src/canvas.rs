use std::collections::VecDeque;

use csscolorparser::Color;
use image::{ImageBuffer, Rgba};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;

use crate::content::Content;
use crate::line::LineInfo;
use crate::text::{Alignment, Text};

const LINE_SPACING: f32 = 1.5f32;

pub trait Drawable {
    fn fill_color(&mut self, color: Color);

    fn draw(&mut self, content: &Content, rect: Rect);

    fn draw_line(&mut self, line: &LineInfo, rect: &mut Rect);

    fn draw_text(&mut self, text: &Text, rect: &mut Rect);
}

pub type ImageCanvas = ImageBuffer<Rgba<u8>, Vec<u8>>;

impl Drawable for ImageCanvas {
    fn fill_color(&mut self, color: Color) {
        draw_filled_rect_mut(
            self,
            Rect::at(0, 0).of_size(self.width(), self.height()),
            Rgba(color.to_rgba8()),
        );
    }

    fn draw(&mut self, content: &Content, rect: Rect) {
        let content_height = content.content_height(LINE_SPACING);

        let mut start_y = (self.height() - content_height) / 2;

        content
            .lines
            .iter()
            .filter(|line| !line.is_empty())
            .for_each(|line| {
                self.draw_line(
                    line,
                    &mut Rect::at(
                        rect.left(),
                        (start_y + (line.line_height(LINE_SPACING - 1f32)) / 2) as i32,
                    )
                    .of_size(rect.width(), line.line_height(1f32)),
                );

                start_y += line.line_height(LINE_SPACING);
            });
    }

    fn draw_line(&mut self, line: &LineInfo, rect: &mut Rect) {
        if line.is_empty() {
            return;
        }

        let pending_drawables = line.texts.iter().fold(VecDeque::new(), |mut acc, text| {
            match text.alignment() {
                Alignment::Right => acc.push_front(text),
                _ => acc.push_back(text),
            }
            acc
        });

        pending_drawables
            .iter()
            .for_each(|text| self.draw_text(text, rect));
    }

    fn draw_text(&mut self, text: &Text, rect: &mut Rect) {
        let font = text.font();
        let (text_width, text_height) = text.size();
        let metrics = font.v_metrics(text.scale());

        // Align the baseline of all the texts with difference font size.
        let offset_y = (rect.height() - metrics.ascent.round() as u32) as i32;

        let text_rect = match text.alignment() {
            Alignment::Left => {
                let left = rect.left();
                *rect = Rect::at(rect.left() + text_width as i32, rect.top())
                    .of_size(rect.width() - text_width, rect.height());
                Rect::at(left, rect.top() + offset_y)
            }
            Alignment::Center => Rect::at(
                rect.left() + (rect.width() - text_width) as i32 / 2,
                rect.top() + offset_y,
            ),
            Alignment::Right => {
                let left = rect.left() + (rect.width() - text_width) as i32;
                *rect = Rect::at(rect.left(), rect.top())
                    .of_size(rect.width() - text_width, rect.height());
                Rect::at(left, rect.top() + offset_y)
            }
        }
        .of_size(text_width, text_height);

        let background_color = Rgba(text.background_color.to_rgba8());
        // don't draw color if it is transparent or it will fill the rect with white color
        // maybe a bug in imageproc or image libs.
        if background_color.0[3] != 0 {
            draw_filled_rect_mut(self, text_rect, background_color);
        }

        draw_text_mut(
            self,
            Rgba(text.font_color.to_rgba8()),
            text_rect.left(),
            text_rect.top(),
            text.scale(),
            &font,
            text.content.as_str(),
        );
    }
}
