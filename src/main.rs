// Dependencies go here
use crate::lib::snake;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::thread;
use std::time;

// this is main
pub mod lib;
fn main() {
    let canvas_width = 720_u32;
    let canvas_height = 720_u32;
    let (mut canvas, mut events) = lib::init(canvas_width, canvas_height);
    let columns:u32 = 12; //this is the rustacean way, but I don't need to specify u32 because it's the default
    let rows = 12_u32;
    let cell_width = rows * columns;
    let mut grid = lib::grid_init(columns, rows);
    let mut direction:(i32, i32) = (0, 0);
    let mut snake = snake::snake_init();

    thread::spawn(move || {
        // some work here
        });

    'game: loop {
        println!("A new loop began");
        for event in events.poll_iter()
        {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    println!("Up");
                    direction.0 = -1;
                    direction.1 = 0;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    println!("Down");
                    direction.0 = 1;
                    direction.1 = 0;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    println!("Left");
                    direction.1 = -1;
                    direction.0 = 0;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    println!("Right");
                    direction.1 = 1;
                    direction.0 = 0;
                }

                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'game,

                _ => continue 'game,
            }
        }
        //i could make a snake_update that will create a new snake
        //only if colision happens (either with the snake or with an apple)
        snake::snake_moves(&mut snake, direction);
        grid = snake::draw_grid_with_snake(grid, &snake);
        lib::display_frame(&mut canvas, &grid, &columns, &rows, &cell_width);
        thread::sleep(time::Duration::from_millis(800));
    }


}

