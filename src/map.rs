use super::WINDOW_HEIGHT;
use super::WINDOW_WIDTH;


#[derive(Copy, Clone)]
pub struct Ground {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Ground {
    pub fn new(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Ground {
        Ground {
            x,
            y,
            width,
            height,
        }
    }
    pub fn tick(&mut self) {
        // not going to move
    }
}

pub fn getLevel1() -> Vec<Ground> {
    let mut map_item_list = Vec::new();
    let ground1 = Ground::new(WINDOW_HEIGHT as f64/2.0, WINDOW_HEIGHT as f64/2.0,100.0,100.0);
    map_item_list.push(ground1);
    return map_item_list
}