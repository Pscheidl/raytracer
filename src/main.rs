extern crate find_folder;
extern crate piston_window;

mod drawing;
mod game;
mod player;
mod projectile;
mod enemy;

use drawing::to_gui_coord_u32;
use game::Game;
use piston_window::types::Color;
use piston_window::*;

const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const WINDOW_WIDTH: usize = 500*2;
const WINDOW_HEIGHT: usize = 500*2 + 50;

pub const TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

use crate::drawing::draw_rectange;


pub struct Pos {
    pub x: f64,
    pub y: f64,
}

fn main() {
    // Prepare fonts   
    // Prepare window settings
    let mut window_settings = piston_window::WindowSettings::new(
        "Raycaster",
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

            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 30).draw(
                format!("X {:#?} Y {:#?} Z {:#?}", game.player.x, game.player.y, game.player.z).as_str(),
            &mut glyphs,
            &c.draw_state,
            transform, g
            ).unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);

            });
    }   

}