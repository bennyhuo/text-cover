use std::fmt::Debug;

use clap::Parser;
use csscolorparser::Color;

#[derive(Parser, Debug)]
pub struct Parameter {
    #[arg(long, default_value_t = 1920)]
    pub image_width: u32,
    #[arg(long, default_value_t = 1080)]
    pub image_height: u32,
    #[arg(long, default_value_t = 300)]
    pub padding: u32,
    #[arg(long, default_value = "#FFFFFFFF")]
    pub background_color: Color,
    #[arg(short, long)]
    pub input_path: String,
    #[arg(short, long)]
    pub output_path: String,
}
