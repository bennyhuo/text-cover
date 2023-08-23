use clap::Parser;
use imageproc::rect::Rect;

use crate::canvas::{Drawable, ImageCanvas};
use crate::content::Content;
use crate::font::FontLoader;
use crate::param::Parameter;

mod canvas;
mod content;
mod font;
mod line;
mod param;
mod text;
mod utils;

fn main() {
    let params = Parameter::parse();

    let mut font_loader = FontLoader::new();
    let mut content = Content::new();
    content.parse_input(&params.input_path, &mut font_loader);

    let mut canvas = ImageCanvas::new(params.image_width, params.image_height);
    canvas.fill_color(params.background_color);
    canvas.draw(
        &content,
        Rect::at(params.padding as i32, params.padding as i32).of_size(
            params.image_width - params.padding * 2,
            params.image_height - params.padding * 2,
        ),
    );
    canvas.save(&params.output_path).unwrap();
}
