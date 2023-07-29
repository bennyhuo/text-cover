use std::any::type_name;
use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

use csscolorparser::Color;
use html_parser::Element;
use image::Rgba;
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut, text_size};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};

use crate::font::{FontId, FontLoader};
use crate::param::ImageCanvas;

pub struct TextLayout {
    pub line_rect: Rect,
    pub offset_left: u32,
    pub offset_right: u32,
}

#[derive(Debug, Clone)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

#[derive(Clone)]
pub struct Text<'a> {
    pub content: String,
    font_size: f32,
    font_color: Color,
    background_color: Color,
    font_id: FontId,
    font: Option<Rc<Font<'a>>>,
    pub line_index: u32,
    pub alignment: Option<Alignment>,
}

impl<'a> Text<'a> {
    pub fn new() -> Text<'a> {
        Text {
            content: "".to_string(),
            font_size: 120f32,
            font_color: Color::from([0, 0, 0, 255]),
            background_color: Color::from([255, 255, 255, 255]),
            font_id: FontId::default(),
            font: None,
            line_index: 0,
            alignment: None,
        }
    }

    pub fn alignment(&self) -> &Alignment {
        self.alignment.as_ref().unwrap_or(match self.line_index {
            1 => &Alignment::Left,
            _ => &Alignment::Right,
        })
    }

    fn scale(&self) -> Scale {
        Scale::uniform(self.font_size)
    }

    pub fn initialize_font(&mut self, font_loader: &mut FontLoader<'a>) {
        self.font = Some(font_loader.load_font(&self.font_id));
    }

    fn font(&self) -> Rc<Font<'a>> {
        self.font.clone().unwrap()
    }

    pub fn size(&self) -> (u32, u32) {
        let font = self.font();
        let (w, h) = if self.content.ends_with(' ') {
            let mut new_content = self.content.clone();
            new_content.pop();
            new_content.push('0');
            text_size(self.scale(), &font, new_content.as_str())
        } else {
            text_size(self.scale(), &font, self.content.as_str())
        };

        (w as u32, h as u32)
    }

    pub fn draw(&self, canvas: &mut ImageCanvas, layout: &mut TextLayout) {
        let font = self.font();
        let (text_width, text_height) = self.size();
        let metrics = font.v_metrics(self.scale());

        // Align the baseline of all the texts with difference font size.
        let offset_y = (layout.line_rect.height() - metrics.ascent.round() as u32) as i32;

        let text_rect = match self.alignment() {
            Alignment::Left => {
                let left = layout.line_rect.left() + layout.offset_left as i32;
                layout.offset_left += text_width;
                Rect::at(left, layout.line_rect.top() + offset_y)
            }
            Alignment::Center => Rect::at(
                layout.line_rect.left() + (layout.line_rect.width() - text_width) as i32 / 2,
                layout.line_rect.top() + offset_y,
            ),
            Alignment::Right => {
                let left = layout.line_rect.left()
                    + (layout.line_rect.width() - layout.offset_right - text_width) as i32;
                layout.offset_right += text_width;
                Rect::at(left, layout.line_rect.top() + offset_y)
            }
        }
        .of_size(text_width, text_height);

        let background_color = Rgba(self.background_color.to_rgba8());
        // don't draw color if it is transparent or it will fill the rect with white color
        // maybe a bug in imageproc or image libs.
        if background_color.0[3] != 0 {
            draw_filled_rect_mut(canvas, text_rect, background_color);
        }

        draw_text_mut(
            canvas,
            Rgba(self.font_color.to_rgba8()),
            text_rect.left(),
            text_rect.top(),
            self.scale(),
            &font,
            self.content.as_str(),
        );
    }

    pub fn parse_font(&mut self, element: &Element) {
        assert_eq!(element.name, "font");

        Text::parse_attribute(element, &mut self.font_size, "size");
        Text::parse_attribute(element, &mut self.font_color, "color");
        Text::parse_attribute(element, &mut self.background_color, "background");
        Text::parse_attribute(element, &mut self.font_id.family_name, "family");
        Text::parse_attribute(element, &mut self.font_id.weight, "weight");
        Text::parse_attribute(element, &mut self.font_id.style, "style");
    }

    fn parse_attribute<U: FromStr>(element: &Element, u: &mut U, key: &str)
    where
        U::Err: Debug,
    {
        let option = element.attributes.get(key).and_then(|value| {
            value.as_ref().map(|value| {
                value
                    .parse()
                    .unwrap_or_else(|_| panic!("Invalid {}. {} expected.", key, type_name::<U>()))
            })
        });

        if let Some(value) = option {
            *u = value;
        }
    }
}
