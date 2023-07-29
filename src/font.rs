use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use std::str::FromStr;

use crate::utils::MapNone;
use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::properties;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use rusttype::Font;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Weight {
    Bold,
    Light,
    Normal,
}

impl Weight {
    pub(crate) fn to_font_kit_weight(&self) -> properties::Weight {
        match self {
            Weight::Bold => properties::Weight::BOLD,
            Weight::Light => properties::Weight::LIGHT,
            _ => properties::Weight::NORMAL,
        }
    }
}

impl FromStr for Weight {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bold" => Ok(Weight::Bold),
            "light" => Ok(Weight::Light),
            _ => Ok(Weight::Normal),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Style {
    Italic,
    Normal,
}

impl Style {
    pub fn to_font_kit_style(&self) -> properties::Style {
        match self {
            Style::Italic => properties::Style::Italic,
            Style::Normal => properties::Style::Normal,
        }
    }
}

impl FromStr for Style {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "italic" => Ok(Style::Italic),
            _ => Ok(Style::Normal),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FontId {
    pub family_name: String,
    pub style: Style,
    pub weight: Weight,
}

impl Default for FontId {
    fn default() -> Self {
        let source = SystemSource::new();
        let font_families = source.all_families().expect("No fonts in the System.");
        let default_family = font_families
            .iter()
            .find(|value| {
                let lowercase_name = value.to_ascii_lowercase();
                lowercase_name.contains("yahei")
                    || lowercase_name.contains("heiti")
                    || lowercase_name.contains("songti")
                    || lowercase_name.contains("kaiti")
            })
            .expect("No preferred default font found. Please specify the font-family explicitly.");

        FontId {
            family_name: default_family.to_string(),
            style: Style::Normal,
            weight: Weight::Normal,
        }
    }
}

pub struct FontLoader<'a> {
    font_cache: HashMap<FontId, Rc<Font<'a>>>,
}

impl<'a> FontLoader<'a> {
    pub fn new() -> FontLoader<'a> {
        FontLoader {
            font_cache: HashMap::new(),
        }
    }

    fn print_all_fonts() {
        eprintln!("Available fonts: ");
        SystemSource::new()
            .all_families()
            .unwrap_or_default()
            .iter()
            .for_each(|name| {
                eprintln!("- {name}");
            });
    }

    fn load_font_or_none(font_id: &FontId) -> Option<Rc<Font<'a>>> {
        let family_name = FamilyName::Title(font_id.family_name.to_string());
        let mut properties = Properties::new();
        properties.weight(font_id.weight.to_font_kit_weight());
        properties.style(font_id.style.to_font_kit_style());
        let source = SystemSource::new();

        let font_handle = source
            .select_best_match(&[family_name], &properties)
            .map_err(|e| {
                eprintln!("Font '{}' not found. Error: {e}.", font_id.family_name);
                FontLoader::print_all_fonts();
            })
            .ok()?;

        let font_data = match font_handle {
            Handle::Path {
                path,
                font_index: _,
            } => {
                let path = path.to_str().map_none(|| {
                    eprintln!("Failed to load font from path.");
                })?;
                fs::read(path)
                    .map_err(|e| {
                        eprintln!("Failed to load font data from {path}. Error: {e}.");
                    })
                    .ok()?
            }

            Handle::Memory {
                bytes,
                font_index: _,
            } => bytes.to_vec(),
        };

        let font = Font::try_from_vec(font_data)
            .map_none(|| eprintln!("Failed to create font. Invalid font data."))?;
        Some(Rc::new(font))
    }

    pub fn load_font(&mut self, font_id: &FontId) -> Rc<Font<'a>> {
        match self.font_cache.get(font_id) {
            None => {
                let font = FontLoader::load_font_or_none(font_id).unwrap_or_else(|| {
                    let default_font_id = FontId::default();
                    eprintln!("Try to load default font: {}", default_font_id.family_name);
                    FontLoader::load_font_or_none(&default_font_id)
                        .expect("Failed to load default font.")
                });
                self.font_cache.insert(font_id.clone(), font.clone());
                font
            }
            Some(font) => font.clone(),
        }
    }
}
