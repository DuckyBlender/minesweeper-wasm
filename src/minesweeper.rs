use crate::random::*;

use std::collections::HashMap;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const MINE_COUNT: usize = 10;

type Position = (usize, usize);

#[derive(Debug)]
struct Cell {
    position: Position,
    is_mine: bool,
    is_revealed: bool,
    is_flagged: bool,
}

#[derive(Debug)]
struct Minesweeper {
    width: usize,
    height: usize,
    board: HashMap<Position, Cell>,
}

impl Minesweeper {
    pub fn new(mine_count: usize) -> Minesweeper {
        let mut minesweeper = Minesweeper {
            width: WIDTH,
            height: HEIGHT,
            board: HashMap::new(),
        };

        // Generate mines
        let mut rng = XORShift::new();
        let mut mines = Vec::new();

        while mines.len() < MINE_COUNT {
            let x = rng.next() as usize % WIDTH;
            let y = rng.next() as usize % HEIGHT;
            let mine = (x, y);

            if !mines.contains(&mine) {
                mines.push(mine);
            }
        }

        // Initialize board
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let position = (x, y);
                let is_mine = mines.contains(&position);
                let cell = Cell {
                    position,
                    is_mine,
                    is_revealed: false,
                    is_flagged: false,
                };
                minesweeper.board.insert(position, cell);
            }
        }

        // Return the initialized board
        minesweeper
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_initialization() {
        // Check if the mines are truly random
        let mut mine_pos: Vec<Vec<Position>> = Vec::new();
        for _ in 0..3 {
            let minesweeper = Minesweeper::new(MINE_COUNT);

            assert_eq!(minesweeper.width, WIDTH);
            assert_eq!(minesweeper.height, HEIGHT);
            assert_eq!(minesweeper.board.len(), WIDTH * HEIGHT);

            // Check that the number of mines is correct
            let mine_count = minesweeper
                .board
                .values()
                .filter(|cell| cell.is_mine)
                .count();

            // Add the mine positions to the vector
            let mut mines = Vec::new();
            for cell in minesweeper.board.values() {
                if cell.is_mine {
                    mines.push(cell.position);
                }
            }
            mine_pos.push(mines);

            assert_eq!(mine_count, MINE_COUNT);
        }

        // Check that the mines are truly random
        for i in 0..mine_pos.len() {
            for j in 0..mine_pos.len() {
                if i != j {
                    assert_ne!(mine_pos[i], mine_pos[j]);
                }
            }
        }
    }
}
