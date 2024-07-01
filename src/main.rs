extern crate find_folder;
extern crate piston_window;

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

const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const WINDOW_WIDTH: usize = 512*2;
const WINDOW_HEIGHT: usize = 512*2 + 50;

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
            
            let result = game.compute_one_tick();

            for color_row in 0..result.len() {
                for color_num in 0..result[color_row].len() {
                let scaled_x = color_num * 2;
                let scaled_z = color_row * 2;
                draw_rectange( result[color_row][color_num], scaled_x as f64, scaled_z as f64, 2, 2, &c, g);
                }
            }
                    
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