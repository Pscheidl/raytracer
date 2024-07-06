extern crate find_folder;
extern crate piston_window;
extern crate image;

mod game;
mod player;
mod projectile;
mod enemy;
mod room;
mod light_ray;

use game::Game;
use piston_window::types::Color;
use piston_window::*;

pub const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
pub const CANVAS_WIDTH_HALF: usize = 512;
pub const CANVAS_MULTIPLIER: usize = 1;
pub const CANVAS_HEIGHT_HALF: usize = CANVAS_WIDTH_HALF;
pub const CANVAS_WIDTH: usize = CANVAS_WIDTH_HALF*2;
pub const CANVAS_HEIGHT: usize = CANVAS_HEIGHT_HALF*2;
pub const WINDOW_WIDTH: usize = CANVAS_WIDTH*CANVAS_MULTIPLIER;
pub const WINDOW_HEIGHT: usize = CANVAS_HEIGHT*CANVAS_MULTIPLIER + 50;

pub const ROOM_SIZE_X: f64 = 150.0;
pub const ROOM_SIZE_Y: f64 = 150.0;
pub const ROOM_SIZE_Z: f64 = 150.0;

pub const LIGHT_POS_X: f64 = ROOM_SIZE_X / 2.0;
pub const LIGHT_POS_Y: f64 = 0.0;
pub const LIGHT_POS_Z: f64 = ROOM_SIZE_Z / 2.0;


pub const TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

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
            WINDOW_WIDTH as u32,
            WINDOW_HEIGHT as u32,
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

            let result = game.compute_one_tick();
            
            let texture = Texture::from_image(&mut texture_context, &result, &TextureSettings::new()).unwrap();        
            
            image(&texture, c.transform.scale(CANVAS_MULTIPLIER as f64, CANVAS_MULTIPLIER as f64), g);
            
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