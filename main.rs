extern crate raylib;
use ffi::rlSetLineWidth;
use raylib::prelude::*;

const BLOCK_WIDTH: f32 = 24.0;
const BLOCK_HEIGHT: f32 = 24.0;

struct Vector2D {
    x: f32,
    y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Vector2D {
        Self { x, y }
    }

    pub fn sub(&self, vec2: Vector2D) -> Self {
        Self {
            x: self.x - vec2.x,
            y: self.y - vec2.y,
        }
    }

    pub fn add(&self, vec: Vector2D) -> Self {
        Self {
            x: self.x + vec.x,
            y: self.y + vec.y,
        }
    }
}

pub fn sub2(vec: &Vector2D, vec2: &Vector2D) -> Vector2D {
    Vector2D {
        x: vec.x - vec2.x,
        y: vec.y - vec2.y,
    }
}

fn main() {
    let world_Map = [
        [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        [
            1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
       
        [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ],
    ];
    let mut pos_X: f32 = 42.0;
    let mut pos_Y: f32 = 54.0;
    let mut dir_X: f32 = 1.0;

    let (mut rl, thread) = raylib::init().size(640, 640).title("Grid Example").build();
    let mut mousePressed:bool = false;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Draw the grid
        for x in 0..world_Map.len() {
            for y in 0..world_Map[x as usize].len(){
                let x_pos = x as f32 * BLOCK_WIDTH;
                let y_pos = y as f32 * BLOCK_HEIGHT;
                unsafe {
                    rlSetLineWidth(6.5);
                    if world_Map[x as usize][y] > 0 {
                        d.draw_rectangle(
                            x_pos as i32,
                            y_pos as i32,
                            BLOCK_WIDTH as i32,
                            BLOCK_HEIGHT as i32,
                            Color::GREEN,
                        );
                    } else {
                        d.draw_rectangle_lines(
                            x_pos as i32,
                            y_pos as i32,
                            BLOCK_WIDTH as i32,
                            BLOCK_HEIGHT as i32,
                            Color::RED,
                        )
                    }
                }
            }
        }
        if(d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)){
            mousePressed = !mousePressed;
        }
        d.draw_circle(d.get_mouse_x(), d.get_mouse_y(), 6.0, Color::WHITE);
        d.draw_circle(pos_X as i32, pos_Y as i32, 12.0, Color::YELLOW);
        if mousePressed {
             d.draw_line(d.get_mouse_x(), d.get_mouse_y(),pos_X as i32, pos_Y as i32, Color::WHITE)
        }
    }
}
