use std::any::type_name;
use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

use csscolorparser::Color;
use html_parser::Element;
use imageproc::drawing::text_size;
use rusttype::{Font, Scale};

use crate::font::{FontId, FontLoader};

#[derive(Debug, Clone)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

#[derive(Clone)]
pub struct Text<'a> {
    pub content: String,
    pub font_size: f32,
    pub font_color: Color,
    pub background_color: Color,
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

    pub fn scale(&self) -> Scale {
        Scale::uniform(self.font_size)
    }

    pub fn initialize_font(&mut self, font_loader: &mut FontLoader<'a>) {
        self.font = Some(font_loader.load_font(&self.font_id));
    }

    pub fn font(&self) -> Rc<Font<'a>> {
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
