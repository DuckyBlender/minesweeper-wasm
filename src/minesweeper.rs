use crate::random::*;

use std::{collections::HashMap, fmt::Display};

type Position = (usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
enum CellState {
    Revealed,
    Flagged,
    Hidden,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum CellType {
    Mine,
    NoMine(u8), // number of adjacent mines
}

#[derive(Debug)]
struct Cell {
    cell_state: CellState,
    cell_type: CellType,
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    mine_count: usize,
    board: HashMap<Position, Cell>,
    game_over: bool,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        // Print size
        // output.push_str(&format!("{}x{}\n", self.width, self.height));

        for y in 0..self.height {
            for x in 0..self.width {
                let position = (x, y);
                let cell = self.board.get(&position).unwrap();

                let cell_state = match cell.cell_state {
                    CellState::Revealed => match cell.cell_type {
                        CellType::Mine => "💣 ".to_string(),
                        CellType::NoMine(adjacent_mines) => format!("{} ", adjacent_mines),
                    },
                    CellState::Flagged => "🚩 ".to_string(),
                    CellState::Hidden => "🟪 ".to_string(),
                };

                output.push_str(&cell_state);
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        assert!(width > 0);
        assert!(height > 0);
        assert!(mine_count > 0);
        assert!(mine_count < width * height);

        let mut minesweeper = Minesweeper {
            width,
            height,
            mine_count,
            board: HashMap::new(),
            game_over: false,
        };

        // Generate mines
        let mut rng = XORShift::new();
        let mut mines = Vec::new();

        while mines.len() < mine_count {
            let x = rng.next() as usize % width;
            let y = rng.next() as usize % height;
            let mine = (x, y);

            if !mines.contains(&mine) {
                mines.push(mine);
            }
        }

        // Initialize board
        for x in 0..width {
            for y in 0..height {
                let position = (x, y);
                let is_mine = mines.contains(&position);

                // We know the mine positions, so we can calculate the number of adjacent mines
                let adjacent_mines = {
                    let mut adjacent_mines = 0;
                    for neighbor in minesweeper.neighbors(position) {
                        if mines.contains(&neighbor) {
                            adjacent_mines += 1;
                        }
                    }
                    adjacent_mines
                };

                let cell = Cell {
                    cell_state: CellState::Hidden,
                    cell_type: if is_mine {
                        CellType::Mine
                    } else {
                        CellType::NoMine(adjacent_mines)
                    },
                };

                minesweeper.board.insert(position, cell);
            }
        }

        // Return the initialized board
        minesweeper
    }

    pub fn reveal(&mut self, position: Position) {
        let cell_state = self.board[&position].cell_state; // this is copied
        let cell_type = self.board[&position].cell_type; // this too
        if cell_state == CellState::Revealed || cell_state == CellState::Flagged || self.game_over {
            return;
        }
        // If the cell is a mine, the game is over
        if let CellType::Mine = cell_type {
            println!("Game over!");
            // Reveal all mines
            for cell in self.board.values_mut() {
                if let CellType::Mine = cell.cell_type {
                    cell.cell_state = CellState::Revealed;
                }
            }
            self.game_over = true;
        } else {
            // Reveal the cell
            self.board.get_mut(&position).unwrap().cell_state = CellState::Revealed;

            // If the cell is a NoMine, reveal all adjacent cells
            if let CellType::NoMine(adjacent_mines) = cell_type {
                if adjacent_mines == 0 {
                    let neighbors = self.neighbors(position);
                    for neighbor in neighbors {
                        self.reveal(neighbor);
                    }
                }
            }
        }
    }

    fn neighbors(&self, position: Position) -> Vec<Position> {
        let mut neighbors = Vec::new(); // can't use an array because we don't know the size
        let (x, y) = position;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    // skip the current cell
                    continue;
                }

                let neighbor_x = x as i32 + i;
                let neighbor_y = y as i32 + j;

                if neighbor_x < 0
                    || neighbor_x >= self.width as i32
                    || neighbor_y < 0
                    || neighbor_y >= self.height as i32
                {
                    // skip out of bounds cells
                    continue;
                }

                neighbors.push((neighbor_x as usize, neighbor_y as usize));
            }
        }

        neighbors
    }

    pub fn flag(&mut self, position: Position) {
        let cell = self.board.get_mut(&position).unwrap();
        if cell.cell_state == CellState::Revealed || self.game_over {
            return;
        }

        match cell.cell_state {
            CellState::Flagged => cell.cell_state = CellState::Hidden,
            CellState::Hidden => cell.cell_state = CellState::Flagged,
            CellState::Revealed => (),
        }
    }

    pub fn reset(&mut self) {
        *self = Minesweeper::new(self.width, self.height, self.mine_count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reveal_cell() {
        let mut minesweeper = Minesweeper::new(20, 20, 40);

        // Reveal a cell
        let position = (5, 5);
        minesweeper.reveal(position);

        println!("{}", minesweeper);
    }

    #[test]
    fn board_initialization() {
        // Check if the mines are truly random
        let height = 20;
        let width = 20;
        let mine_count = 40;

        let minesweeper = Minesweeper::new(height, width, mine_count);

        assert_eq!(minesweeper.width, height);
        assert_eq!(minesweeper.height, width);
        assert_eq!(minesweeper.board.len(), height * width);

        // Check that the number of mines is correct
        let mine_count = minesweeper
            .board
            .values()
            .filter(|cell| cell.cell_type == CellType::Mine)
            .count();

        // Add the mine positions to the vector
        let mut mines = Vec::new();
        for cell in minesweeper.board.values() {
            if let CellType::Mine = cell.cell_type {
                mines.push(cell);
            }
        }

        assert_eq!(mine_count, 1000);
    }

    #[test]
    fn test_neigbors() {
        let minesweeper = Minesweeper::new(20, 20, 40);
        let position = (5, 5);
        let neighbors = minesweeper.neighbors(position);
        assert_eq!(neighbors.len(), 8);

        let neighbors = minesweeper.neighbors((0, 0));
        assert_eq!(neighbors.len(), 3);
    }
}
