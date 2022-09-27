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

//const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];
const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
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

    // Create a world
    let mut game = Game::new();

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
    let enemy_sprite = assets.join("userstory_icon.png");
    let enemy_sprite: piston_window::G2dTexture = piston_window::Texture::from_path(
            &mut window.create_texture_context(),
            &enemy_sprite,
            piston_window::Flip::None,
            &piston_window::TextureSettings::new()
        ).unwrap();
 

    // Event loop
    while let Some(event) = window.next() {
        // Catch the events of the keyboard
        if let Some(piston_window::Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        if let Some(piston_window::Button::Keyboard(key)) = event.release_args() {
            game.key_released(key);
        }
        game.player.tick();
        
        // Draw all of them
        window.draw_2d(&event, |c, g, _| {
            piston_window::clear(BACK_COLOR, g);
            
            game.draw(&c, g, &player_sprite, &player_sprite_thrust, &enemy_sprite);
        });
    }
}
