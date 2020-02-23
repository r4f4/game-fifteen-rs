use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::fmt;

const SIZE: usize = 4;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    pub fn opposites(self, other: Direction) -> bool {
        other.opposite() == self
    }

    pub fn value(self) -> isize {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            Direction::Up => -(SIZE as isize),
            Direction::Down => SIZE as isize,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Board {
    tiles: [u8; SIZE * SIZE],
    empty: usize,
}

impl Board {
    pub fn new() -> Board {
        let mut b = Board {
            tiles: [0; 16],
            empty: 0,
        };
        for i in 0..16u8 {
            b.tiles[i as usize] = i;
        }
        b
    }

    pub fn new_from(tiles: &[u8]) -> Result<Board, &'static str> {
        let mut b = Board {
            tiles: [0; 16],
            empty: 0,
        };
        let mut nums = HashSet::new();
        for (i, &t) in tiles.iter().enumerate() {
            nums.insert(t);
            match t {
                0 => b.empty = i,
                1..=15 => b.tiles[i] = t,
                _ => return Err("tiles should be in the range [0, 15]"),
            }
        }
        match nums.len() {
            0..=15 => Err("missing or repeated tiles"),
            16 => Ok(b),
            _ => Err("too many tiles"),
        }
    }

    pub fn new_random() -> Board {
        let mut b = Board::new();
        b.shuffle();
        b
    }

    pub fn tiles(&self) -> &[u8; 16] {
        &self.tiles
    }

    fn safe_pos(&self, step: isize) -> usize {
        let empty = self.empty;
        if step.is_negative() {
            match empty.checked_sub(step.wrapping_abs() as u32 as usize) {
                Some(v) => v,
                None => empty,
            }
        } else {
            match empty.checked_add(step as usize) {
                Some(v) => v,
                None => empty,
            }
        }
    }

    pub fn slide_safe(&mut self, dir: Direction) -> Result<bool, &'static str> {
        let pos = self.safe_pos(dir.value());
        if pos == self.empty || pos >= self.tiles.len() {
            return Err("Invalid move");
        }
        if (dir == Direction::Left || dir == Direction::Right)
            && ((pos / SIZE) != (self.empty / SIZE))
        {
            return Err("Invalid move");
        }
        self.tiles.swap(self.empty, pos);
        self.empty = pos;
        Ok(true)
    }

    pub fn slide(&mut self, dir: Direction) {
        let pos = self.safe_pos(dir.value());
        self.tiles.swap(self.empty, pos);
        self.empty = pos;
    }

    pub fn can_slide(&self, dir: Direction) -> bool {
        let pos = self.safe_pos(dir.value());
        pos != self.empty
            && pos < self.tiles.len()
            && (dir == Direction::Up
                || dir == Direction::Down
                || (pos / SIZE) == (self.empty / SIZE))
    }

    pub fn shuffle(&mut self) {
        self.tiles.shuffle(&mut thread_rng());
        // Since we know the board is valid, it must contain the empty tile (0)
        self.empty = self.tiles.iter().position(|&x| x == 0).expect("no empty tile?!");
    }

    pub fn solved(&self) -> bool {
        self.empty == self.tiles.len() - 1 && self.tiles.last() == Some(&0u8) &&
            self.tiles.windows(2).all(|win| win[1] == 0 || win[0] < win[1])
    }

    pub fn solvable(&self) -> bool {
        let invs: usize = self
            .tiles
            .iter()
            .enumerate()
            .map(|(i, &t)| self.tiles.iter().skip(i).filter(|&&x| x != 0 && x < t).count())
            .sum();
        // (empty tile in even row, even # of inversions)
        match ((self.empty / SIZE) % 2 == 0, invs % 2 == 0) {
            (true, false) | (false, true) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..4 {
            write!(f, "[")?;
            for j in 0..4 {
                write!(f, "{}", self.tiles[i * 4 + j])?;
                if j != 3 {
                    write!(f, " ")?;
                }
            }
            write!(f, "]")?;
            if i != 3 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const DEFAULT_CONFIG: &'static [u8; SIZE * SIZE] =
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    const SOLVED_CONFIG: &'static [u8; SIZE * SIZE] =
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
    const SOLVABLE_CONFIG: &'static [u8; SIZE * SIZE] =
        &[1, 2, 3, 4, 0, 5, 6, 7, 8, 10, 11, 9, 12, 13, 14, 15];

    #[test]
    fn dir_is_opposite() {
        assert_eq!(Direction::Up, Direction::Down.opposite());
        assert_eq!(Direction::Down, Direction::Up.opposite());
        assert_eq!(Direction::Left, Direction::Right.opposite());
        assert_eq!(Direction::Right, Direction::Left.opposite());

        assert_ne!(Direction::Up, Direction::Up.opposite());
        assert_ne!(Direction::Up, Direction::Left.opposite());
        assert_ne!(Direction::Up, Direction::Right.opposite());
    }

    #[test]
    fn dir_opposes() {
        assert!(Direction::Up.opposites(Direction::Down));
        assert!(Direction::Down.opposites(Direction::Up));
        assert!(Direction::Left.opposites(Direction::Right));
        assert!(Direction::Right.opposites(Direction::Left));

        assert!(!Direction::Up.opposites(Direction::Up));
        assert!(!Direction::Up.opposites(Direction::Left));
        assert!(!Direction::Up.opposites(Direction::Right));
    }

    #[test]
    fn dir_value() {
        assert_eq!(Direction::Up.value(), -4);
        assert_eq!(Direction::Down.value(), 4);
        assert_eq!(Direction::Left.value(), -1);
        assert_eq!(Direction::Right.value(), 1);
    }

    fn is_board_valid(b: &Board) -> bool {
        let mut tiles = HashSet::new();
        for tile in &b.tiles {
            tiles.insert(tile);
        }
        if tiles.len() != b.tiles.len() {
            return false;
        }
        for tile in &tiles {
            let value = **tile as usize;
            if value >= b.tiles.len() {
                return false;
            }
        }
        true
    }

    #[test]
    fn create_board() {
        let b = Board::new();
        assert_eq!(b.tiles, *DEFAULT_CONFIG);
        assert_eq!(b.empty, 0);

        let b = Board::new_from(DEFAULT_CONFIG).expect("failed to create default board");
        assert_eq!(b.tiles, *DEFAULT_CONFIG);
        assert_eq!(b.empty, 0);

        let b = Board::new_from(SOLVED_CONFIG).expect("failed to create solved board");
        assert_eq!(b.tiles, *SOLVED_CONFIG);
        assert_eq!(b.empty, 15);
    }

    #[test]
    fn board_move() {
        let mut b = Board::new();

        // Move all the way to the right
        for i in 0..3 {
            assert!(b.can_slide(Direction::Right));
            b.slide(Direction::Right);
            assert!(is_board_valid(&b));
            assert_eq!(b.empty, i + 1);
        }
        // Try to go over the right edge
        assert!(!b.can_slide(Direction::Right));
        let res = b.slide_safe(Direction::Right);
        assert!(res.is_err());
        assert!(is_board_valid(&b));
        assert_eq!(b.empty, 3);

        // Move all the way down
        for i in 0..3 {
            assert!(b.can_slide(Direction::Down));
            b.slide(Direction::Down);
            assert!(is_board_valid(&b));
            assert_eq!(b.empty, (i + 1) * SIZE + 3);
        }
        // Try to go over the bottom edge
        assert!(!b.can_slide(Direction::Down));
        let res = b.slide_safe(Direction::Down);
        assert!(res.is_err());
        assert!(is_board_valid(&b));
        assert_eq!(b.empty, 15);

        // Move all the way to the left
        for i in 0..3 {
            assert!(b.can_slide(Direction::Left));
            b.slide(Direction::Left);
            assert!(is_board_valid(&b));
            assert_eq!(b.empty, b.tiles.len() - 1 - i - 1);
        }
        // Try to go over the left edge
        assert!(!b.can_slide(Direction::Left));
        let res = b.slide_safe(Direction::Left);
        assert!(res.is_err());
        assert!(is_board_valid(&b));
        assert_eq!(b.empty, 12);

        // Move all the way up
        for i in 0..3 {
            assert!(b.can_slide(Direction::Up));
            b.slide(Direction::Up);
            assert!(is_board_valid(&b));
            assert_eq!(b.empty, (3 - i - 1) * SIZE);
        }
        // Try to go over the top edge
        assert!(!b.can_slide(Direction::Up));
        let res = b.slide_safe(Direction::Up);
        assert!(res.is_err());
        assert!(is_board_valid(&b));
        assert_eq!(b.empty, 0);
    }

    #[test]
    fn random_board() {
        let b = Board::new_random();
        assert!(is_board_valid(&b));

        let mut b = Board::new();
        assert!(is_board_valid(&b));
        b.shuffle();
        assert!(is_board_valid(&b));
    }

    #[test]
    fn board_solvable() {
        let b = Board::new_from(SOLVED_CONFIG).expect("failed to create solved board");
        assert!(b.solvable());

        let b = Board::new_from(DEFAULT_CONFIG).expect("failed to create default board");
        assert!(!b.solvable());

        let b = Board::new_from(SOLVABLE_CONFIG).expect("failed to create solvable board");
        assert!(b.solvable());
    }

    #[test]
    fn board_solved() {
        let b = Board::new_from(SOLVED_CONFIG).expect("failed to create solved board");
        assert!(b.solved());

        let b = Board::new_from(DEFAULT_CONFIG).expect("failed to create default board");
        assert!(!b.solved());

        let b = Board::new_from(SOLVABLE_CONFIG).expect("failed to create solvable board");
        assert!(!b.solved());
    }

    #[test]
    fn board_clone() {
        let b = Board::new();
        let clone1 = b.clone();
        let clone2 = Board::new_from(&b.tiles).expect("failed to create clone board");
        assert_eq!(b.tiles, clone1.tiles);
        assert_ne!(b.tiles.as_ptr(), clone1.tiles.as_ptr());
        assert_eq!(b.empty, clone1.empty);
        assert_eq!(b.tiles, clone2.tiles);
        assert_ne!(b.tiles.as_ptr(), clone2.tiles.as_ptr());
        assert_eq!(b.empty, clone2.empty);
    }
}
