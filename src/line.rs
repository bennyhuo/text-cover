use std::cmp::max;

use crate::text::Text;

pub struct LineInfo<'a> {
    pub max_text_height: u32,
    pub texts: Vec<Text<'a>>,
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
}
