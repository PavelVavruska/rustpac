extern crate piston_window;
extern crate find_folder;

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
const RED_COLOR: Color = [0.5, 0.0, 0.0, 1.0];
// ZX Spectrum resolution 256×192
const WINDOW_WIDTH: usize = 256*4;
const WINDOW_HEIGHT: usize = 192*4;

pub const TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];


pub struct Pos {
    pub x: f64,
    pub y: f64,
}

fn main() {
    // Prepare fonts
   
    // Prepare window settings
    let mut window_settings = piston_window::WindowSettings::new(
        "Rustpac game",
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

    // texture player
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let player_sprite = assets.join("astronaut.png");
    let player_sprite: piston_window::G2dTexture = piston_window::Texture::from_path(
            &mut window.create_texture_context(),
            &player_sprite,
            piston_window::Flip::None,
            &piston_window::TextureSettings::new()
        ).unwrap();

    // texture player thrust
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let player_sprite_thrust = assets.join("astronaut_thrust.png");
    let player_sprite_thrust: piston_window::G2dTexture = piston_window::Texture::from_path(
            &mut window.create_texture_context(),
            &player_sprite_thrust,
            piston_window::Flip::None,
            &piston_window::TextureSettings::new()
        ).unwrap();

    // texture enemy
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let enemy_sprite_user_story = assets.join("userstory_icon.png");
    let enemy_sprite_user_story: piston_window::G2dTexture = piston_window::Texture::from_path(
            &mut window.create_texture_context(),
            &enemy_sprite_user_story,
            piston_window::Flip::None,
            &piston_window::TextureSettings::new()
        ).unwrap();

    // texture enemy 2
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let enemy_sprite_bug = assets.join("bug_icon.png");
    let enemy_sprite_bug: piston_window::G2dTexture = piston_window::Texture::from_path(
            &mut window.create_texture_context(),
            &enemy_sprite_bug,
            piston_window::Flip::None,
            &piston_window::TextureSettings::new()
        ).unwrap();
 
    // load fonts
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();
    let mut game_score:usize = 0;
    // how to text https://github.com/PistonDevelopers/piston-examples/blob/master/examples/hello_world.rs

    let mut is_player_dead = false;

    // Create a world
    let mut game = Game::new();

    // Event loop
    while let Some(event) = window.next() {        

        if !is_player_dead {

            // Catch the events of the keyboard
            if let Some(piston_window::Button::Keyboard(key)) = event.press_args() {
                game.key_pressed(key);
            }
            if let Some(piston_window::Button::Keyboard(key)) = event.release_args() {
                game.key_released(key);
            }
            game.player.tick();  // observe keypress all the time
                   
            // Draw all of them
            window.draw_2d(&event, |c, g, device| {
                piston_window::clear(BACK_COLOR, g);
                
                let result = game.compute_one_tick(&c, g, 
                    &player_sprite, &player_sprite_thrust, &enemy_sprite_user_story, &enemy_sprite_bug);
                if *result.get(0).unwrap() == 1 as usize {
                    is_player_dead = true;
                }
                game_score += result.get(1).unwrap();
                     
                // draw text            
                let transform = c.transform.trans(10.0, 50.0);
    
                text::Text::new_color([0.1, 0.7, 0.3, 1.0], 32).draw(
                    format!("Completed tasks: {}", game_score).as_str(),
                &mut glyphs,
                &c.draw_state,
                transform, g
                ).unwrap();
    
                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);
    
                });                
        } else { 
            // player died - show end screen
            window.draw_2d(&event, |c, g, device| {
                piston_window::clear(RED_COLOR, g);
                
                let transform = c.transform.trans(10.0, 150.0);
        
                text::Text::new_color([1.0, 1.0, 1.0, 0.5], 100).draw(
                    "Results",
                &mut glyphs,
                &c.draw_state,
                transform, g
                ).unwrap();
    
                let transform = c.transform.trans(10.0, 450.0);
        
                text::Text::new_color([1.0, 1.0, 1.0, 0.75], 100).draw(
                    format!("Your score: {}", game_score).as_str(),
                &mut glyphs,
                &c.draw_state,
                transform, g
                ).unwrap();
        
                let transform = c.transform.trans(10.0, 700.0);
        
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 50).draw(
                    "Press SPACE to restart the game.",
                &mut glyphs,
                &c.draw_state,
                transform, g
                ).unwrap();

                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);        
                });

            // handle game restart if requested by keypress
            if let Some(piston_window::Button::Keyboard(key)) = event.press_args() {
                if key == piston_window::Key::Space {
                    game = Game::new();
                    is_player_dead = false;
                    game_score = 0;
                }
            }
        }
    }   

}
