/// Create diagrams, flowcharts, graphs, etc.,

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use crate::text::draw_text;
use crate::elements::draw_solid_rect;
use crate::Rgb;
use crate::PhotonImage;
use image::DynamicImage;
use crate::helpers;

/// Draw a flowchart.
#[wasm_bindgen]
pub fn draw_flowchart(mut img: &mut DynamicImage, item1: &str) {
    let rgb = Rgb{ r: 255, g: 255, b: 255 };
    let start_x: i32 = 20;
    let start_y: i32 = 20;
    let width: i32 = 130;
    let height: i32 = 100;
    draw_solid_rect(img, &rgb, height as u32, width as u32, start_x, start_y);      
    draw_solid_rect(img, &rgb, height as u32, width as u32, start_x + width + 50, start_y);        
}

/// Draw a barchart, with a specified title and data.
#[wasm_bindgen]
pub fn draw_barchart(mut img: &mut DynamicImage, title: &str, height: u32, width: u32) {
    let rgb = Rgb{ r: 255, g: 255, b: 255 };
    let start_x: i32 = 20;
    let mut start_y: i32 = 20;
    let mut bar_width: u32 = width - 2*(width / 10);
    let num_bars = 5;
    let bar_height: u32 = height / num_bars;
    let lilac = Rgb{r: 204, g: 195, b: 240};
    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for _ in 0..num_bars {
        draw_solid_rect(img, &yellow, bar_height as u32, bar_width as u32, start_x, start_y);
        start_y += (bar_height + 20) as i32;      
        bar_width -= 20;
    }    
    let rgb_white = Rgb { r: 255, g: 255, b: 255};

    draw_text(img, title, 10, start_y as u32, "Lato-Regular", 50.0, &rgb_white);
}

/// Draw a histogram with a specified title, and data.
#[wasm_bindgen]
pub fn draw_histogram(mut img: &mut DynamicImage, title: &str, height: u32, width: u32) {
    let rgb = Rgb{ r: 255, g: 255, b: 255 };
    let start_x: i32 = 20;
    let mut start_y: i32 = 20;
    let mut bar_width: u32 = width - 2*(width / 10);
    let num_bars = 5;
    let bar_height: u32 = height / num_bars;
    let lilac = Rgb{r: 204, g: 195, b: 240};
    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for _ in 0..num_bars {
        draw_solid_rect(img, &lilac, bar_height as u32, bar_width as u32, start_x, start_y);
        start_y += (bar_height) as i32;      
        bar_width -= 20;
    }    
    let rgb_white = Rgb { r: 255, g: 255, b: 255};
    draw_text(img, title, 10, start_y as u32, "Lato-Regular", 50.0, &rgb_white);
}