use crate::lib::types::{Cell, SnakeSegment, Grid};

pub fn snake_init() -> Vec<SnakeSegment> {
    let mut snake = Vec::new();
    snake.push(SnakeSegment {
        row: 2,
        column: 2,
        color: Cell {
            red: 255,
            green: 255,
            blue: 255,
        }
    });

    println!("A snake segment was created");

    snake
}
//what I have to do in these commented out parts is an iterator to go
//through my recently created snake vector
pub fn snake_moves(snake: &mut Vec<SnakeSegment>, direction: (i32, i32)) {
    // snake.row = snake.row + direction.0 ;
    // println!("snake row: {:?}", snake.row);
    // snake.column = snake.column + direction.1;
    // println!("snake col: {:?}", snake.column);

    // let new_snake = SnakeSegment {
    //     row: snake.row,
    //     column: snake.column,
    //     color: Cell {
    //         red: rand::random(),
    //         green: rand::random(),
    //         blue: rand::random(),
    //     }
    // };

    // //cell is actually colloring the background and that shouldnt happen]
    // //i should have a vector that holds the colors

    // println!("A new snake was created");

}

pub fn draw_grid_with_snake(mut grid: Grid, snake: &Vec<SnakeSegment>) -> Grid {
    // let color = Cell {
    //     red: snake.color.red,
    //     green: snake.color.green,
    //     blue: snake.color.blue,
    // };

    // println!("{:?}, {:?}, {:?}", color.red, color.blue, color.green);

    // grid.grid[snake.row as usize][snake.column as usize] = color;

    grid
}
