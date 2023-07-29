use std::cmp::max;
use std::collections::VecDeque;

use imageproc::rect::Rect;

use crate::param::ImageCanvas;
use crate::text::{Alignment, Text, TextLayout};

pub struct LineInfo<'a> {
    pub max_text_height: u32,
    texts: Vec<Text<'a>>,
}

impl<'a> LineInfo<'a> {
    pub fn new() -> LineInfo<'a> {
        LineInfo {
            max_text_height: 0,
            texts: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.texts.is_empty()
    }

    pub fn push_text(&mut self, text: Text<'a>) {
        let (_, height) = text.size();
        self.max_text_height = max(self.max_text_height, height);
        self.texts.push(text);
    }

    pub fn line_height(&self, line_spacing: f32) -> u32 {
        (self.max_text_height as f32 * line_spacing + 0.5) as u32
    }

    pub fn draw(&self, canvas: &mut ImageCanvas, line_rect: Rect) {
        if self.is_empty() {
            return;
        }

        let mut text_layout = TextLayout {
            line_rect,
            offset_left: 0,
            offset_right: 0,
        };

        let pending_drawables = self.texts.iter().fold(VecDeque::new(), |mut acc, text| {
            match text.alignment() {
                Alignment::Right => acc.push_front(text),
                _ => acc.push_back(text),
            }
            acc
        });

        pending_drawables
            .iter()
            .for_each(|text| text.draw(canvas, &mut text_layout));
    }
}
