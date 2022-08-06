#![allow(dead_code)]

use bevy::{
    prelude::{AssetServer, Handle, Res},
    text::Font,
};

#[derive(PartialEq, Eq)]
pub enum Fonts {
    FPS(FpsFontWeight),
    MENU(MenuFontWeight),
}

#[derive(PartialEq, Eq, Debug)]
pub enum FpsFontWeight {
    Regular = 400,
}

#[derive(PartialEq, Eq, Debug)]
pub enum MenuFontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

pub fn get_font(asset_server: &Res<AssetServer>, font: Fonts) -> Handle<Font> {
    let path = match font {
        Fonts::FPS(weight) => format!("fonts\\fps\\{}.otf", weight as u32),
        Fonts::MENU(weight) => format!("fonts\\menu\\{}.otf", weight as u32),
    };

    asset_server.load(&path)
}
