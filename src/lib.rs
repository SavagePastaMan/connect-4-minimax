pub mod solver;

#[derive(Clone)]
pub struct Position {
    pub width: i32,
    pub height: i32,
    pub moves: i32,
    pub position: u64,
    pub mask: u64,
    min_score: i32,
    max_score: i32,
}

impl Position {
    pub fn new(width: i32, height: i32) -> Self {
        Position {
            width,
            height,
            moves: 0,
            position: 0,
            mask: 0,
            min_score: -(width * height) / 2 + 3,
            max_score: (width * height + 1) / 2 - 3,
        }
    }

    pub fn from_str(width: i32, height: i32, moves: &str) -> Self {
        let mut p = Self::new(width, height);

        for b in moves.bytes() {
            let col = b - b'1';
            if p.can_play(col as usize) {
                p.play(col as usize);
            }
        }

        p
    }

    pub const fn can_play(&self, col: usize) -> bool {
        self.mask & self.top_mask(col) == 0
    }

    pub fn play(&mut self, col: usize) {
        if !self.can_play(col) {
            println!("can't play in {}", col);
            panic!()
        }
        self.position ^= self.mask;
        self.mask |= self.mask + self.bottom_mask(col);
        self.moves += 1
    }

    pub fn is_winning_move(&self, col: usize) -> bool {
        let mut pos = self.position;

        pos |= (self.mask + self.bottom_mask(col)) & self.column_mask(col);

        self.winner(pos)
    }

    pub const fn key(&self) -> u64 {
        self.position + self.mask
    }

    const fn top_mask(&self, col: usize) -> u64 {
        (1 << (self.height - 1)) << (col as i32 * (self.height + 1)) as u64
    }

    const fn bottom_mask(&self, col: usize) -> u64 {
        1 << (col as i32 * (self.height + 1)) as u64
    }

    const fn column_mask(&self, col: usize) -> u64 {
        ((1 << self.height) - 1) << (col as i32 * (self.height + 1)) as u64
    }

    fn winner(&self, position: u64) -> bool {
        let m = position & (position >> (self.height + 1));
        if m & (m >> (2 * (self.height + 1))) != 0 {
            return true;
        }

        let m = position & (position >> self.height);
        if m & (m >> (2 * self.height)) != 0 {
            return true;
        }

        let m = position & (position >> (self.height + 2));
        if m & (m >> (2 * (self.height + 2))) != 0 {
            return true;
        }

        let m = position & (position >> 1);
        if m & (m >> 2) != 0 {
            return true;
        }

        false
    }
}
