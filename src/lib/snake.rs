use crate::lib::types::{Cell, SnakeSegment, Grid};

pub fn snake_init() -> SnakeSegment {
    let snake = SnakeSegment {
        row: 6,
        column: 6,
        color: Cell {
            red: 0_u8,
            green: 0_u8,
            blue: 0_u8,
        }
    };
    snake
}

pub fn snake_moves(snake: &mut SnakeSegment, direction: (i32, i32)) -> SnakeSegment {
    snake.row = snake.row + direction.0 ;
    snake.column = snake.column + direction.1;

    let new_snake = SnakeSegment {
        row: snake.row,
        column: snake.column,
        color: Cell {
            red: 0_u8,
            green: 0_u8,
            blue: 0_u8,
        }
    };

    new_snake
}

pub fn draw_grid_with_snake(mut grid: Grid, snake: &SnakeSegment) -> Grid {
    let color = Cell {
        red: snake.color.red,
        green: snake.color.green,
        blue: snake.color.green,
    };

    grid.grid[snake.row as usize][snake.column as usize] = color;
    grid
}
