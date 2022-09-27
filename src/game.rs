use std::collections::HashSet;

use super::WINDOW_HEIGHT;
use super::WINDOW_WIDTH;
use crate::player;
use crate::drawing::draw_rectange;
use crate::player::WIDTH;
use crate::enemy;
use piston_window::Transformed;
use piston_window::color::WHITE;
use piston_window::types::Color;
use rand::Rng;

const WORLD_COLOR: Color = [0.1, 0.9, 0.1, 1.0];
const PORTAL_COLOR: Color = [0.9, 0.0, 0.0, 1.0];
const PLAYER_COLOR: Color = [0.9, 0.9, 0.9, 1.0];
const TRANSFORMED_MINIMAP_X_OFFSET: f64 = 650.0;

pub struct Game {
    // World buffers
    pub frame_buffer: [[bool; WINDOW_HEIGHT]; WINDOW_WIDTH],
    pub frame_buffer_next_tick: [[bool; WINDOW_HEIGHT]; WINDOW_WIDTH],
    pub player: player::Player,
    pub enemies: Vec<enemy::Enemy>,
    pub enemy_spawn_ticks: usize,
}

impl Game {
    pub fn new() -> Game {
        // randomize world
        let temp_world = [[false; WINDOW_HEIGHT]; WINDOW_WIDTH];

        Game {
            frame_buffer: temp_world,
            frame_buffer_next_tick: [[false; WINDOW_HEIGHT]; WINDOW_WIDTH],
            player: player::Player::new(
            600.0,
            500.0,
            0.0,
            10.0,
            false,
            false,
            false,
            false,
            false,
            Vec::new(),
            50,            
            ),
            enemies: Vec::new(),
            enemy_spawn_ticks: 150,
        }
    }
    pub fn key_pressed(&mut self, key: piston_window::Key) {
        match key {
            piston_window::Key::Up => self.player.is_moving_up = true,
            piston_window::Key::W => self.player.is_moving_up = true,
            piston_window::Key::Left => self.player.is_turning_left = true,
            piston_window::Key::A => self.player.is_turning_left = true,
            piston_window::Key::Right => self.player.is_turning_right = true,
            piston_window::Key::D => self.player.is_turning_right = true,
            piston_window::Key::RCtrl => self.player.is_shooting = true,
            piston_window::Key::LCtrl => self.player.is_shooting = true,
            _ => {}
        };
    }
    pub fn key_released(&mut self, key: piston_window::Key) {
        match key {
            piston_window::Key::Up => self.player.is_moving_up = false,
            piston_window::Key::W => self.player.is_moving_up = false,
            piston_window::Key::Left => self.player.is_turning_left = false,
            piston_window::Key::A => self.player.is_turning_left = false,
            piston_window::Key::Right => self.player.is_turning_right = false,
            piston_window::Key::D => self.player.is_turning_right = false,
            piston_window::Key::RCtrl => self.player.is_shooting = false,
            piston_window::Key::LCtrl => self.player.is_shooting = false,
            _ => {}
        };
    }  

    /// Draws entire world.
    pub fn draw(&mut self, con: &piston_window::Context,
         g: &mut piston_window::G2d, player_sprite: &piston_window::G2dTexture, player_sprite_thrust: &piston_window::G2dTexture, enemy_sprite: &piston_window::G2dTexture) {
        // Iterate over the world

        // player
        let mut player_trans = con.transform.trans(self.player.x, self.player.y);
        if self.player.is_facing_left {
            player_trans = player_trans.scale(-1.0, 1.0).trans(-(WIDTH as f64), 0.0);
        }
        if self.player.is_moving_up {            
            piston_window::image(player_sprite_thrust, player_trans, g);  
        } else {
            piston_window::image(player_sprite, player_trans, g);   
        }          
        
        // projectiles
        let mut dead_projectiles = Vec::<usize>::new();
        for (index, projectile) in self.player.projectiles.iter_mut().enumerate() {            
            draw_rectange(WHITE, projectile.x, projectile.y, 50, 4, con, g);
            projectile.tick();
            if projectile.time_to_live == 0 {
                dead_projectiles.push(index);
            }            
        }

        for index in dead_projectiles {
            self.player.projectiles.remove(index);
            break; // only one projectile can die by natural death in one tick
        }

        // enemies
        if self.enemy_spawn_ticks > 0 {
            self.enemy_spawn_ticks -= 1;
        } else {
            self.enemy_spawn_ticks = 15;
            let mut rng = rand::thread_rng();
            let mut random_x = 0;
            let mut speed_x = rng.gen_range(0.8..1.2);
            if rng.gen_range(0..2) == 0 {
                random_x = WINDOW_WIDTH;
                speed_x = -rng.gen_range(0.8..1.2);
            }

            self.enemies.push(enemy::Enemy::new(random_x as f64, 50.0 + rng.gen_range(0.0..600.0), speed_x, 50));
        }

        // enemies
        let mut dead_enemies = Vec::<usize>::new();
        for (index, enemy) in self.enemies.iter_mut().enumerate() {         
            
            // draw enemy
            let enemy_trans = con.transform.trans(enemy.x, enemy.y);

            piston_window::image(enemy_sprite, enemy_trans, g);     

            enemy.tick();
            if enemy.time_to_live == 0 {
                dead_enemies.push(index);
            }            
        }   

        for index in dead_enemies {
            self.enemies.remove(index);
            break; // only one enemy can die by natural death in one tick
        }

        // collision
        let mut dead_enemies = HashSet::<usize>::new();
        let mut dead_projectiles = HashSet::<usize>::new();

        for (index_enemy, enemy) in self.enemies.iter_mut().enumerate() {
            // collision enemy - projectile
            for (index_projectile, projectile) in self.player.projectiles.iter_mut().enumerate() {
                if enemy.x - enemy::WIDTH as f64 / 2.0 < projectile.x && enemy.x + enemy::WIDTH as f64 / 2.0 > projectile.x && enemy.y - enemy::HEIGHT as f64 / 2.0 < projectile.y && enemy.y + enemy::HEIGHT as f64 / 2.0 > projectile.y  {
                    dead_enemies.insert(index_enemy);
                    dead_projectiles.insert(index_projectile);
                }                
            }
            // collision enemy - player
            if enemy.x - enemy::WIDTH as f64 / 2.0 < self.player.x && enemy.x + enemy::WIDTH as f64 / 2.0 > self.player.x && enemy.y - enemy::HEIGHT as f64 / 2.0 < self.player.y && enemy.y + enemy::HEIGHT as f64 / 2.0 > self.player.y  {
                // exit
                print!("DEAD!");
            }
        }
        let mut dead_enemies_sorted: Vec<usize> = dead_enemies.into_iter().collect();
        dead_enemies_sorted.sort();
        dead_enemies_sorted.reverse();        
        for index in dead_enemies_sorted {
            self.enemies.remove(index);
        }
        let mut dead_projectiles_sorted: Vec<usize> = dead_projectiles.into_iter().collect();
        dead_projectiles_sorted.sort();
        dead_projectiles_sorted.reverse();
        
        for index in dead_projectiles_sorted {
            self.player.projectiles.remove(index);
        }
    }

    fn restart_game(self) -> Game {
        Game::new()
    }
}