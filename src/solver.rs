use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::board;
use board::*;

#[derive(Clone, Eq, PartialEq)]
struct Solution {
    cost: usize,
    moves: Vec<Direction>,
    board: Board,
}

impl Ord for Solution {
    fn cmp(&self, other: &Solution) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.moves.len().cmp(&other.moves.len()))
    }
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Solution) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

static DIRECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Right,
    Direction::Up,
    Direction::Down,
];

pub struct Astar;

impl Astar {
    // Calculate the Manhattan distance of a value `val` at index `idx`
    fn manhattan_dist(tile: u8, idx: usize) -> usize {
        let val = tile as isize;
        // Linear distance from where tile should be
        let diff = match val.checked_sub(1 + idx as isize) {
            Some(v) => v.wrapping_abs() as u32 as usize,
            None => panic!("Manhattan distance should not overflow: {}-1-{}", val, idx),
        };
        ((diff / 4) + (diff % 4)) // # of rows + cols to move
    }

    pub fn run(b: &Board) -> Option<Vec<Direction>> {
        let mut heap = BinaryHeap::new();
        let cost: usize = b
            .tiles()
            .iter()
            .enumerate()
            .map(|(i, t)| Astar::manhattan_dist(*t, i))
            .sum();
        heap.push(Solution {
            cost,
            moves: vec![],
            board: b.clone(),
        });

        while let Some(Solution {
            ref moves,
            ref board,
            ..
        }) = heap.pop()
        {
            if board.solved() {
                return Some(moves.to_vec());
            }
            let n_moves = moves.len();
            for &dir in DIRECTIONS.iter() {
                // Do not undo last move
                if let Some(last) = moves.last() {
                    if last.opposites(dir) {
                        continue;
                    }
                }
                if !board.can_slide(dir) {
                    continue;
                }
                let mut b = board.clone();
                b.slide(dir);
                let mut nc: usize = b
                    .tiles()
                    .iter()
                    .enumerate()
                    .map(|(i, t)| Astar::manhattan_dist(*t, i))
                    .sum();
                nc += n_moves + 1;
                let mut nm = moves.clone();
                nm.push(dir);
                heap.push(Solution {
                    cost: nc,
                    moves: nm,
                    board: b,
                });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOLVABLE_CONFIG: &'static [u8; 16] =
        &[1, 6, 2, 9, 7, 8, 4, 0, 13, 5, 3, 11, 15, 14, 10, 12];
    const ALMOST_CONFIG: &'static [u8; 16] =
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 15];

    #[test]
    fn solve_almost_solved() {
        let res = Board::new_from(ALMOST_CONFIG);
        assert!(res.is_ok());
        let mut board = res.expect("failed to create almost board");
        assert!(board.solvable());
        match Astar::run(&board) {
            Some(moves) => {
                for &dir in moves.iter() {
                    assert!(!board.slide_safe(dir).is_err());
                }
                assert!(board.solved());
            }
            None => panic!("result should not be None"),
        }
    }

    #[test]
    fn solve_solvable() {
        let res = Board::new_from(SOLVABLE_CONFIG);
        assert!(res.is_ok());
        let mut board = res.expect("failed to create solvable board");
        assert!(board.solvable());
        match Astar::run(&board) {
            Some(moves) => {
                for &dir in moves.iter() {
                    assert!(!board.slide_safe(dir).is_err());
                }
                assert!(board.solved());
            }
            None => panic!("result should not be None"),
        }
    }

    #[test]
    #[ignore]
    fn solve_random() {
        let mut board = Board::new_random();
        assert!(board.solvable());
        match Astar::run(&board) {
            Some(moves) => {
                for dir in moves {
                    assert!(!board.slide_safe(dir).is_err());
                }
                assert!(board.solved());
            }
            None => panic!("result should not be None"),
        }
    }
}
