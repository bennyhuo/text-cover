use std::fs;

use html_parser::{Dom, Element, Node};
use imageproc::rect::Rect;

use crate::font::FontLoader;
use crate::line::LineInfo;
use crate::param::ImageCanvas;
use crate::text::{Alignment, Text};

const LINE_SPACING: f32 = 1.5f32;

pub struct Input<'a> {
    pub lines: Vec<LineInfo<'a>>,
}

impl<'a> Input<'a> {
    pub fn new() -> Input<'a> {
        Input {
            lines: vec![LineInfo::new()],
        }
    }

    fn new_line(&mut self) {
        self.lines.push(LineInfo::new())
    }

    pub fn line_size(&self) -> usize {
        self.lines.len()
    }

    pub fn content_height(&self) -> u32 {
        self.lines.iter().fold(0f32, |value, line| {
            line.line_height(LINE_SPACING) as f32 + value
        }) as u32
    }

    pub fn draw(&self, canvas: &mut ImageCanvas, content_rect: Rect) {
        let content_height = self.content_height();

        let mut start_y = (canvas.height() - content_height) / 2;

        self.lines
            .iter()
            .filter(|line| !line.is_empty())
            .for_each(|line| {
                line.draw(
                    canvas,
                    Rect::at(
                        content_rect.left(),
                        (start_y + (line.line_height(LINE_SPACING - 1f32)) / 2) as i32,
                    )
                    .of_size(content_rect.width(), line.line_height(1f32)),
                );

                start_y += line.line_height(LINE_SPACING);
            });
    }

    pub fn push_text(&mut self, text: Text<'a>) {
        self.lines
            .last_mut()
            .get_or_insert(&mut LineInfo::new())
            .push_text(text);
    }

    pub fn parse_input(&mut self, input_path: &str, font_loader: &mut FontLoader<'a>) {
        let html = fs::read_to_string(input_path)
            .unwrap()
            .replace('\r', "")
            .replace('\n', "<br>");

        let dom = Dom::parse(&html).expect("Invalid input.");
        dom.children
            .iter()
            .for_each(|node| self.parse_node(node, &Text::new(), font_loader));
    }

    pub fn parse_node(
        &mut self,
        node: &Node,
        outer_text: &Text<'a>,
        font_loader: &mut FontLoader<'a>,
    ) {
        let mut text = outer_text.clone();
        text.line_index = self.line_size() as u32;
        match node {
            Node::Text(string) => {
                text.content = string.to_string();
                text.initialize_font(font_loader);
                self.push_text(text)
            }
            Node::Element(element) => {
                self.parse_element(element, &mut text);
                element
                    .children
                    .iter()
                    .for_each(|node| self.parse_node(node, &text, font_loader));
            }
            _ => (),
        }
    }

    fn parse_element(&mut self, element: &Element, text: &mut Text) {
        match element.name.as_str() {
            "l" => {
                text.alignment = Option::from(Alignment::Left);
            }
            "r" => {
                text.alignment = Option::from(Alignment::Right);
            }
            "c" => {
                text.alignment = Option::from(Alignment::Center);
            }
            "font" => {
                text.parse_font(element);
            }
            "br" => {
                self.new_line();
            }
            _ => (),
        }
    }
}
