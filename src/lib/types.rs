pub struct Grid {
    pub grid: Vec<Vec<Cell>>,
}

pub struct Cell {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct SnakeSegment {
    pub row: i32,
    pub column: i32,
    pub color: Cell,
}