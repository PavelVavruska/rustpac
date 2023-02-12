use std::collections::HashSet;

use super::WINDOW_HEIGHT;
use super::WINDOW_WIDTH;
use crate::player;
use crate::drawing::draw_rectange;
use crate::player::Movement;
use crate::player::WIDTH;
use crate::enemy;
use crate::map;
use piston_window::Transformed;
use piston_window::color::WHITE;
use rand::Rng;

pub struct Game {
    // World buffers
    pub frame_buffer: [[bool; WINDOW_HEIGHT]; WINDOW_WIDTH],
    pub frame_buffer_next_tick: [[bool; WINDOW_HEIGHT]; WINDOW_WIDTH],
    pub player: player::Player,
    pub enemies: Vec<enemy::Enemy>,
    pub map_grounds: Vec<map::Ground>,
    pub enemy_spawn_difficulty: usize,
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
            Movement::new(
            0.0,
            10.0,
            false,
            false,
            false,
            false),
            false,
            Vec::new(),
            50,            
            ),
            enemies: Vec::new(),
            map_grounds: map::get_level_first(),
            enemy_spawn_ticks: 150,
            enemy_spawn_difficulty: 50,
        }
    }
    pub fn key_pressed(&mut self, key: piston_window::Key) {
        match key {
            piston_window::Key::Up => self.player.movement.is_moving_up = true,
            piston_window::Key::W => self.player.movement.is_moving_up = true,
            piston_window::Key::Left => self.player.movement.is_turning_left = true,
            piston_window::Key::A => self.player.movement.is_turning_left = true,
            piston_window::Key::Right => self.player.movement.is_turning_right = true,
            piston_window::Key::D => self.player.movement.is_turning_right = true,
            piston_window::Key::RCtrl => self.player.is_shooting = true,
            piston_window::Key::LCtrl => self.player.is_shooting = true,
            _ => {}
        };
    }
    pub fn key_released(&mut self, key: piston_window::Key) {
        match key {
            piston_window::Key::Up => self.player.movement.is_moving_up = false,
            piston_window::Key::W => self.player.movement.is_moving_up = false,
            piston_window::Key::Left => self.player.movement.is_turning_left = false,
            piston_window::Key::A => self.player.movement.is_turning_left = false,
            piston_window::Key::Right => self.player.movement.is_turning_right = false,
            piston_window::Key::D => self.player.movement.is_turning_right = false,
            piston_window::Key::RCtrl => self.player.is_shooting = false,
            piston_window::Key::LCtrl => self.player.is_shooting = false,
            _ => {}
        };
    }  

    /// Draws entire world.
    pub fn compute_one_tick(&mut self, con: &piston_window::Context,
         g: &mut piston_window::G2d, 
         player_sprite: &piston_window::G2dTexture, 
         player_sprite_thrust: &piston_window::G2dTexture, 
         enemy_sprite_user_story: &piston_window::G2dTexture,
         enemy_sprite_bug: &piston_window::G2dTexture) -> Vec<usize> {

        let mut result = Vec::<usize>::new();
        // Iterate over the world


        // map - ground
        for ground in &self.map_grounds {
            draw_rectange(WHITE, ground.x, ground.y, ground.width as i32, ground.height as i32, con, g);
        }


        // player
        let mut player_trans = con.transform.trans(self.player.x, self.player.y).scale(0.5, 0.5);
        if self.player.movement.is_facing_left {
            player_trans = player_trans.scale(-1.0, 1.0).trans(-WIDTH, 0.0);
        }
        if self.player.movement.is_moving_up {            
            piston_window::image(player_sprite_thrust, player_trans, g);  
        } else {
            piston_window::image(player_sprite, player_trans, g);   
        }          
        
        // projectiles
        let mut dead_projectiles = Vec::<usize>::new();
        for (index, projectile) in self.player.projectiles.iter_mut().enumerate() {            
            draw_rectange(WHITE, projectile.x, projectile.y, 25, 4, con, g);
            projectile.tick();
            if projectile.time_to_live == 0 {
                dead_projectiles.push(index);
            }            
        }
        
        if let Some(projectile) = dead_projectiles.pop() {
            // only one projectile can die by natural death in one tick
            self.player.projectiles.remove(projectile);
        }

        // enemies
        if self.enemy_spawn_ticks > 10 {
            self.enemy_spawn_ticks -= 1;
        } else {
            self.enemy_spawn_ticks = self.enemy_spawn_difficulty;
            if self.enemy_spawn_difficulty > 20 {
                self.enemy_spawn_difficulty -= 2; // slowly increase rate to enemy spawing
            }
            let mut rng = rand::thread_rng();
            let mut random_x = 0;
            let mut enemy_type = 1;
            let mut speed_x = rng.gen_range(0.8..1.2);
            if rng.gen_range(0..2) == 0 {
                random_x = WINDOW_WIDTH;
                speed_x = -rng.gen_range(0.8..1.2);
                enemy_type = 2;
            }

            // memory leak prevention
            if self.enemies.len() < 16 {
                self.enemies.push(enemy::Enemy::new(random_x as f64, 50.0 + rng.gen_range(0.0..600.0), speed_x, 50, enemy_type));
            }            
        }

        // enemies
        let mut dead_enemies = Vec::<usize>::new();
        for (index, enemy) in self.enemies.iter_mut().enumerate() {         
            
            // draw enemy
            let enemy_trans = con.transform.trans(enemy.x, enemy.y).scale(0.3, 0.3);

            if enemy.enemy_type == 1 {
                piston_window::image(enemy_sprite_user_story, enemy_trans, g);    
            } else {
                piston_window::image(enemy_sprite_bug, enemy_trans, g);  
            }
             

            enemy.tick();
            if enemy.time_to_live == 0 {
                dead_enemies.push(index);
            }            
        }   
        if let Some(dead_enemy) = dead_enemies.pop() {
            // only one enemy can die by natural death in one tick
            self.enemies.remove(dead_enemy);
        }

        // collision
        let mut dead_enemies = HashSet::<usize>::new();
        let mut dead_projectiles = HashSet::<usize>::new();

        let mut is_player_dead = false;

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
                is_player_dead = true;
                print!("DEAD")
            } 
        }
        if is_player_dead {
            result.push(1);
        } else {
            result.push(0);
        }

        let mut dead_enemies_sorted: Vec<usize> = dead_enemies.into_iter().collect();
        let enemy_destroyed = dead_enemies_sorted.len();
        dead_enemies_sorted.sort();
        dead_enemies_sorted.reverse();        
        for index in dead_enemies_sorted {
            self.enemies.remove(index);
        }
        let mut dead_projectiles_sorted: Vec<usize> = dead_projectiles.into_iter().collect();
        dead_projectiles_sorted.sort();
        dead_projectiles_sorted.reverse();
        
        result.push(enemy_destroyed);

        for index in dead_projectiles_sorted {
            self.player.projectiles.remove(index);
        }
        //TODO
        result
    }
}
