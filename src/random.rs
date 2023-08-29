// XORShift algorithm to generate random numbers
// https://en.wikipedia.org/wiki/Xorshift
pub struct XORShift {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl XORShift {
    pub fn new() -> XORShift {
        XORShift {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: 88675123,
        }
    }

    pub fn next(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ (t ^ (t >> 8));
        self.w
    }
}

// Function to generate mines for minesweeper game
fn generate_mines(width: usize, height: usize, num_mines: usize) -> Vec<(usize, usize)> {
    let mut rng = XORShift::new();
    let mut mines = Vec::new();

    while mines.len() < num_mines {
        let x = rng.next() as usize % width;
        let y = rng.next() as usize % height;
        let mine = (x, y);

        if !mines.contains(&mine) {
            mines.push(mine);
        }
    }

    mines
}
