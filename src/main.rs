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
use piston_window::color::{WHITE, RED, BLUE, GREEN, YELLOW, GRAY, CYAN, MAGENTA, MAROON};

const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
// ZX Spectrum resolution 256×192
const WINDOW_WIDTH: usize = 256*6;
const WINDOW_HEIGHT: usize = 192*4+30;

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
    let mut game_score:usize = 0;
    // how to text https://github.com/PistonDevelopers/piston-examples/blob/master/examples/hello_world.rs

    let mut is_player_dead = false;

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
            
            let result = game.compute_one_tick(&c, g);

            for color_row in 0..result.len() {
                for color_num in 0..result[color_row].len()  {
                    let color = match result[color_row][color_num] {
                        0 => GRAY,
                        1 => GREEN,
                        2 => RED,
                        3 => BLUE,
                        4 => YELLOW,
                        5 => CYAN,
                        6 => MAGENTA,
                        _ => WHITE,
                    };
                let scaled_x = color_num * 3;
                let scaled_z = color_row * 3;
                draw_rectange( color, scaled_x as f64, scaled_z as f64, 3, 3, &c, g);
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