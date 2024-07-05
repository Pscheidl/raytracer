extern crate find_folder;
extern crate piston_window;
extern crate image;

use image::{RgbaImage, Rgba, ImageBuffer};

mod drawing;
mod game;
mod player;
mod projectile;
mod enemy;
mod room;
mod light_ray;

use drawing::to_gui_coord_u32;
use game::Game;
use piston_window::types::Color;
use piston_window::*;
use smallvec::SmallVec;


const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const WINDOW_WIDTH: usize = 512*2;
const WINDOW_HEIGHT: usize = 512*2 + 50;
const PIXEL_MULTIPLIER: usize = 2;

pub const TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

use crate::drawing::draw_rectange;
use std::time::SystemTime;


pub struct Pos {
    pub x: f64,
    pub y: f64,
}

fn main() {
    // Prepare fonts   
    // Prepare window settings
    let mut window_settings = piston_window::WindowSettings::new(
        "3D Raytracer",
        [
            to_gui_coord_u32(WINDOW_WIDTH),
            to_gui_coord_u32(WINDOW_HEIGHT),
        ],
    )
    .exit_on_esc(true);

    // Fix vsync extension error for linux
    window_settings.set_vsync(true);

    // Create a window
    let mut window: piston_window::PistonWindow = window_settings.build().unwrap();    
 
    // load fonts
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();


    // Create a world
    let mut game = Game::new();
    let mut start_time = SystemTime::now();

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };
    // Event loop
    while let Some(event) = window.next() {
        
        // Catch the events of the keyboard
        if let Some(piston_window::Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        if let Some(piston_window::Button::Keyboard(key)) = event.release_args() {
            game.key_released(key);
        }
        game.player.spawn_new_rays();  // observe keypress all the time
                
        // Draw all of them
        window.draw_2d(&event, |c, g, device| {
            piston_window::clear(BACK_COLOR, g);

            let result: SmallVec<[Box<SmallVec<[[f32; 4]; 512]>>; 512]> = game.compute_one_tick();
            
            let mut img = RgbaImage::new(512, 512);

            for (y, row) in result.iter().enumerate() {
                let row = row.clone().into_inner().unwrap();
                for (x, val) in row.iter().enumerate() {
                    img.put_pixel(x as u32, y as u32, Rgba([(val[0]*255_f32) as u8, (val[1]*255_f32) as u8, (val[2]*255_f32) as u8, (val[3]*255_f32) as u8]));
                }
            }
         
            let texture = Texture::from_image(&mut texture_context, &img, &TextureSettings::new()).unwrap();        
            
            image(&texture, c.transform.scale(2.0, 2.0), g);
            
            // draw text            
            let transform = c.transform.trans(10.0, WINDOW_HEIGHT as f64 - 12.0);
            let since_the_epoch_in_ms = SystemTime::now() 
                .duration_since(start_time)
                .expect("Time went backwards").as_millis();
            
            let mut detail_string = "high";
            if game.player.is_low_detail_render {
                detail_string = "low";
            }
            
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 30).draw(
                format!("Detail: {} FrameTime: {:.2?} ms, FPS: {:.2?}, X {:#.2?}, Y {:#.2?}, Z {:#.2?}", detail_string, since_the_epoch_in_ms, 1000.0/since_the_epoch_in_ms as f64, game.player.x, game.player.y, game.player.z).as_str(),
            &mut glyphs,
            &c.draw_state,
            transform, g
            ).unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
            start_time = SystemTime::now();
        });
    }
}